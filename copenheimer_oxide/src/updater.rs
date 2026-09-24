/// Módulo de auto-actualización
///
/// Verifica y descarga actualizaciones desde GitHub automáticamente
use colored::*;
use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

const REPO_OWNER: &str = "juancartek";
const REPO_NAME: &str = "1ueloke";
const VERSION_FILE: &str = "version.txt";

/// Información de versión
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub local: String,
    pub remote: String,
    pub update_available: bool,
}

/// Estructura de respuesta de GitHub API para releases
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

/// Obtiene la versión local desde version.txt
pub fn get_local_version() -> Result<String, Box<dyn std::error::Error>> {
    let version = fs::read_to_string(VERSION_FILE)?
        .trim()
        .to_string();
    Ok(version)
}

/// Obtiene la última versión desde GitHub
pub async fn get_remote_version() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );

    let client = reqwest::Client::builder()
        .user_agent("copenheimer-updater")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Intentar obtener desde releases
    if let Ok(response) = client.get(&url).send().await {
        if response.status().is_success() {
            if let Ok(release) = response.json::<GitHubRelease>().await {
                let version = release.tag_name.trim_start_matches('v').to_string();
                return Ok(version);
            }
        }
    }

    // Fallback: obtener desde version.txt en main branch
    let raw_url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/version.txt",
        REPO_OWNER, REPO_NAME
    );

    let response = client.get(&raw_url).send().await?;
    let version = response.text().await?.trim().to_string();

    Ok(version)
}

/// Compara dos versiones en formato X.Y.Z
///
/// Retorna true si v1 > v2
fn version_greater_than(v1: &str, v2: &str) -> bool {
    let v1_parts: Vec<u32> = v1.split('.').filter_map(|s| s.parse().ok()).collect();
    let v2_parts: Vec<u32> = v2.split('.').filter_map(|s| s.parse().ok()).collect();

    for i in 0..3 {
        let p1 = v1_parts.get(i).unwrap_or(&0);
        let p2 = v2_parts.get(i).unwrap_or(&0);

        if p1 > p2 {
            return true;
        } else if p1 < p2 {
            return false;
        }
    }

    false
}

/// Verifica si hay actualizaciones disponibles
pub async fn check_for_updates() -> Result<VersionInfo, Box<dyn std::error::Error>> {
    let local = get_local_version()?;
    let remote = get_remote_version().await?;
    let update_available = version_greater_than(&remote, &local);

    Ok(VersionInfo {
        local,
        remote,
        update_available,
    })
}

/// Verifica si git está disponible
fn is_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Verifica si el directorio actual es un repositorio git
fn is_git_repo() -> bool {
    Path::new(".git").exists()
}

/// Actualiza usando git pull
async fn update_with_git() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔄 Actualizando con git...".cyan());

    // Guardar cambios locales si existen
    let status = Command::new("git")
        .args(&["diff-index", "--quiet", "HEAD", "--"])
        .status()?;

    if !status.success() {
        println!("{}", "   💾 Guardando cambios locales...".yellow());
        Command::new("git")
            .args(&["stash", "save", "Auto-update backup"])
            .status()?;
    }

    // Fetch y pull
    println!("{}", "   ⬇️  Descargando actualizaciones...".cyan());
    Command::new("git")
        .args(&["fetch", "origin"])
        .status()?;

    let pull_status = Command::new("git")
        .args(&["pull", "origin", "main"])
        .status()?;

    if !pull_status.success() {
        return Err("Error al hacer git pull".into());
    }

    Ok(())
}

/// Descarga y actualiza desde GitHub (sin git)
async fn update_with_download() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📥 Descargando desde GitHub...".cyan());

    let url = format!(
        "https://github.com/{}/{}/archive/refs/heads/main.tar.gz",
        REPO_OWNER, REPO_NAME
    );

    // Crear directorio temporal
    let temp_dir = env::temp_dir().join(format!("copenheimer_update_{}", std::process::id()));
    fs::create_dir_all(&temp_dir)?;

    // Descargar archivo
    println!("{}", "   ⬇️  Descargando...".cyan());
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let bytes = response.bytes().await?;

    let archive_path = temp_dir.join("repo.tar.gz");
    let mut file = File::create(&archive_path)?;
    file.write_all(&bytes)?;

    // Extraer
    println!("{}", "   📂 Extrayendo archivos...".cyan());
    let tar = Command::new("tar")
        .args(&["-xzf", archive_path.to_str().unwrap(), "-C", temp_dir.to_str().unwrap()])
        .status()?;

    if !tar.success() {
        return Err("Error al extraer el archivo".into());
    }

    // Copiar archivos actualizados
    println!("{}", "   📝 Actualizando archivos...".cyan());
    let extract_dir = temp_dir.join(format!("{}-main", REPO_NAME));

    // Actualizar código fuente
    let src_dir = extract_dir.join("copenheimer_oxide");
    if src_dir.exists() {
        copy_dir_all(&src_dir, Path::new("copenheimer_oxide"))?;
    }

    // Actualizar version.txt
    let version_file = extract_dir.join("version.txt");
    if version_file.exists() {
        fs::copy(&version_file, "version.txt")?;
    }

    // Actualizar archivos de documentación
    for file in &["README.md", "CHANGELOG.md", "INSTALL.md", "TUTORIAL.md", "QUICKSTART.md"] {
        let src = extract_dir.join(file);
        if src.exists() {
            fs::copy(&src, file)?;
        }
    }

    // Limpiar
    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

/// Copia un directorio recursivamente
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Recompila el proyecto
fn rebuild_project() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("{}", "🔨 Recompilando proyecto...".cyan());

    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("copenheimer_oxide")
        .status()?;

    if !status.success() {
        return Err("Error al compilar el proyecto".into());
    }

    println!("{}", "✅ Compilación exitosa".green());
    Ok(())
}

/// Ejecuta la actualización completa
pub async fn run_updater() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("{}", "╔════════════════════════════════════════════════════╗".blue());
    println!("{}", "║     🔥 COPENHEIMER AUTO-UPDATER 🔥                ║".blue());
    println!("{}", "╚════════════════════════════════════════════════════╝".blue());
    println!();

    // Verificar conexión a Internet
    print!("{}", "🌐 Verificando conexión... ".cyan());
    io::stdout().flush()?;
    
    let client = reqwest::Client::new();
    if client.get("https://github.com").send().await.is_err() {
        println!("{}", "❌".red());
        return Err("No hay conexión a Internet".into());
    }
    println!("{}", "✅".green());

    // Decidir método de actualización
    if is_git_repo() && is_git_available() {
        println!("{}", "🔍 Repositorio git detectado".cyan());
        update_with_git().await?;
    } else {
        println!("{}", "📦 Usando descarga directa".cyan());
        update_with_download().await?;
    }

    // Recompilar
    if let Err(e) = rebuild_project() {
        println!();
        println!("{}", "⚠️  No se pudo recompilar automáticamente".yellow());
        println!("{}", format!("   Error: {}", e).yellow());
        println!("{}", "   Recompila manualmente con: cd copenheimer_oxide && cargo build --release".yellow());
        return Ok(()); // No es error fatal
    }

    // Mostrar nueva versión
    println!();
    if let Ok(version) = get_local_version() {
        println!("{}", format!("✨ Actualización completada exitosamente - v{}", version).green().bold());
    }

    println!();
    println!("{}", "🔄 Reinicia el programa para usar la nueva versión".cyan());
    
    Ok(())
}

/// Muestra información sobre actualizaciones disponibles
pub fn display_update_info(info: &VersionInfo) {
    if info.update_available {
        println!();
        println!("{}", "═══════════════════════════════════════════════════".yellow());
        println!("{}", "   ✨ NUEVA ACTUALIZACIÓN DISPONIBLE ✨".green().bold());
        println!("{}", "═══════════════════════════════════════════════════".yellow());
        println!(
            "   {} {} → {}",
            "Versión:".cyan(),
            format!("v{}", info.local).yellow(),
            format!("v{}", info.remote).green().bold()
        );
        println!("{}", "═══════════════════════════════════════════════════".yellow());
        println!();
    }
}

/// Pregunta al usuario si quiere actualizar
pub fn prompt_update() -> bool {
    use std::io::{self, Write};
    
    print!("{}", "   ¿Deseas actualizar ahora? (s/n): ".cyan().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    matches!(input.trim().to_lowercase().as_str(), "s" | "si" | "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(version_greater_than("2.0.0", "1.9.9"));
        assert!(version_greater_than("1.10.0", "1.9.0"));
        assert!(version_greater_than("1.0.1", "1.0.0"));
        assert!(!version_greater_than("1.0.0", "1.0.0"));
        assert!(!version_greater_than("1.0.0", "1.0.1"));
    }

    #[test]
    fn test_local_version() {
        // Este test solo funciona si existe version.txt
        if let Ok(version) = get_local_version() {
            assert!(!version.is_empty());
            assert!(version.contains('.'));
        }
    }
}
