/// Biblioteca COPENHEIMER
///
/// Escáner masivo de servidores de Minecraft escrito en Rust
/// con soporte para alto nivel de concurrencia.
///
/// # Ejemplo
///
/// ```no_run
/// use copenheimer_oxide::cidr::CIDR;
/// use copenheimer_oxide::server::check_server;
///
/// #[tokio::main]
/// async fn main() {
///     let cidr = CIDR::new("51.210.0.0", 20, "FR", "OVH");
///     let ip = cidr.random_ip();
///     
///     if let Some(info) = check_server(ip, 300, 400).await {
///         println!("Servidor encontrado: {}", info.ip);
///     }
/// }
/// ```

pub mod cidr;
pub mod config;
pub mod export;
pub mod scanner;
pub mod server;
pub mod ui;
pub mod updater;
pub mod database;
