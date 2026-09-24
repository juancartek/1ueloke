/// Ejemplo de verificación de un servidor Minecraft específico
///
/// Uso: cargo run --example check_server <IP>
use copenheimer_oxide::server::check_server;
use std::env;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run --example check_server <IP>");
        eprintln!("Ejemplo: cargo run --example check_server 51.210.45.123");
        std::process::exit(1);
    }

    let ip_str = &args[1];
    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("❌ IP inválida: {}", ip_str);
            std::process::exit(1);
        }
    };

    println!("🔍 Verificando servidor Minecraft en {}...\n", ip);

    match check_server(ip, 300, 400).await {
        Some(info) => {
            println!("✅ Servidor encontrado!\n");
            println!("╔══════════════════════════════════════════════════════════╗");
            println!("║  📍 IP:         {:<39} ║", info.ip);
            println!("║  🌐 Domain:     {:<39} ║", info.domain);
            println!("║  🎮 Version:    {:<39} ║", info.version);
            println!("║  📊 Protocol:   {:<39} ║", info.protocol);
            println!("║  👥 Players:    {}/{:<35} ║", info.online_players, info.max_players);
            println!("╠══════════════════════════════════════════════════════════╣");
            println!("║  📝 MOTD:                                                ║");
            println!("║  {:<55} ║", info.motd);
            println!("╚══════════════════════════════════════════════════════════╝");
        }
        None => {
            println!("❌ No se pudo conectar al servidor");
            println!("   Posibles causas:");
            println!("   • El servidor está offline");
            println!("   • El puerto 25565 está cerrado");
            println!("   • El servidor no responde al protocolo estándar");
            println!("   • La IP no tiene un servidor de Minecraft");
        }
    }
}
