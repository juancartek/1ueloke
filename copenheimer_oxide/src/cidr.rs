/// Módulo de gestión de rangos CIDR
///
/// Maneja la representación y manipulación de rangos de IPs en notación CIDR
use std::net::Ipv4Addr;
use rand::Rng;

/// Representa un rango CIDR con información adicional
#[derive(Debug, Clone)]
pub struct CIDR {
    /// Dirección de red base (en formato u32)
    pub net: u32,
    /// Tamaño del rango (cantidad de IPs)
    pub size: u32,
    /// País o región
    pub country: String,
    /// Proveedor de hosting
    pub provider: String,
}

impl CIDR {
    /// Crea un nuevo rango CIDR
    ///
    /// # Argumentos
    ///
    /// * `ip` - Dirección IP base (ej: "192.168.0.0")
    /// * `bits` - Bits de máscara (ej: 24 para /24)
    /// * `country` - País o región
    /// * `provider` - Proveedor de hosting
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use copenheimer_oxide::cidr::CIDR;
    /// let cidr = CIDR::new("192.168.0.0", 24, "US", "Example Provider");
    /// ```
    pub fn new(ip: &str, bits: u8, country: &str, provider: &str) -> Self {
        let addr: Ipv4Addr = ip.parse().expect("IP inválida");
        let net = u32::from(addr);
        let size = 1u32 << (32 - bits);
        Self {
            net,
            size,
            country: country.to_string(),
            provider: provider.to_string(),
        }
    }

    /// Crea un CIDR desde formato string "ip/bits:provider:country"
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use copenheimer_oxide::cidr::CIDR;
    /// let cidr = CIDR::from_string("192.168.0.0/24:Example:US").unwrap();
    /// ```
    pub fn from_string(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(format!("Formato inválido: {}", s));
        }

        let ip_parts: Vec<&str> = parts[0].split('/').collect();
        if ip_parts.len() != 2 {
            return Err(format!("Formato IP/bits inválido: {}", parts[0]));
        }

        let ip = ip_parts[0];
        let bits: u8 = ip_parts[1].parse().map_err(|_| format!("Bits inválidos: {}", ip_parts[1]))?;
        let provider = parts[1];
        let country = parts[2];

        Ok(Self::new(ip, bits, country, provider))
    }

    /// Genera una IP aleatoria dentro del rango CIDR
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use copenheimer_oxide::cidr::CIDR;
    /// let cidr = CIDR::new("192.168.0.0", 24, "US", "Example");
    /// let ip = cidr.random_ip();
    /// ```
    pub fn random_ip(&self) -> Ipv4Addr {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(0..self.size);
        Ipv4Addr::from(self.net + offset)
    }

    /// Verifica si una IP pertenece a este rango CIDR
    ///
    /// # Argumentos
    ///
    /// * `ip` - IP a verificar
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let ip_u32 = u32::from(ip);
        ip_u32 >= self.net && ip_u32 < (self.net + self.size)
    }

    /// Retorna la cantidad de IPs en el rango
    pub fn ip_count(&self) -> u32 {
        self.size
    }
}

/// Carga múltiples rangos CIDR desde una lista de strings
///
/// # Argumentos
///
/// * `ranges` - Vector de strings en formato "ip/bits:provider:country"
pub fn load_cidr_ranges(ranges: &[String]) -> Result<Vec<CIDR>, String> {
    ranges
        .iter()
        .map(|s| CIDR::from_string(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_creation() {
        let cidr = CIDR::new("192.168.0.0", 24, "US", "Test Provider");
        assert_eq!(cidr.size, 256);
        assert_eq!(cidr.country, "US");
        assert_eq!(cidr.provider, "Test Provider");
    }

    #[test]
    fn test_cidr_from_string() {
        let cidr = CIDR::from_string("192.168.0.0/24:TestProvider:US").unwrap();
        assert_eq!(cidr.size, 256);
        assert_eq!(cidr.country, "US");
        assert_eq!(cidr.provider, "TestProvider");
    }

    #[test]
    fn test_cidr_contains() {
        let cidr = CIDR::new("192.168.0.0", 24, "US", "Test");
        let ip1: Ipv4Addr = "192.168.0.100".parse().unwrap();
        let ip2: Ipv4Addr = "192.168.1.100".parse().unwrap();
        
        assert!(cidr.contains(ip1));
        assert!(!cidr.contains(ip2));
    }

    #[test]
    fn test_random_ip_in_range() {
        let cidr = CIDR::new("192.168.0.0", 24, "US", "Test");
        for _ in 0..100 {
            let ip = cidr.random_ip();
            assert!(cidr.contains(ip));
        }
    }

    #[test]
    fn test_ip_count() {
        let cidr1 = CIDR::new("192.168.0.0", 24, "US", "Test");
        assert_eq!(cidr1.ip_count(), 256);

        let cidr2 = CIDR::new("10.0.0.0", 16, "US", "Test");
        assert_eq!(cidr2.ip_count(), 65536);
    }
}
