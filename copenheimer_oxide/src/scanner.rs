/// Módulo de escaneo masivo
///
/// Implementa la lógica principal de escaneo concurrente de servidores Minecraft
use crate::cidr::CIDR;
use crate::server::{check_server, ServerInfo};
use futures::stream::{FuturesUnordered, StreamExt};
use rand::Rng;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Resultado de un escaneo
pub struct ScanResult {
    pub info: ServerInfo,
    pub provider: String,
}

/// Configuración del escáner
pub struct ScannerConfig {
    pub workers: usize,
    pub timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub target_count: Option<u64>,
}

/// Escáner de servidores Minecraft
pub struct Scanner {
    config: ScannerConfig,
    cidrs: Arc<Vec<CIDR>>,
    pub scanned: Arc<AtomicU64>,
    pub found: Arc<AtomicU64>,
    pub scanning: Arc<AtomicBool>,
}

impl Scanner {
    /// Crea un nuevo escáner
    ///
    /// # Argumentos
    ///
    /// * `config` - Configuración del escáner
    /// * `cidrs` - Rangos CIDR a escanear
    pub fn new(config: ScannerConfig, cidrs: Vec<CIDR>) -> Self {
        Self {
            config,
            cidrs: Arc::new(cidrs),
            scanned: Arc::new(AtomicU64::new(0)),
            found: Arc::new(AtomicU64::new(0)),
            scanning: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Inicia el escaneo
    ///
    /// # Retorna
    ///
    /// Un receiver de mpsc::channel con los resultados encontrados
    pub async fn start(&self) -> tokio::sync::mpsc::Receiver<ScanResult> {
        let (tx, rx) = tokio::sync::mpsc::channel(10000);

        let cidrs = self.cidrs.clone();
        let scanned = self.scanned.clone();
        let scanning = self.scanning.clone();
        let timeout_ms = self.config.timeout_ms;
        let read_timeout_ms = self.config.read_timeout_ms;
        let workers = self.config.workers;

        // Spawn main scanning task
        tokio::spawn(async move {
            let mut futures = FuturesUnordered::new();

            // Seed initial batch of workers
            for _ in 0..workers {
                if !scanning.load(Ordering::Relaxed) {
                    break;
                }

                let cidrs_clone = cidrs.clone();
                let scanned_clone = scanned.clone();
                
                futures.push(tokio::spawn(async move {
                    // Seleccionar un CIDR aleatorio y generar IP
                    let (ip, provider) = {
                        let mut rng = rand::thread_rng();
                        let idx = rng.gen_range(0..cidrs_clone.len());
                        let cidr = &cidrs_clone[idx];
                        let ip = cidr.random_ip();
                        (ip, cidr.provider.clone())
                    };

                    // Verificar servidor
                    let result = check_server(ip, timeout_ms, read_timeout_ms).await;
                    
                    scanned_clone.fetch_add(1, Ordering::Relaxed);
                    
                    result.map(|info| (info, provider))
                }));
            }

            // Process results and maintain worker pool
            while let Some(result) = futures.next().await {
                if !scanning.load(Ordering::Relaxed) {
                    break;
                }

                // Handle the completed task
                if let Ok(Some((info, provider))) = result {
                    let scan_result = ScanResult { info, provider };
                    if tx.send(scan_result).await.is_err() {
                        break;
                    }
                }

                // Add new task to maintain pool size
                if scanning.load(Ordering::Relaxed) {
                    let cidrs_clone = cidrs.clone();
                    let scanned_clone = scanned.clone();
                    
                    futures.push(tokio::spawn(async move {
                        let (ip, provider) = {
                            let mut rng = rand::thread_rng();
                            let idx = rng.gen_range(0..cidrs_clone.len());
                            let cidr = &cidrs_clone[idx];
                            let ip = cidr.random_ip();
                            (ip, cidr.provider.clone())
                        };

                        let result = check_server(ip, timeout_ms, read_timeout_ms).await;
                        scanned_clone.fetch_add(1, Ordering::Relaxed);
                        
                        result.map(|info| (info, provider))
                    }));
                }
            }
        });

        rx
    }

    /// Detiene el escaneo
    pub fn stop(&self) {
        self.scanning.store(false, Ordering::Relaxed);
    }

    /// Obtiene el total de IPs escaneadas
    pub fn scanned_count(&self) -> u64 {
        self.scanned.load(Ordering::Relaxed)
    }

    /// Obtiene el total de servidores encontrados
    pub fn found_count(&self) -> u64 {
        self.found.load(Ordering::Relaxed)
    }

    /// Incrementa el contador de servidores encontrados
    pub fn increment_found(&self) -> u64 {
        self.found.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Verifica si se alcanzó el objetivo
    pub fn target_reached(&self) -> bool {
        if let Some(target) = self.config.target_count {
            self.found_count() >= target
        } else {
            false
        }
    }

    /// Obtiene si está escaneando
    pub fn is_scanning(&self) -> bool {
        self.scanning.load(Ordering::Relaxed)
    }
}

/// Monitor de progreso del escaneo
pub struct ProgressMonitor {
    scanned: Arc<AtomicU64>,
    found: Arc<AtomicU64>,
    start_time: Instant,
    last_scanned: u64,
    last_update: Instant,
}

impl ProgressMonitor {
    /// Crea un nuevo monitor
    pub fn new(scanned: Arc<AtomicU64>, found: Arc<AtomicU64>) -> Self {
        let now = Instant::now();
        Self {
            scanned,
            found,
            start_time: now,
            last_scanned: 0,
            last_update: now,
        }
    }

    /// Obtiene las estadísticas actuales
    ///
    /// # Retorna
    ///
    /// Tupla con (pps, scanned_total, found_total, elapsed_time)
    pub fn get_stats(&mut self) -> (u64, u64, u64, Duration) {
        let now = Instant::now();
        let current = self.scanned.load(Ordering::Relaxed);
        
        // Calculate PPS based on actual elapsed time
        let elapsed_since_last = now.duration_since(self.last_update).as_secs_f64();
        let pps = if elapsed_since_last > 0.0 {
            ((current - self.last_scanned) as f64 / elapsed_since_last) as u64
        } else {
            0
        };
        
        self.last_scanned = current;
        self.last_update = now;

        let found = self.found.load(Ordering::Relaxed);
        let elapsed = self.start_time.elapsed();

        (pps, current, found, elapsed)
    }

    /// Reinicia el monitor
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_update = now;
        self.last_scanned = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cidr::CIDR;

    #[test]
    fn test_scanner_creation() {
        let config = ScannerConfig {
            workers: 1000,
            timeout_ms: 300,
            read_timeout_ms: 400,
            target_count: Some(10),
        };
        
        let cidrs = vec![
            CIDR::new("192.168.0.0", 24, "US", "Test"),
        ];

        let scanner = Scanner::new(config, cidrs);
        assert_eq!(scanner.scanned_count(), 0);
        assert_eq!(scanner.found_count(), 0);
        assert!(scanner.is_scanning());
    }

    #[test]
    fn test_scanner_stop() {
        let config = ScannerConfig {
            workers: 1000,
            timeout_ms: 300,
            read_timeout_ms: 400,
            target_count: None,
        };
        
        let scanner = Scanner::new(config, vec![]);
        assert!(scanner.is_scanning());
        
        scanner.stop();
        assert!(!scanner.is_scanning());
    }

    #[test]
    fn test_target_reached() {
        let config = ScannerConfig {
            workers: 1000,
            timeout_ms: 300,
            read_timeout_ms: 400,
            target_count: Some(5),
        };
        
        let scanner = Scanner::new(config, vec![]);
        assert!(!scanner.target_reached());
        
        for _ in 0..5 {
            scanner.increment_found();
        }
        assert!(scanner.target_reached());
    }
}
