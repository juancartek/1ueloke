/// Módulo de configuración
/// 
/// Maneja la carga y validación de configuración desde archivos TOML
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuración principal de la aplicación
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub scan: ScanConfig,
    pub output: OutputConfig,
    pub filters: FilterConfig,
    pub cidr_ranges: CidrRangesConfig,
    pub intensity: IntensityConfig,
    pub logging: LoggingConfig,
}

/// Configuración de escaneo
#[derive(Debug, Deserialize, Clone)]
pub struct ScanConfig {
    pub timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub port: u16,
    pub workers: usize,
}

/// Configuración de salida
#[derive(Debug, Deserialize, Clone)]
pub struct OutputConfig {
    pub format: String,
    pub file: String,
    pub timestamp_filename: bool,
}

/// Configuración de filtros
#[derive(Debug, Deserialize, Clone)]
pub struct FilterConfig {
    pub min_players: i64,
    pub country: String,
}

/// Configuración de rangos CIDR
#[derive(Debug, Deserialize, Clone)]
pub struct CidrRangesConfig {
    pub ranges: Vec<String>,
}

/// Configuración de intensidades
#[derive(Debug, Deserialize, Clone)]
pub struct IntensityConfig {
    pub home: usize,
    pub pro: usize,
    pub nitro: usize,
}

/// Configuración de logging
#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub show_pps: bool,
    pub pps_update_interval: u64,
}

impl Config {
    /// Carga la configuración desde un archivo TOML
    ///
    /// # Argumentos
    ///
    /// * `path` - Ruta al archivo de configuración
    ///
    /// # Errores
    ///
    /// Retorna error si el archivo no existe o no es válido
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Carga la configuración por defecto
    pub fn default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_file("config/default.toml")
    }

    /// Obtiene el número de workers según la intensidad
    pub fn get_workers_for_intensity(&self, intensity: &str) -> usize {
        match intensity {
            "home" | "1" => self.intensity.home,
            "pro" | "2" => self.intensity.pro,
            "nitro" | "3" => self.intensity.nitro,
            _ => self.scan.workers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workers_for_intensity() {
        let config = Config {
            scan: ScanConfig {
                timeout_ms: 300,
                read_timeout_ms: 400,
                port: 25565,
                workers: 10000,
            },
            output: OutputConfig {
                format: "txt".to_string(),
                file: "test.txt".to_string(),
                timestamp_filename: false,
            },
            filters: FilterConfig {
                min_players: 0,
                country: String::new(),
            },
            cidr_ranges: CidrRangesConfig {
                ranges: vec![],
            },
            intensity: IntensityConfig {
                home: 5000,
                pro: 25000,
                nitro: 400000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                show_pps: true,
                pps_update_interval: 500,
            },
        };

        assert_eq!(config.get_workers_for_intensity("home"), 5000);
        assert_eq!(config.get_workers_for_intensity("pro"), 25000);
        assert_eq!(config.get_workers_for_intensity("nitro"), 400000);
        assert_eq!(config.get_workers_for_intensity("unknown"), 10000);
    }
}
