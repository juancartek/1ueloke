/// Módulo de interfaz de usuario
///
/// Maneja la visualización en terminal y entrada del usuario
use colored::*;
use crate::server::ServerInfo;

/// Imprime el logo ASCII del programa
pub fn print_logo() {
    println!("{}", "   ╔══════════════════════════════════════════════════════════════════════════╗".red().bold());
    println!("   ║ {} ║", r"      __  ____   standard  ____  _   _  _____  ____   ____        ".red().bold());
    println!("   ║ {} ║", r"     / / / / /  /_  __/ __ \/ | / / /_  __/ __ \/ __ \       ".red().bold());
    println!("   ║ {} ║", r"    / / / / /    / / / /_/ /  |/ /   / / / /_/ / / / /       ".red().bold());
    println!("   ║ {} ║", r"   / /_/ / /___ / / / _, _/ /|  /   / / / _, _/ /_/ /        ".red().bold());
    println!("   ║ {} ║", r"   \____/_____//_/ /_/ |_/_/ |_/   /_/ /_/ |_|\____/         ".red().bold());
    println!("   ║                                                                          ║");
    println!("   ║  {}  {} v5.6.0 | OXIDE ENGINE          ║", "🚀".red(), "COPENHEIMER ULTRA-NITRO".white().bold());
    println!("{}", "   ╚══════════════════════════════════════════════════════════════════════════╝".red().bold());
    println!();
}

/// Imprime el menú principal
pub fn print_main_menu() {
    println!("   {} Selecciona una opción:", "🎮".red());
    println!("   {} {}  {}  -  Búsqueda ilimitada", " [1]".red(), "🚀".green(), "Quick Scan".white());
    println!("   {} {}  {}  -  Buscar N servidores y parar", " [2]".red(), "🎯".cyan(), "Target Mode".white());
    println!();
}

/// Imprime el menú de intensidades
pub fn print_intensity_menu() {
    println!("   {} Selecciona la Intensidad:", "⚡".yellow());
    println!("   {} {}  {}  -  Para routers normales (Seguro)", " [1]".green(), "🏠", "HOME".white());
    println!("   {} {}  {}  -  Para fibra óptica (Rápido)", " [2]".yellow(), "🚀", "PRO".white());
    println!("   {} {}  {}  -  Para VPS/Dedicados (Extremo)", " [3]".red(), "🔥", "NITRO".white());
    println!();
}

/// Muestra información de un servidor encontrado
///
/// # Argumentos
///
/// * `info` - Información del servidor
/// * `provider` - Proveedor de hosting
pub fn display_server(info: &ServerInfo, provider: &str) {
    println!("{}", "   ╭──────────────────────────────────────────────────────────────────────────╮".red());
    println!(
        "   🔥  IP: {:<15} 🏢  HOST: {:<20}",
        info.ip.cyan(),
        provider.yellow()
    );
    println!(
        "   🛠️   VER: {:<15} 👥  PLAYERS: {}/{}",
        info.version.green(),
        info.online_players.to_string().white(),
        info.max_players.to_string().white()
    );
    println!("   📝  MOTD: {:<50}", info.motd.white());
    println!("{}", "   ╰──────────────────────────────────────────────────────────────────────────╯".red());
}

/// Muestra el progreso del escaneo
///
/// # Argumentos
///
/// * `pps` - Paquetes por segundo
/// * `scanned` - Total de IPs escaneadas
/// * `found` - Total de servidores encontrados
pub fn display_progress(pps: u64, scanned: u64, found: u64) {
    print!(
        "\r  {} PPS: {} | Scanned: {} | Found: {} ",
        "🚀".bright_green(),
        format_count(pps),
        format_count(scanned),
        found
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

/// Formatea un número grande en formato legible (K, M, B, T)
///
/// # Argumentos
///
/// * `count` - Número a formatear
pub fn format_count(count: u64) -> String {
    if count >= 1_000_000_000_000 {
        format!("{:.2}T", count as f64 / 1_000_000_000_000.0)
    } else if count >= 1_000_000_000 {
        format!("{:.2}B", count as f64 / 1_000_000_000.0)
    } else if count >= 1_000_000 {
        format!("{:.2}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.2}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

/// Muestra mensaje de inicio del escaneo
pub fn show_scan_start(workers: usize) {
    println!("\n   {} {}", "🔥".red(), format!("Iniciando motor con {} workers...", workers).yellow());
}

/// Muestra mensaje de objetivo alcanzado
///
/// # Argumentos
///
/// * `duration` - Duración del escaneo
/// * `scanned` - Total de IPs verificadas
pub fn show_scan_complete(duration: std::time::Duration, scanned: u64) {
    println!("\n   {} Objetivo alcanzado.", "🏁".green());
    println!("   {} Tiempo total: {:?}.", "⏱️".cyan(), duration);
    println!("   {} IPs verificadas: {}.", "🔍".magenta(), format_count(scanned));
}

/// Lee una línea de entrada del usuario
///
/// # Argumentos
///
/// * `prompt` - Texto a mostrar como prompt
pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error leyendo entrada");
    input.trim().to_string()
}

/// Pausa hasta que el usuario presione Enter
pub fn pause() {
    println!("\n   {} Presiona [ENTER] para continuar...", "👉".yellow());
    let _ = std::io::stdin().read_line(&mut String::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(500), "500");
        assert_eq!(format_count(1_500), "1.50K");
        assert_eq!(format_count(1_500_000), "1.50M");
        assert_eq!(format_count(1_500_000_000), "1.50B");
        assert_eq!(format_count(1_500_000_000_000), "1.50T");
    }
}
