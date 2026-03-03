use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::timeout;
use serde_json::Value;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::fs::OpenOptions;
use std::io::{Write, BufRead};
use rand::Rng;
use std::env;
use dns_lookup::lookup_addr;

const MC_PORT: u16 = 25565;

fn format_count(count: u64) -> String {
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

#[derive(Debug, Clone)]
struct CIDR {
    net: u32,
    size: u32,
    provider: &'static str,
}

impl CIDR {
    fn new(ip: &str, bits: u8, provider: &'static str) -> Self {
        let addr: Ipv4Addr = ip.parse().unwrap();
        let net = u32::from(addr);
        let size = 1u32 << (32 - bits);
        Self { net, size, provider }
    }
}

use colored::*;

const LOGO: &str = r#"
   ____  _____  ____  ____  _      _      ____  _____  ____  ____  ____ 
  /  _ \/  __/ /  _ \/  _ \/ \  /|/ \__/|/  _ \/__ __\/  _ \/  _ \/  _ \
  | / \||  \   | / \|| | //| |\ ||| |\/||| / \|  / \  | | //| / \|| | //
  | \_/||  /_  | \_/|| |_\\| | \||| |  ||| \_/|  | |  | |_\\| \_/|| |_\\
  \____/\____\ \____/\____/\_/  \|\_/  \|\____/  \_/  \____/\____/\____/
"#;

fn print_logo() {
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
    println!("   {} {}", "🔥".red(), "Iniciando escaneo masivo...".yellow());
}

fn clean_motd(desc: &Value) -> String {
    let mut raw = String::new();
    if let Some(text) = desc.as_str() {
        raw = text.to_string();
    } else if let Some(obj) = desc.as_object() {
        if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
            raw.push_str(text);
        }
        if let Some(extra) = obj.get("extra").and_then(|v| v.as_array()) {
            for part in extra {
                if let Some(t) = part.get("text").and_then(|v| v.as_str()) {
                    raw.push_str(t);
                }
            }
        }
    }
    raw.chars().filter(|c| !c.is_control() && *c != '§').collect::<String>().replace('\n', " ").trim().to_string()
}

async fn check_server(ip: Ipv4Addr) -> Option<(Value, String)> {
    let addr = SocketAddr::new(IpAddr::V4(ip), MC_PORT);
    let mut stream = match timeout(Duration::from_millis(150), TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        _ => return None,
    };

    let ip_str = ip.to_string();
    let mut handshake = vec![0x00];
    handshake.extend_from_slice(&[0x2F]); 
    handshake.push(ip_str.len() as u8);
    handshake.extend_from_slice(ip_str.as_bytes());
    handshake.extend_from_slice(&[0x63, 0xDD]); 
    handshake.push(0x01); 

    let mut packet = vec![handshake.len() as u8];
    packet.extend(handshake);
    packet.extend_from_slice(&[0x01, 0x00]); 

    if stream.write_all(&packet).await.is_err() { return None; }

    let mut buf = vec![0; 8192];
    let n = match timeout(Duration::from_millis(400), stream.read(&mut buf)).await {
        Ok(Ok(n)) if n > 10 => n,
        _ => return None,
    };

    let res_str = String::from_utf8_lossy(&buf[..n]);
    if let Some(json_start) = res_str.find('{') {
        if let Ok(json) = serde_json::from_str::<Value>(&res_str[json_start..]) {
            let domain = lookup_addr(&IpAddr::V4(ip)).unwrap_or_else(|_| "N/A".to_string());
            return Some((json, domain));
        }
    }
    None
}

#[tokio::main]
async fn main() {
    loop {
        print_logo();
        let args: Vec<String> = env::args().collect();
        // ... (resto del menú)

    // MODO CHECK: Para actualizar datos de una IP específica
    if args.len() > 2 && args[1] == "--check" {
        if let Ok(ip) = args[2].parse::<Ipv4Addr>() {
            if let Some((info, domain)) = check_server(ip).await {
                let motd = clean_motd(&info["description"]);
                let online = info["players"]["online"].as_i64().unwrap_or(0);
                let max = info["players"]["max"].as_i64().unwrap_or(0);
                let version = info["version"]["name"].as_str().unwrap_or("Unknown");
                println!("{}", "   ╭──────────────────────────────────────────────────────────────────────────╮".red());
                println!("   🔥  IP: {:<15} 🏢  HOST: {:<20}", ip.to_string().cyan(), domain.yellow());
                println!("   🛠️   VER: {:<15} 👥  PLAYERS: {}/{}", version.green(), online.to_string().white(), max.to_string().white());
                println!("   📝  MOTD: {:<50}", motd.white());
                println!("{}", "   ╰──────────────────────────────────────────────────────────────────────────╯".red());
            } else {
                println!("{}", "OFFLINE".red());
            }
        }
        return;
    }

    println!("   {} Selecciona una opción:", "🎮".red());
    println!("   {} {}  {}  -  Búsqueda ilimitada", " [1]".red(), "🚀".green(), "Quick Scan".white());
    println!("   {} {}  {}  -  Buscar N servidores y parar", " [2]".red(), "🎯".cyan(), "Target Mode".white());
    println!();
    print!("   {} Opción > ", "👉".red());
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error leyendo entrada");
    let choice = input.trim();

    let limit_arg: Option<u64> = match choice {
        "1" => None,
        "2" => {
            print!("   {} Cuántos servidores quieres encontrar? > ", "🔢".yellow());
            let _ = std::io::stdout().flush();
            let mut limit_input = String::new();
            std::io::stdin().read_line(&mut limit_input).expect("Error leyendo límite");
            limit_input.trim().parse().ok()
        }
        _ => {
            println!("   {} Opción no válida. Iniciando Quick Scan por defecto...", "⚠️".yellow());
            None
        }
    };

    println!("   {} Selecciona la Intensidad:", "⚡".yellow());
    println!("   {} {}  {}  -  Para routers normales (Seguro)", " [1]".green(), "🏠", "HOME".white());
    println!("   {} {}  {}  -  Para fibra óptica (Rápido)", " [2]".yellow(), "🚀", "PRO".white());
    println!("   {} {}  {}  -  Para VPS/Dedicados (Extremo)", " [3]".red(), "🔥", "NITRO".white());
    println!();
    print!("   {} Intensidad > ", "👉".yellow());
    let _ = std::io::stdout().flush();
    
    let mut int_input = String::new();
    std::io::stdin().read_line(&mut int_input).expect("Error");
    let intensity = match int_input.trim() {
        "1" => 5_000,
        "2" => 25_000,
        "3" => 400_000,
        _ => 10_000,
    };

    println!("\n   {} {}", "🔥".red(), "Iniciando motor GIGA-NITRO...".yellow());
    let start_time = Instant::now();

    let target_cidrs = vec![
        CIDR::new("181.0.0.0", 8, "Residencial LATAM"),
        CIDR::new("190.0.0.0", 8, "Residencial LATAM"),
        CIDR::new("200.0.0.0", 8, "Residencial LATAM"),
        CIDR::new("70.0.0.0", 8, "Residencial US"),
        CIDR::new("80.0.0.0", 8, "Residencial EU"),
        CIDR::new("51.254.0.0", 15, "OVH Game"),
        CIDR::new("144.76.0.0", 16, "Hetzner DE"),
        CIDR::new("129.146.0.0", 16, "Oracle Cloud US"),
        CIDR::new("150.136.0.0", 16, "Oracle Cloud US"),
        CIDR::new("37.187.0.0", 16, "Aternos/OVH"),
    ];

    let (tx, mut rx) = mpsc::channel(100000);
    let semaphore = Arc::new(Semaphore::new(intensity)); 
    let cidrs = Arc::new(target_cidrs);
    let total_scanned = Arc::new(AtomicU64::new(0));
    let total_found = Arc::new(AtomicU64::new(0));
    let scanning = Arc::new(AtomicBool::new(true));

    for _ in 0..intensity {
        let sem = semaphore.clone();
        let cidrs = cidrs.clone();
        let tx = tx.clone();
        let ts = total_scanned.clone();
        let sc = scanning.clone();

        tokio::spawn(async move {
            loop {
                if !sc.load(Ordering::Relaxed) { break; }
                let _permit = match sem.acquire().await { Ok(p) => p, Err(_) => break };
                let (ip, cidr_info) = {
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..cidrs.len());
                    let cidr = &cidrs[idx];
                    let ip_int = cidr.net + rng.gen_range(0..cidr.size);
                    (Ipv4Addr::from(ip_int), cidr.clone())
                };
                if let Some((info, domain)) = check_server(ip).await {
                    if tx.send((ip, info, cidr_info, domain)).await.is_err() {
                        break;
                    }
                }
                ts.fetch_add(1, Ordering::Relaxed);
                drop(_permit);
            }
        });
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("hits_omega.txt")
        .expect("No se pudo abrir hits_omega.txt");

    let mut return_to_menu = false;
    while let Some((ip, info, cidr, domain)) = rx.recv().await {
        let motd = clean_motd(&info["description"]);
        let online = info["players"]["online"].as_i64().unwrap_or(0);
        let max = info["players"]["max"].as_i64().unwrap_or(0);
        let version = info["version"]["name"].as_str().unwrap_or("Unknown");
        let current_hits = total_found.fetch_add(1, Ordering::Relaxed) + 1;
        
        let log_entry = format!("🔥 IP: {} | DOMAIN: {} | HOST: {} | VER: {} | PLAYERS: {}/{} | MOTD: {}", ip, domain, cidr.provider, version, online, max, motd);
        
        println!("{}", "   ╭──────────────────────────────────────────────────────────────────────────╮".red());
        println!("   🔥  IP: {:<15} 🏢  HOST: {:<20}", ip.to_string().cyan(), cidr.provider.yellow());
        println!("   🛠️   VER: {:<15} 👥  PLAYERS: {}/{}", version.green(), online.to_string().white(), max.to_string().white());
        println!("   📝  MOTD: {:<50}", motd.white());
        println!("{}", "   ╰──────────────────────────────────────────────────────────────────────────╯".red());

        if let Err(e) = writeln!(file, "{}", log_entry) {
            eprintln!("Error escribiendo en hits_omega.txt: {}", e);
        }

        if let Some(l) = limit_arg {
            if current_hits >= l {
                scanning.store(false, Ordering::Relaxed);
                let duration = start_time.elapsed();
                let scanned = total_scanned.load(Ordering::Relaxed);
                println!("\n   {} Objetivo alcanzado.", "🏁".green());
                println!("   {} Tiempo total: {:?}.", "⏱️".cyan(), duration);
                println!("   {} IPs verificadas: {}.", "🔍".magenta(), format_count(scanned));
                println!("\n   {} Presiona [ENTER] para volver al menú principal...", "👉".yellow());
                let _ = std::io::stdout().flush();
                let mut temp = String::new();
                let _ = std::io::stdin().read_line(&mut temp);
                return_to_menu = true;
                break;
            }
        }
    }
    if return_to_menu {
        continue;
    }
}
}
