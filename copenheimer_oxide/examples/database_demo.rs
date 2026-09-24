/// Ejemplo de uso del módulo de base de datos
///
/// Este ejemplo muestra cómo conectarse y guardar servidores en MongoDB

#[cfg(feature = "mongodb")]
use copenheimer_oxide::database::{DatabaseClient, DatabaseConfig, DatabaseProvider, ServerDocument};
use copenheimer_oxide::server::ServerInfo;
use chrono::Utc;

#[tokio::main]
async fn main() {
    #[cfg(feature = "mongodb")]
    {
        println!("📊 Ejemplo de integración con MongoDB\n");

        // Configuración de ejemplo (usa variables de entorno o archivo)
        let config = DatabaseConfig {
            enabled: true,
            provider: DatabaseProvider::MongoDB,
            connection_string: std::env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database_name: "copenheimer_test".to_string(),
            collection_name: "servers_test".to_string(),
        };

        println!("🔌 Conectando a MongoDB...");
        match DatabaseClient::new(config).await {
            Ok(client) => {
                println!("✅ Conexión exitosa\n");

                // Crear un servidor de ejemplo
                let test_server = ServerInfo {
                    ip: "192.168.1.100".to_string(),
                    domain: "test.example.com".to_string(),
                    version: "1.20.1".to_string(),
                    online_players: 5,
                    max_players: 20,
                    motd: "Test Server".to_string(),
                    protocol: 761,
                };

                // Guardar en la base de datos
                println!("💾 Guardando servidor de prueba...");
                match client.save_server(&test_server, "TestProvider").await {
                    Ok(_) => println!("✅ Servidor guardado exitosamente\n"),
                    Err(e) => eprintln!("❌ Error guardando: {}\n", e),
                }

                // Obtener estadísticas
                println!("📊 Obteniendo estadísticas...");
                match client.get_stats().await {
                    Ok(stats) => {
                        println!("✅ Total de servidores en BD: {}", stats.total_servers);
                    }
                    Err(e) => eprintln!("❌ Error obteniendo stats: {}", e),
                }

                // Verificar si existe
                println!("\n🔍 Verificando si el servidor existe...");
                match client.server_exists("192.168.1.100").await {
                    Ok(exists) => {
                        if exists {
                            println!("✅ El servidor existe en la BD");
                        } else {
                            println!("❌ El servidor NO existe en la BD");
                        }
                    }
                    Err(e) => eprintln!("❌ Error verificando: {}", e),
                }
            }
            Err(e) => {
                eprintln!("❌ Error de conexión: {}", e);
                eprintln!("\n💡 Asegúrate de:");
                eprintln!("   1. Tener MongoDB corriendo (local o Atlas)");
                eprintln!("   2. Configurar MONGODB_URI correctamente");
                eprintln!("   3. Compilar con: cargo run --example database_demo --features mongodb");
            }
        }
    }

    #[cfg(not(feature = "mongodb"))]
    {
        eprintln!("❌ MongoDB no está habilitado");
        eprintln!("\n💡 Compila con soporte MongoDB:");
        eprintln!("   cargo run --example database_demo --features mongodb");
    }
}
