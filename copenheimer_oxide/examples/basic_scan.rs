/// Ejemplo básico de uso de COPENHEIMER
///
/// Este ejemplo muestra cómo usar la biblioteca para escanear un rango CIDR específico
use copenheimer_oxide::cidr::CIDR;
use copenheimer_oxide::server::check_server;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("🔥 COPENHEIMER - Ejemplo Básico\n");

    // Crear un rango CIDR pequeño para prueba
    let cidr = CIDR::new("51.210.0.0", 20, "FR", "OVH/Gaming");

    println!("📊 Información del rango:");
    println!("   Red: 51.210.0.0/20");
    println!("   Total IPs: {}", cidr.ip_count());
    println!("   Proveedor: {}", cidr.provider);
    println!("   País: {}\n", cidr.country);

    println!("🔍 Escaneando 10 IPs aleatorias del rango...\n");

    let mut found = 0;
    for i in 1..=10 {
        let ip = cidr.random_ip();
        print!("   [{}] Probando {}... ", i, ip);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        if let Some(info) = check_server(ip, 300, 400).await {
            println!("✅ ENCONTRADO!");
            println!("       🏷️  Domain: {}", info.domain);
            println!("       🎮  Version: {}", info.version);
            println!("       👥  Players: {}/{}", info.online_players, info.max_players);
            println!("       📝  MOTD: {}", info.motd);
            println!();
            found += 1;
        } else {
            println!("❌");
        }

        // Pequeña pausa para no saturar
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!("\n📈 Resultados:");
    println!("   Total escaneado: 10 IPs");
    println!("   Servidores encontrados: {}", found);
    println!("   Tasa de éxito: {}%", found * 10);
}
