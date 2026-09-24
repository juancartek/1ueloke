/// Ejemplo de exportación de resultados a diferentes formatos
///
/// Muestra cómo exportar información de servidores a TXT, JSON y CSV
use copenheimer_oxide::cidr::CIDR;
use copenheimer_oxide::export::{Exporter, ExportFormat};
use copenheimer_oxide::server::check_server;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("📤 COPENHEIMER - Ejemplo de Exportación\n");

    // Crear exportadores para cada formato
    let mut txt_exporter = Exporter::new(ExportFormat::Txt, "example_output.txt".to_string());
    let mut json_exporter = Exporter::new(ExportFormat::Json, "example_output.json".to_string());
    let mut csv_exporter = Exporter::new(ExportFormat::Csv, "example_output.csv".to_string());

    println!("📁 Archivos de salida:");
    println!("   • example_output.txt");
    println!("   • example_output.json");
    println!("   • example_output.csv\n");

    // Crear rango CIDR
    let cidr = CIDR::new("51.210.0.0", 18, "FR", "OVH/Gaming");

    println!("🔍 Escaneando hasta encontrar 5 servidores...\n");

    let mut found_count = 0;
    let mut attempts = 0;
    let max_attempts = 1000;

    while found_count < 5 && attempts < max_attempts {
        attempts += 1;
        let ip = cidr.random_ip();

        if let Some(info) = check_server(ip, 300, 400).await {
            found_count += 1;
            
            println!("✅ [{}] Servidor encontrado: {}", found_count, info.ip);
            println!("   {} - {}/{} jugadores", info.motd, info.online_players, info.max_players);

            // Exportar a todos los formatos
            if let Err(e) = txt_exporter.export(&info, &cidr.provider) {
                eprintln!("   ⚠️  Error exportando TXT: {}", e);
            }
            if let Err(e) = json_exporter.export(&info, &cidr.provider) {
                eprintln!("   ⚠️  Error exportando JSON: {}", e);
            }
            if let Err(e) = csv_exporter.export(&info, &cidr.provider) {
                eprintln!("   ⚠️  Error exportando CSV: {}", e);
            }
            println!();
        }

        if attempts % 100 == 0 {
            println!("   📊 Progreso: {} IPs verificadas, {} servidores encontrados", attempts, found_count);
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    if found_count == 5 {
        println!("\n✅ ¡Completado!");
        println!("   {} servidores exportados a 3 formatos diferentes", found_count);
    } else {
        println!("\n⚠️  Alcanzado límite de intentos");
        println!("   {} servidores encontrados en {} intentos", found_count, attempts);
    }

    println!("\n📂 Revisa los archivos generados en el directorio actual.");
}
