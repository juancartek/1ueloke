/// COPENHEIMER - Ultra-fast Minecraft Server Scanner
///
/// Escáner masivo de servidores Minecraft con soporte para hasta 400,000
/// conexiones concurrentes usando Tokio.

use copenheimer_oxide::cidr::load_cidr_ranges;
use copenheimer_oxide::config::Config;
use copenheimer_oxide::export::{ExportFormat, Exporter};
use copenheimer_oxide::scanner::{ProgressMonitor, Scanner, ScannerConfig};
use copenheimer_oxide::server::check_server;
use copenheimer_oxide::ui;
use copenheimer_oxide::updater;
#[cfg(feature = "mongodb")]
use copenheimer_oxide::database::{self, DatabaseClient};
use colored::*;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Manejar argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();

    // Modo de verificación de actualizaciones
    if args.len() > 1 && args[1] == "--check-updates" {
        handle_check_updates().await;
        return;
    }

    // Modo de actualización automática
    if args.len() > 1 && args[1] == "--update" {
        handle_update().await;
        return;
    }

    // Modo de verificación de servidor único
    if args.len() > 2 && args[1] == "--check" {
        if let Ok(ip) = args[2].parse() {
            handle_check_mode(ip).await;
        } else {
            eprintln!("❌ IP inválida: {}", args[2]);
        }
        return;
    }

    // Loop principal del menú
    loop {
        ui::print_logo();

        // Verificar actualizaciones en segundo plano (solo la primera vez)
        static UPDATE_CHECKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !UPDATE_CHECKED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            // Verificar de forma sincrónica en el primer inicio
            if let Ok(info) = tokio::runtime::Handle::current().block_on(updater::check_for_updates()) {
                if info.update_available {
                    updater::display_update_info(&info);
                    
                    if updater::prompt_update() {
                        println!();
                        match tokio::runtime::Handle::current().block_on(updater::run_updater()) {
                            Ok(_) => {
                                println!();
                                println!("{}", "═══════════════════════════════════════════════════".green());
                                println!("{}", "   ✅ Actualización completada exitosamente".green().bold());
                                println!("{}", "   🔄 Reiniciando el programa...".cyan());
                                println!("{}", "═══════════════════════════════════════════════════".green());
                                std::process::exit(0);
                            }
                            Err(e) => {
                                eprintln!();
                                eprintln!("{}", "═══════════════════════════════════════════════════".red());
                                eprintln!("{} {}", "   ❌ Error durante la actualización:".red(), e);
                                eprintln!("{}", "   💡 Intenta más tarde o usa: ./copenheimer_oxide --update".yellow());
                                eprintln!("{}", "═══════════════════════════════════════════════════".red());
                                println!();
                                ui::pause();
                            }
                        }
                    } else {
                        println!("{}", "   ⏭️  Omitiendo actualización por ahora...".yellow());
                        println!();
                        sleep(Duration::from_millis(1500)).await;
                    }
                }
            }
        }

        // Cargar configuración con fallback a default hardcoded
        let config = Config::default().unwrap_or_else(|_| create_default_config());

        // Mostrar menú principal
        ui::print_main_menu();
        let choice = ui::read_input("   👉 Opción > ");

        // Determinar modo de escaneo
        let target_count: Option<u64> = match choice.as_str() {
            "1" => None, // Quick Scan - ilimitado
            "2" => {
                let input = ui::read_input("   🔢 Cuántos servidores quieres encontrar? > ");
                input.parse().ok()
            }
            _ => {
                println!("   ⚠️  Opción no válida. Iniciando Quick Scan por defecto...");
                None
            }
        };

        // Seleccionar intensidad
        ui::print_intensity_menu();
        let intensity_input = ui::read_input("   👉 Intensidad > ");
        let workers = config.get_workers_for_intensity(&intensity_input);

        ui::show_scan_start(workers);

        // Cargar rangos CIDR
        let cidrs = match load_cidr_ranges(&config.cidr_ranges.ranges) {
            Ok(cidrs) => cidrs,
            Err(e) => {
                eprintln!("❌ Error cargando rangos CIDR: {}", e);
                ui::pause();
                continue;
            }
        };

        // Crear configuración del escáner
        let scanner_config = ScannerConfig {
            workers,
            timeout_ms: config.scan.timeout_ms,
            read_timeout_ms: config.scan.read_timeout_ms,
            target_count,
        };

        // Crear escáner
        let scanner = Arc::new(Scanner::new(scanner_config, cidrs));
        let mut rx = scanner.start().await;

        // Crear exportador
        let export_format = ExportFormat::from_str(&config.output.format);
        let mut exporter = Exporter::new(export_format, config.output.file.clone());

        // Inicializar cliente de base de datos si está configurado
        #[cfg(feature = "mongodb")]
        let db_client = {
            if let Some(db_config) = database::load_database_config() {
                match DatabaseClient::new(db_config).await {
                    Ok(client) => {
                        if client.is_enabled() {
                            if let Ok(stats) = client.get_stats().await {
                                println!("   📊 Base de datos: {} servidores almacenados", stats.total_servers);
                            }
                        }
                        Some(Arc::new(client))
                    }
                    Err(e) => {
                        eprintln!("   ⚠️  Error conectando a BD: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        };
        #[cfg(not(feature = "mongodb"))]
        let _db_client: Option<Arc<()>> = None;

        // Crear monitor de progreso
        let mut monitor = ProgressMonitor::new(scanner.scanned.clone(), scanner.found.clone());

        // Tarea de monitoreo de progreso
        let scanner_clone = scanner.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(500)).await;
                let (pps, scanned, found, _) = monitor.get_stats();
                ui::display_progress(pps, scanned, found);
                
                if !scanner_clone.is_scanning() {
                    break;
                }
            }
        });

        // Procesar resultados
        let start_time = std::time::Instant::now();
        while let Some(result) = rx.recv().await {
            let _current_found = scanner.increment_found();

            // Mostrar en consola
            ui::display_server(&result.info, &result.provider);

            // Exportar resultado
            if let Err(e) = exporter.export(&result.info, &result.provider) {
                eprintln!("   ⚠️  Error exportando: {}", e);
            }

            // Guardar en base de datos si está habilitada
            #[cfg(feature = "mongodb")]
            if let Some(ref client) = db_client {
                if let Err(e) = client.save_server(&result.info, &result.provider).await {
                    eprintln!("   ⚠️  Error guardando en BD: {}", e);
                }
            }

            // Verificar si se alcanzó el objetivo
            if scanner.target_reached() {
                scanner.stop();
                let duration = start_time.elapsed();
                let scanned = scanner.scanned_count();
                ui::show_scan_complete(duration, scanned);
                
                // Mostrar stats finales de BD
                #[cfg(feature = "mongodb")]
                if let Some(ref client) = db_client {
                    if let Ok(stats) = client.get_stats().await {
                        println!("   📊 Total en BD: {} servidores", stats.total_servers);
                    }
                }
                
                ui::pause();
                break;
            }
        }

        // Si el receiver se cerró, continuar al menú
    }
}

/// Maneja el modo de verificación de un solo servidor
async fn handle_check_mode(ip: std::net::Ipv4Addr) {
    if let Some(info) = check_server(ip, 300, 400).await {
        println!("{}", "   ╭──────────────────────────────────────────────────────────────────────────╮".red());
        println!(
            "   🔥  IP: {:<15} 🏢  HOST: {:<20}",
            info.ip.cyan(),
            info.domain.yellow()
        );
        println!(
            "   🛠️   VER: {:<15} 👥  PLAYERS: {}/{}",
            info.version.green(),
            info.online_players.to_string().white(),
            info.max_players.to_string().white()
        );
        println!("   📝  MOTD: {:<50}", info.motd.white());
        println!("{}", "   ╰──────────────────────────────────────────────────────────────────────────╯".red());
    } else {
        println!("{}", "OFFLINE".red());
    }
}

/// Maneja la verificación de actualizaciones
async fn handle_check_updates() {
    match updater::check_for_updates().await {
        Ok(info) => {
            updater::display_update_info(&info);
            if !info.update_available {
                println!("{}", "✅ Estás usando la última versión".green());
            }
        }
        Err(e) => {
            eprintln!("{} {}", "❌ Error verificando actualizaciones:".red(), e);
        }
    }
}

/// Maneja la actualización automática
async fn handle_update() {
    match updater::run_updater().await {
        Ok(_) => {
            println!();
            println!("{}", "═══════════════════════════════════════════════════".green());
            println!("{}", "   ✅ Actualización completada exitosamente".green().bold());
            println!("{}", "═══════════════════════════════════════════════════".green());
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!();
            eprintln!("{}", "═══════════════════════════════════════════════════".red());
            eprintln!("{} {}", "   ❌ Error durante la actualización:".red(), e);
            eprintln!("{}", "═══════════════════════════════════════════════════".red());
            std::process::exit(1);
        }
    }
}

/// Crea una configuración por defecto en caso de error
fn create_default_config() -> Config {
    use copenheimer_oxide::config::*;

    Config {
        scan: ScanConfig {
            timeout_ms: 300,
            read_timeout_ms: 400,
            port: 25565,
            workers: 65000,
        },
        output: OutputConfig {
            format: "txt".to_string(),
            file: "hits_omega.txt".to_string(),
            timestamp_filename: false,
        },
        filters: FilterConfig {
            min_players: 0,
            country: String::new(),
        },
        cidr_ranges: CidrRangesConfig {
            ranges: vec![
                "51.0.0.0/8:OVH/Gaming:EU".to_string(),
                "144.76.0.0/16:Hetzner DE:DE".to_string(),
                "51.254.0.0/15:OVH Game:FR".to_string(),
                "181.0.0.0/8:Residencial LATAM:LATAM".to_string(),
                "190.0.0.0/8:Residencial LATAM:LATAM".to_string(),
            ],
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
    }
}

