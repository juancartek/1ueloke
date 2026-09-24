/// Módulo de integración con bases de datos
///
/// Soporta almacenamiento de resultados en múltiples proveedores:
/// - MongoDB Atlas
/// - PostgreSQL
/// - MySQL
/// - SQLite
use crate::server::ServerInfo;
use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Configuración de base de datos
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub enabled: bool,
    pub provider: DatabaseProvider,
    pub connection_string: String,
    pub database_name: String,
    pub collection_name: String,
}

/// Proveedores de base de datos soportados
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseProvider {
    MongoDB,
    PostgreSQL,
    MySQL,
    SQLite,
}

/// Documento de servidor para almacenar en BD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDocument {
    pub ip: String,
    pub domain: String,
    pub version: String,
    pub online_players: i64,
    pub max_players: i64,
    pub motd: String,
    pub protocol: i64,
    pub provider: String,
    pub discovered_at: DateTime<Utc>,
}

impl From<(&ServerInfo, &str)> for ServerDocument {
    fn from((info, provider): (&ServerInfo, &str)) -> Self {
        Self {
            ip: info.ip.clone(),
            domain: info.domain.clone(),
            version: info.version.clone(),
            online_players: info.online_players,
            max_players: info.max_players,
            motd: info.motd.clone(),
            protocol: info.protocol,
            provider: provider.to_string(),
            discovered_at: Utc::now(),
        }
    }
}

/// Cliente de base de datos genérico
pub struct DatabaseClient {
    config: DatabaseConfig,
    #[cfg(feature = "mongodb")]
    mongo_client: Option<mongodb::Client>,
}

impl DatabaseClient {
    /// Crea un nuevo cliente de base de datos
    pub async fn new(config: DatabaseConfig) -> Result<Self, Box<dyn Error>> {
        if !config.enabled {
            return Ok(Self {
                config,
                #[cfg(feature = "mongodb")]
                mongo_client: None,
            });
        }

        println!("{}", "📊 Conectando a base de datos...".cyan());

        match config.provider {
            DatabaseProvider::MongoDB => {
                #[cfg(feature = "mongodb")]
                {
                    let client = mongodb::Client::with_uri_str(&config.connection_string).await?;
                    
                    // Verificar conexión
                    client
                        .database(&config.database_name)
                        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
                        .await?;
                    
                    println!("{}", "✅ Conectado a MongoDB Atlas".green());
                    
                    Ok(Self {
                        config,
                        mongo_client: Some(client),
                    })
                }
                #[cfg(not(feature = "mongodb"))]
                {
                    Err("MongoDB no está habilitado. Compila con --features mongodb".into())
                }
            }
            DatabaseProvider::PostgreSQL => {
                println!("{}", "⚠️  PostgreSQL soporte en desarrollo".yellow());
                Err("PostgreSQL no implementado aún".into())
            }
            DatabaseProvider::MySQL => {
                println!("{}", "⚠️  MySQL soporte en desarrollo".yellow());
                Err("MySQL no implementado aún".into())
            }
            DatabaseProvider::SQLite => {
                println!("{}", "⚠️  SQLite soporte en desarrollo".yellow());
                Err("SQLite no implementado aún".into())
            }
        }
    }

    /// Guarda un servidor encontrado en la base de datos
    pub async fn save_server(
        &self,
        _info: &ServerInfo,
        _provider: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !self.config.enabled {
            return Ok(());
        }

        match self.config.provider {
            DatabaseProvider::MongoDB => {
                #[cfg(feature = "mongodb")]
                {
                    if let Some(client) = &self.mongo_client {
                        let db = client.database(&self.config.database_name);
                        let collection = db.collection::<ServerDocument>(&self.config.collection_name);
                        
                        let doc = ServerDocument::from((_info, _provider));
                        
                        collection.insert_one(doc, None).await?;
                    }
                    Ok(())
                }
                #[cfg(not(feature = "mongodb"))]
                {
                    Err("MongoDB no habilitado".into())
                }
            }
            _ => Err("Proveedor no implementado".into()),
        }
    }

    /// Verifica si un servidor ya existe en la BD
    pub async fn server_exists(&self, ip: &str) -> Result<bool, Box<dyn Error>> {
        if !self.config.enabled {
            return Ok(false);
        }

        match self.config.provider {
            DatabaseProvider::MongoDB => {
                #[cfg(feature = "mongodb")]
                {
                    if let Some(client) = &self.mongo_client {
                        let db = client.database(&self.config.database_name);
                        let collection = db.collection::<ServerDocument>(&self.config.collection_name);
                        
                        let filter = mongodb::bson::doc! { "ip": ip };
                        let count = collection.count_documents(filter, None).await?;
                        
                        return Ok(count > 0);
                    }
                    Ok(false)
                }
                #[cfg(not(feature = "mongodb"))]
                {
                    let _ = ip; // Evitar warning
                    Err("MongoDB no habilitado".into())
                }
            }
            _ => Err("Proveedor no implementado".into()),
        }
    }

    /// Obtiene estadísticas de la base de datos
    pub async fn get_stats(&self) -> Result<DatabaseStats, Box<dyn Error>> {
        if !self.config.enabled {
            return Ok(DatabaseStats::default());
        }

        match self.config.provider {
            DatabaseProvider::MongoDB => {
                #[cfg(feature = "mongodb")]
                {
                    if let Some(client) = &self.mongo_client {
                        let db = client.database(&self.config.database_name);
                        let collection = db.collection::<ServerDocument>(&self.config.collection_name);
                        
                        let total = collection.count_documents(None, None).await?;
                        
                        return Ok(DatabaseStats {
                            total_servers: total as u64,
                            provider: self.config.provider.clone(),
                        });
                    }
                    Ok(DatabaseStats::default())
                }
                #[cfg(not(feature = "mongodb"))]
                {
                    Err("MongoDB no habilitado".into())
                }
            }
            _ => Err("Proveedor no implementado".into()),
        }
    }

    /// Verifica si el cliente está habilitado
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

/// Estadísticas de la base de datos
#[derive(Debug, Default)]
pub struct DatabaseStats {
    pub total_servers: u64,
    pub provider: DatabaseProvider,
}

impl Default for DatabaseProvider {
    fn default() -> Self {
        DatabaseProvider::MongoDB
    }
}

/// Carga la configuración de base de datos desde config
pub fn load_database_config() -> Option<DatabaseConfig> {
    // Intentar cargar desde archivo de configuración
    if let Ok(content) = std::fs::read_to_string("config/database.toml") {
        if let Ok(config) = toml::from_str::<DatabaseConfig>(&content) {
            return Some(config);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_document_creation() {
        let info = ServerInfo {
            ip: "1.2.3.4".to_string(),
            domain: "test.com".to_string(),
            version: "1.20.1".to_string(),
            online_players: 10,
            max_players: 100,
            motd: "Test Server".to_string(),
            protocol: 47,
        };

        let doc = ServerDocument::from((&info, "TestProvider"));
        assert_eq!(doc.ip, "1.2.3.4");
        assert_eq!(doc.provider, "TestProvider");
    }

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig {
            enabled: false,
            provider: DatabaseProvider::MongoDB,
            connection_string: String::new(),
            database_name: String::new(),
            collection_name: String::new(),
        };
        
        assert!(!config.enabled);
    }
}
