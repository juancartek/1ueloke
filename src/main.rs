use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::timeout;
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::fs::OpenOptions;
use std::io::{Write, stdout};
use colored::*;
use rand::Rng;
use dns_lookup::lookup_addr;

const MC_PORT: u16 = 25565;

#[derive(Debug, Clone)]
struct CIDR {
    net: u32,
    size: u32,
    country: &'static str,
    provider: &'static str,
}

impl CIDR {
    fn new(ip: &str, bits: u8, country: &'static str, provider: &'static str) -> Self {
        let addr: Ipv4Addr = ip.parse().unwrap();
        let net = u32::from(addr);
        let size = 1u32 << (32 - bits);
        Self { net, size, country, provider }
    }
}

#[inline(always)]
fn push_varint(buf: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 { temp |= 0b10000000; }
        buf.push(temp);
        if value == 0 { break; }
    }
}

fn parse_motd(desc: &Value) -> String {
    let mut raw_motd = String::new();
    if let Some(text) = desc.as_str() {
        raw_motd = text.to_string();
    } else if let Some(obj) = desc.as_object() {
        if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
            raw_motd.push_str(text);
        }
        if let Some(extra) = obj.get("extra").and_then(|v| v.as_array()) {
            for part in extra {
                if let Some(part_text) = part.get("text").and_then(|v| v.as_str()) {
                    raw_motd.push_str(part_text);
                }
            }
        }
    }
    raw_motd.chars().filter(|c| !c.is_control() && *c != '§').collect::<String>().replace('\n', " ").trim().to_string()
}

async fn check_server(ip: Ipv4Addr) -> Option<(Value, String)> {
    let addr = SocketAddr::new(IpAddr::V4(ip), MC_PORT);
    let mut stream = match timeout(Duration::from_millis(300), TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        _ => return None,
    };
    let _ = stream.set_nodelay(true);

    let ip_str = ip.to_string();
    let mut handshake = Vec::with_capacity(ip_str.len() + 10);
    handshake.push(0x00); 
    push_varint(&mut handshake, 47); 
    push_varint(&mut handshake, ip_str.len() as i32);
    handshake.extend_from_slice(ip_str.as_bytes());
    handshake.extend_from_slice(&MC_PORT.to_be_bytes());
    push_varint(&mut handshake, 1); 

    let mut packet = Vec::with_capacity(handshake.len() + 5);
    push_varint(&mut packet, handshake.len() as i32);
    packet.extend(handshake);
    packet.extend_from_slice(&[0x01, 0x00]);

    if stream.write_all(&packet).await.is_err() { return None; }

    let mut response = vec![0; 4096];
    let n = match timeout(Duration::from_millis(400), stream.read(&mut response)).await {
        Ok(Ok(n)) if n > 10 => n,
        _ => return None,
    };

    let res_str = String::from_utf8_lossy(&response[..n]);
    if let Some(json_start) = res_str.find('{') {
        if let Ok(json_data) = serde_json::from_str::<Value>(&res_str[json_start..]) {
            let ip_addr = IpAddr::V4(ip);
            let domain = lookup_addr(&ip_addr).unwrap_or_else(|_| "No Domain".to_string());
            return Some((json_data, domain));
        }
    }
    None
}

async fn get_input(prompt: &str) -> String {
    print!("{}", prompt.cyan());
    stdout().flush().unwrap();
    let mut input = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    reader.read_line(&mut input).await.unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    let target_cidrs = vec![
        CIDR::new("51.0.0.0", 8, "EU", "OVH/Gaming"),
        CIDR::new("15.235.0.0", 16, "CA/BR", "OVH/PebbleHost"),
        CIDR::new("15.204.0.0", 16, "US", "OVH/Gaming"),
        CIDR::new("66.70.0.0", 16, "CA", "OVH/Bisect"),
        CIDR::new("135.148.0.0", 16, "US", "OVH/Sparked"),
        CIDR::new("135.125.0.0", 16, "EU", "OVH"),
        CIDR::new("144.76.0.0", 16, "DE", "Hetzner/Lemehost"),
        CIDR::new("95.216.0.0", 15, "FI", "Hetzner/Minehost"),
        CIDR::new("161.97.0.0", 16, "DE", "Contabo"),
        CIDR::new("31.220.0.0", 16, "US/NL", "Hostinger"),
        CIDR::new("185.150.188.0", 22, "EU", "Shockbyte"),
        CIDR::new("192.187.104.0", 21, "US", "MelonCube"),
        CIDR::new("198.50.141.0", 24, "CA", "Apex Hosting"),
        CIDR::new("45.35.0.0", 16, "US", "ReliableSite/Hostings"),
        CIDR::new("147.135.0.0", 16, "US", "OVH/Game"),
    ];

    println!("{}", r#"
  _    _ _      _______ _____           _   _ _____ _______ _____   ____  
 | |  | | |    |__   __|  __ \    /\   | \ | |_   _|__   __|  __ \ / __ \ 
 | |  | | |       | |  | |__) |  /  \  |  \| | | |    | |  | |__) | |  | |
 | |  | | |       | |  |  _  /  / /\ \ | . ` | | |    | |  |  _  /| |  | |
 | |__| | |____   | |  | | \ \ / ____ \| |\  |_| |_   | |  | | \ \| |__| |
  \____/|______|  |_|  |_|  \_\/_/    \_\_| \_|_____|  |_|  |_|  \_\\____/ 
    "#.bright_red().bold());

    println!("{}", "==========================================================".bright_black());
    println!("  {}  {} v4.5 GIGA-PANEL - C++ Extreme Power", "💥".green(), "COPENHEIMER".bright_white().bold());
    println!("{}", "==========================================================".bright_black());
    println!("  [1] {}  {}  -  Unlimited scanning mode", "🚀".green(), "Quick Scan".white());
    println!("  [2] {}  {}  -  Find N servers and stop", "🎯".cyan(), "Target Mode".white());
    println!("  [3] {}  {}  -  Filter by Players/Country", "🔍".yellow(), "Filter Mode".white());
    println!("  [4] {}  {}  -  Detailed information mode", "📊".magenta(), "Deep Scan".white());
    println!("{}", "----------------------------------------------------------".bright_black());

    let choice = get_input("  🕹️  Selecciona una opción (1-4): ").await;
    let mut target_count: u64 = 0;
    let mut min_players: i64 = 0;
    let mut country_filter: String = String::new();
    let mut deep_mode = false;

    match choice.as_str() {
        "2" => {
            let n = get_input("  🔢 Cantidad deseada: ").await;
            target_count = n.parse().unwrap_or(10);
        },
        "3" => {
            let p = get_input("  👥 Min jugadores: ").await;
            min_players = p.parse().unwrap_or(0);
            country_filter = get_input("  🌍 País (FR, DE, US, EU): ").await.to_uppercase();
        },
        "4" => { deep_mode = true; },
        _ => {}
    }

    println!("\n{} {}", "🔥".red(), "[*] Iniciando motor GIGA-NITRO (65,000 tasks)...".yellow());

    let mut log_file = OpenOptions::new().create(true).append(true).open("found_oxide.txt").unwrap();
    let (tx, mut rx) = mpsc::channel(5000); 
    let semaphore = Arc::new(Semaphore::new(65000)); 
    let cidrs = Arc::new(target_cidrs);
    let total_scanned = Arc::new(AtomicU64::new(0));
    let total_found = Arc::new(AtomicU64::new(0));

    // Monitor PPS GIGA
    let ts_clone = total_scanned.clone();
    let tf_clone = total_found.clone();
    tokio::spawn(async move {
        let mut last_scanned = 0;
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let current = ts_clone.load(Ordering::Relaxed);
            let pps = (current - last_scanned) * 2;
            last_scanned = current;
            print!("\r  {} PPS: {} | Scanned: {} | Found: {} ", "🚀".bright_green(), pps, current, tf_clone.load(Ordering::Relaxed));
            stdout().flush().unwrap();
        }
    });

    // Workers masivos
    for _ in 0..65000 {
        let sem = semaphore.clone();
        let cidrs = cidrs.clone();
        let tx = tx.clone();
        let ts = total_scanned.clone();

        tokio::spawn(async move {
            loop {
                let _permit = match sem.acquire().await { Ok(p) => p, Err(_) => break };
                
                let (ip, cidr_info) = {
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..cidrs.len());
                    let cidr = &cidrs[idx];
                    let ip_int = cidr.net + rng.gen_range(0..cidr.size);
                    (Ipv4Addr::from(ip_int), cidr.clone())
                };

                if let Some((info, domain)) = check_server(ip).await {
                    let _ = tx.send((ip, info, cidr_info, domain)).await;
                }
                ts.fetch_add(1, Ordering::Relaxed);
                drop(_permit);
            }
        });
    }

    // Listener de resultados
    while let Some((ip, info, cidr, domain)) = rx.recv().await {
        let motd = parse_motd(&info["description"]);
        let online = info["players"]["online"].as_i64().unwrap_or(0);
        let max = info["players"]["max"].as_i64().unwrap_or(0);
        let version = info["version"]["name"].as_str().unwrap_or("Unknown");

        // Filtros
        if online < min_players { continue; }
        if !country_filter.is_empty() && !cidr.country.contains(&country_filter) { continue; }

        total_found.fetch_add(1, Ordering::Relaxed);
        let current_found = total_found.load(Ordering::Relaxed);
        
        if deep_mode {
            println!("\n  {} IP: {} | {} | Domain: {}", "💎".green(), ip.to_string().cyan(), cidr.country.yellow(), domain.white().bold());
            println!("     {} Host: {} | Version: {} | Players: {}/{} | MOTD: {}", "🛠️".white(), cidr.provider, version, online, max, motd);
        } else {
            println!("\n  {} {} | {} | {} | {}/{} | {}", "✅".green(), ip.to_string().cyan(), domain.white().bold(), cidr.country.yellow(), online, max, motd);
        }

        let _ = writeln!(log_file, "IP: {} | Domain: {} | Host: {} | Version: {} | Players: {}/{} | MOTD: {}", ip, domain, cidr.provider, version, online, max, motd);

        if target_count > 0 && current_found >= target_count {
            println!("\n  {} Objetivo alcanzado.", "🏁".green());
            std::process::exit(0);
        }
    }
}
