/// Módulo de parsing de respuestas de servidores Minecraft
///
/// Maneja la comunicación con servidores Minecraft y el parsing de información
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use dns_lookup::lookup_addr;
use serde::{Deserialize, Serialize};

const MC_PORT: u16 = 25565;

/// Información de un servidor Minecraft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub ip: String,
    pub domain: String,
    pub version: String,
    pub online_players: i64,
    pub max_players: i64,
    pub motd: String,
    pub protocol: i64,
}

impl ServerInfo {
    /// Crea una instancia de ServerInfo desde JSON y datos adicionales
    pub fn from_json(ip: Ipv4Addr, json: &Value, domain: String) -> Self {
        let version = json["version"]["name"].as_str().unwrap_or("Unknown").to_string();
        let online_players = json["players"]["online"].as_i64().unwrap_or(0);
        let max_players = json["players"]["max"].as_i64().unwrap_or(0);
        let protocol = json["version"]["protocol"].as_i64().unwrap_or(0);
        let motd = parse_motd(&json["description"]);

        Self {
            ip: ip.to_string(),
            domain,
            version,
            online_players,
            max_players,
            motd,
            protocol,
        }
    }

    /// Formatea la información como string legible
    pub fn to_display_string(&self) -> String {
        format!(
            "IP: {} | Domain: {} | Ver: {} | Players: {}/{} | MOTD: {}",
            self.ip, self.domain, self.version, self.online_players, self.max_players, self.motd
        )
    }

    /// Convierte a formato CSV
    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.ip,
            self.domain,
            self.version,
            self.online_players,
            self.max_players,
            self.motd.replace(',', ";")
        )
    }
}

/// Parsea el MOTD (Message of the Day) desde JSON
///
/// Maneja tanto formato simple (string) como complejo (objeto con texto y extras)
///
/// # Argumentos
///
/// * `desc` - Valor JSON con la descripción del servidor
fn parse_motd(desc: &Value) -> String {
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

    // Limpiar caracteres de control y códigos de color de Minecraft
    raw.chars()
        .filter(|c| !c.is_control() && *c != '§')
        .collect::<String>()
        .replace('\n', " ")
        .trim()
        .to_string()
}

/// Codifica un varint (variable-length integer) usado en el protocolo de Minecraft
///
/// # Argumentos
///
/// * `buf` - Buffer donde agregar los bytes del varint
/// * `value` - Valor a codificar
#[inline(always)]
fn push_varint(buf: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        buf.push(temp);
        if value == 0 {
            break;
        }
    }
}

/// Verifica si un servidor Minecraft está activo y obtiene su información
///
/// # Argumentos
///
/// * `ip` - Dirección IP del servidor
/// * `timeout_ms` - Timeout de conexión en milisegundos
/// * `read_timeout_ms` - Timeout de lectura en milisegundos
///
/// # Retorna
///
/// `Some(ServerInfo)` si el servidor responde correctamente, `None` en caso contrario
pub async fn check_server(
    ip: Ipv4Addr,
    timeout_ms: u64,
    read_timeout_ms: u64,
) -> Option<ServerInfo> {
    let addr = SocketAddr::new(IpAddr::V4(ip), MC_PORT);
    
    // Intentar conectar con timeout
    let mut stream = match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        _ => return None,
    };

    let _ = stream.set_nodelay(true);

    // Construir handshake packet
    let ip_str = ip.to_string();
    let mut handshake = Vec::with_capacity(ip_str.len() + 10);
    handshake.push(0x00); // Packet ID
    push_varint(&mut handshake, 47); // Protocol version
    push_varint(&mut handshake, ip_str.len() as i32);
    handshake.extend_from_slice(ip_str.as_bytes());
    handshake.extend_from_slice(&MC_PORT.to_be_bytes());
    push_varint(&mut handshake, 1); // Next state (status)

    // Construir packet completo
    let mut packet = Vec::with_capacity(handshake.len() + 5);
    push_varint(&mut packet, handshake.len() as i32);
    packet.extend(handshake);
    packet.extend_from_slice(&[0x01, 0x00]); // Status request

    // Enviar packet
    if stream.write_all(&packet).await.is_err() {
        return None;
    }

    // Leer respuesta con buffer más grande para MOTDs largos
    let mut response = vec![0; 16384];
    let n = match timeout(Duration::from_millis(read_timeout_ms), stream.read(&mut response)).await {
        Ok(Ok(n)) if n > 10 => n,
        _ => return None,
    };

    // Parsear JSON de la respuesta
    let res_str = String::from_utf8_lossy(&response[..n]);
    
    // Buscar el inicio y fin del JSON
    if let Some(json_start) = res_str.find('{') {
        // Buscar el último } para encontrar el fin del JSON
        let json_slice = &res_str[json_start..];
        if let Some(json_end) = json_slice.rfind('}') {
            let json_str = &json_slice[..=json_end];
            
            if let Ok(json_data) = serde_json::from_str::<Value>(json_str) {
                // Resolver dominio en blocking task para no bloquear el runtime de Tokio
                let ip_addr = IpAddr::V4(ip);
                let domain = tokio::task::spawn_blocking(move || {
                    lookup_addr(&ip_addr).unwrap_or_else(|_| "N/A".to_string())
                })
                .await
                .unwrap_or_else(|_| "N/A".to_string());
                
                return Some(ServerInfo::from_json(ip, &json_data, domain));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motd_simple() {
        let json: Value = serde_json::from_str(r#""Simple MOTD""#).unwrap();
        let motd = parse_motd(&json);
        assert_eq!(motd, "Simple MOTD");
    }

    #[test]
    fn test_parse_motd_complex() {
        let json: Value = serde_json::from_str(
            r#"{"text":"Welcome ","extra":[{"text":"to "},{"text":"server"}]}"#
        ).unwrap();
        let motd = parse_motd(&json);
        assert_eq!(motd, "Welcome to server");
    }

    #[test]
    fn test_server_info_to_csv() {
        let info = ServerInfo {
            ip: "1.2.3.4".to_string(),
            domain: "example.com".to_string(),
            version: "1.20.1".to_string(),
            online_players: 10,
            max_players: 100,
            motd: "Test Server".to_string(),
            protocol: 47,
        };
        
        let csv = info.to_csv_line();
        assert!(csv.contains("1.2.3.4"));
        assert!(csv.contains("example.com"));
    }

    #[test]
    fn test_push_varint() {
        let mut buf = Vec::new();
        push_varint(&mut buf, 127);
        assert_eq!(buf, vec![127]);

        buf.clear();
        push_varint(&mut buf, 128);
        assert_eq!(buf, vec![128, 1]);
    }
}
