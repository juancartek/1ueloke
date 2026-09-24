/// Ejemplo de uso del módulo de actualización
///
/// Este ejemplo muestra cómo verificar y aplicar actualizaciones
/// programáticamente.

use copenheimer_oxide::updater;

#[tokio::main]
async fn main() {
    println!("🔍 Verificando actualizaciones...\n");

    // Verificar si hay actualizaciones disponibles
    match updater::check_for_updates().await {
        Ok(info) => {
            println!("📦 Versión local:  v{}", info.local);
            println!("☁️  Versión remota: v{}", info.remote);
            println!();

            if info.update_available {
                println!("✨ ¡Nueva actualización disponible!");
                println!();

                // Mostrar información de la actualización
                updater::display_update_info(&info);

                // Preguntar si actualizar
                if updater::prompt_update() {
                    println!();
                    println!("🔄 Iniciando actualización...");
                    println!();

                    match updater::run_updater().await {
                        Ok(_) => {
                            println!();
                            println!("✅ Actualización completada exitosamente");
                        }
                        Err(e) => {
                            eprintln!();
                            eprintln!("❌ Error durante la actualización: {}", e);
                        }
                    }
                } else {
                    println!("⏭️  Actualización omitida");
                }
            } else {
                println!("✅ Ya estás usando la última versión");
            }
        }
        Err(e) => {
            eprintln!("❌ Error verificando actualizaciones: {}", e);
        }
    }
}
