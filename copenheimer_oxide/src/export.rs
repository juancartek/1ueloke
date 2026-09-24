/// Módulo de exportación de datos
///
/// Maneja la exportación de resultados a diferentes formatos (TXT, JSON, CSV)
use crate::server::ServerInfo;
use serde_json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Tipos de formato de exportación disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Txt,
    Json,
    Csv,
}

impl ExportFormat {
    /// Convierte un string a ExportFormat
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => ExportFormat::Json,
            "csv" => ExportFormat::Csv,
            _ => ExportFormat::Txt,
        }
    }

    /// Obtiene la extensión de archivo apropiada
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::Txt => "txt",
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
        }
    }
}

/// Exportador de resultados
pub struct Exporter {
    format: ExportFormat,
    file_path: String,
    csv_header_written: bool,
}

impl Exporter {
    /// Crea un nuevo exportador
    ///
    /// # Argumentos
    ///
    /// * `format` - Formato de exportación
    /// * `file_path` - Ruta del archivo de salida
    pub fn new(format: ExportFormat, file_path: String) -> Self {
        Self {
            format,
            file_path,
            csv_header_written: false,
        }
    }

    /// Exporta información de un servidor
    ///
    /// # Argumentos
    ///
    /// * `info` - Información del servidor
    /// * `provider` - Proveedor de hosting
    pub fn export(&mut self, info: &ServerInfo, provider: &str) -> Result<(), std::io::Error> {
        let content = match self.format {
            ExportFormat::Txt => self.format_txt(info, provider),
            ExportFormat::Json => self.format_json(info, provider),
            ExportFormat::Csv => {
                let mut result = String::new();
                if !self.csv_header_written {
                    result.push_str("IP,Domain,Provider,Version,Online,Max,MOTD\n");
                    self.csv_header_written = true;
                }
                result.push_str(&self.format_csv(info, provider));
                result
            }
        };

        self.append_to_file(&content)
    }

    /// Formatea como TXT
    fn format_txt(&self, info: &ServerInfo, provider: &str) -> String {
        format!(
            "🔥 IP: {} | DOMAIN: {} | HOST: {} | VER: {} | PLAYERS: {}/{} | MOTD: {}\n",
            info.ip, info.domain, provider, info.version, info.online_players, info.max_players, info.motd
        )
    }

    /// Formatea como JSON (una línea por servidor)
    fn format_json(&self, info: &ServerInfo, provider: &str) -> String {
        let json_obj = serde_json::json!({
            "ip": info.ip,
            "domain": info.domain,
            "provider": provider,
            "version": info.version,
            "players": {
                "online": info.online_players,
                "max": info.max_players
            },
            "motd": info.motd,
            "protocol": info.protocol
        });

        format!("{}\n", serde_json::to_string(&json_obj).unwrap())
    }

    /// Formatea como CSV
    fn format_csv(&self, info: &ServerInfo, provider: &str) -> String {
        format!(
            "{},{},{},{},{},{},{}\n",
            info.ip,
            info.domain,
            provider,
            info.version,
            info.online_players,
            info.max_players,
            info.motd.replace(',', ";").replace('\n', " ")
        )
    }

    /// Agrega contenido al archivo
    fn append_to_file(&self, content: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        
        file.write_all(content.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Obtiene la ruta del archivo
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

/// Crea un nombre de archivo con timestamp
///
/// # Argumentos
///
/// * `base_name` - Nombre base del archivo (sin extensión)
/// * `format` - Formato de exportación
pub fn create_timestamped_filename(base_name: &str, format: ExportFormat) -> String {
    use chrono::Local;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}.{}", base_name, timestamp, format.extension())
}

/// Agrega timestamp a un archivo existente
///
/// # Argumentos
///
/// * `filename` - Nombre del archivo
pub fn add_timestamp_to_filename(filename: &str) -> String {
    use chrono::Local;
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    
    format!("{}_{}.{}", stem, timestamp, ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_str() {
        assert_eq!(ExportFormat::from_str("txt"), ExportFormat::Txt);
        assert_eq!(ExportFormat::from_str("json"), ExportFormat::Json);
        assert_eq!(ExportFormat::from_str("csv"), ExportFormat::Csv);
        assert_eq!(ExportFormat::from_str("unknown"), ExportFormat::Txt);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Txt.extension(), "txt");
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
    }

    #[test]
    fn test_format_txt() {
        let exporter = Exporter::new(ExportFormat::Txt, "test.txt".to_string());
        let info = ServerInfo {
            ip: "1.2.3.4".to_string(),
            domain: "test.com".to_string(),
            version: "1.20.1".to_string(),
            online_players: 10,
            max_players: 100,
            motd: "Test Server".to_string(),
            protocol: 47,
        };
        
        let result = exporter.format_txt(&info, "TestProvider");
        assert!(result.contains("1.2.3.4"));
        assert!(result.contains("TestProvider"));
    }

    #[test]
    fn test_format_csv() {
        let exporter = Exporter::new(ExportFormat::Csv, "test.csv".to_string());
        let info = ServerInfo {
            ip: "1.2.3.4".to_string(),
            domain: "test.com".to_string(),
            version: "1.20.1".to_string(),
            online_players: 10,
            max_players: 100,
            motd: "Test Server".to_string(),
            protocol: 47,
        };
        
        let result = exporter.format_csv(&info, "TestProvider");
        assert!(result.contains("1.2.3.4,test.com,TestProvider"));
    }
}
