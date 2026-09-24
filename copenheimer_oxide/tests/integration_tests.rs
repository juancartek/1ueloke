/// Tests de integración para COPENHEIMER
use copenheimer_oxide::cidr::{CIDR, load_cidr_ranges};
use copenheimer_oxide::server::ServerInfo;
use copenheimer_oxide::export::{Exporter, ExportFormat};
use std::net::Ipv4Addr;

#[test]
fn test_cidr_operations() {
    let cidr = CIDR::new("192.168.0.0", 24, "US", "Test Provider");
    
    // Verificar tamaño del rango
    assert_eq!(cidr.ip_count(), 256);
    
    // Verificar contención de IPs
    let ip1: Ipv4Addr = "192.168.0.100".parse().unwrap();
    let ip2: Ipv4Addr = "192.168.1.100".parse().unwrap();
    assert!(cidr.contains(ip1));
    assert!(!cidr.contains(ip2));
    
    // Verificar generación de IPs aleatorias
    for _ in 0..10 {
        let random_ip = cidr.random_ip();
        assert!(cidr.contains(random_ip));
    }
}

#[test]
fn test_cidr_from_string() {
    let cidr_str = "10.0.0.0/16:TestProvider:US";
    let cidr = CIDR::from_string(cidr_str).unwrap();
    
    assert_eq!(cidr.ip_count(), 65536);
    assert_eq!(cidr.provider, "TestProvider");
    assert_eq!(cidr.country, "US");
}

#[test]
fn test_load_cidr_ranges() {
    let ranges = vec![
        "192.168.0.0/24:Provider1:US".to_string(),
        "10.0.0.0/16:Provider2:EU".to_string(),
    ];
    
    let cidrs = load_cidr_ranges(&ranges).unwrap();
    assert_eq!(cidrs.len(), 2);
    assert_eq!(cidrs[0].provider, "Provider1");
    assert_eq!(cidrs[1].provider, "Provider2");
}

#[test]
fn test_server_info_formatting() {
    let info = ServerInfo {
        ip: "1.2.3.4".to_string(),
        domain: "mc.example.com".to_string(),
        version: "1.20.1".to_string(),
        online_players: 15,
        max_players: 100,
        motd: "Welcome to Test Server".to_string(),
        protocol: 47,
    };
    
    // Test display string
    let display = info.to_display_string();
    assert!(display.contains("1.2.3.4"));
    assert!(display.contains("mc.example.com"));
    assert!(display.contains("1.20.1"));
    
    // Test CSV line
    let csv = info.to_csv_line();
    assert!(csv.contains("1.2.3.4,mc.example.com"));
}

#[test]
fn test_export_formats() {
    let info = ServerInfo {
        ip: "1.2.3.4".to_string(),
        domain: "test.com".to_string(),
        version: "1.20.1".to_string(),
        online_players: 10,
        max_players: 100,
        motd: "Test".to_string(),
        protocol: 47,
    };
    
    // Test TXT format
    let txt_exporter = Exporter::new(ExportFormat::Txt, "test_output.txt".to_string());
    // Verify creation
    assert_eq!(txt_exporter.file_path(), "test_output.txt");
    
    // Test JSON format
    let json_exporter = Exporter::new(ExportFormat::Json, "test_output.json".to_string());
    assert_eq!(json_exporter.file_path(), "test_output.json");
    
    // Test CSV format
    let csv_exporter = Exporter::new(ExportFormat::Csv, "test_output.csv".to_string());
    assert_eq!(csv_exporter.file_path(), "test_output.csv");
}

#[test]
fn test_export_format_conversion() {
    assert_eq!(ExportFormat::from_str("txt"), ExportFormat::Txt);
    assert_eq!(ExportFormat::from_str("TXT"), ExportFormat::Txt);
    assert_eq!(ExportFormat::from_str("json"), ExportFormat::Json);
    assert_eq!(ExportFormat::from_str("JSON"), ExportFormat::Json);
    assert_eq!(ExportFormat::from_str("csv"), ExportFormat::Csv);
    assert_eq!(ExportFormat::from_str("CSV"), ExportFormat::Csv);
    assert_eq!(ExportFormat::from_str("unknown"), ExportFormat::Txt);
}

#[test]
fn test_cidr_edge_cases() {
    // Test /32 (single IP)
    let cidr = CIDR::new("192.168.1.1", 32, "US", "Test");
    assert_eq!(cidr.ip_count(), 1);
    
    // Test /8 (large range)
    let cidr_large = CIDR::new("10.0.0.0", 8, "US", "Test");
    assert_eq!(cidr_large.ip_count(), 16777216);
}
