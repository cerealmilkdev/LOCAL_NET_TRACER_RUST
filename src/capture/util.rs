use pnet::util::{MacAddr, checksum, ipv4_checksum, ipv6_checksum};
use std::net::IpAddr;

/// Constantes réseau utiles
pub const ETHERNET_HEADER_LEN: usize = 14;
pub const IPV4_HEADER_LEN: usize = 20;
pub const TCP_HEADER_LEN: usize = 20;
pub const UDP_HEADER_LEN: usize = 8;

/// Types de protocoles Ethernet courants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EtherType {
    IPv4 = 0x0800,
    IPv6 = 0x86DD,
    ARP = 0x0806,
    Unknown(u16),
}

impl From<u16> for EtherType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EtherType::IPv4,
            0x86DD => EtherType::IPv6,
            0x0806 => EtherType::ARP,
            _ => EtherType::Unknown(value),
        }
    }
}

/// Utilitaires pour la manipulation des adresses MAC
pub fn format_mac_addr(mac: &MacAddr) -> String {
    format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", 
            mac.octets()[0], mac.octets()[1], mac.octets()[2],
            mac.octets()[3], mac.octets()[4], mac.octets()[5])
}

/// Parse une adresse MAC depuis une string
pub fn parse_mac_addr(mac_str: &str) -> Result<MacAddr, String> {
    MacAddr::parse_str(mac_str)
        .map_err(|e| format!("Erreur parsing MAC: {}", e))
}

/// Utilitaires pour les checksums
pub fn calculate_tcp_checksum_ipv4(tcp_data: &[u8], src_ip: &IpAddr, dst_ip: &IpAddr) -> u16 {
    // Créer un pseudo-header IPv4 pour le calcul du checksum TCP
    let mut pseudo_header = Vec::new();
    
    if let IpAddr::V4(src_ipv4) = src_ip {
        pseudo_header.extend_from_slice(&src_ipv4.octets());
    }
    
    if let IpAddr::V4(dst_ipv4) = dst_ip {
        pseudo_header.extend_from_slice(&dst_ipv4.octets());
    }
    
    // Protocole TCP = 6
    pseudo_header.extend_from_slice(&[0x00, 0x06]);
    
    // Longueur TCP
    let tcp_len = tcp_data.len() as u16;
    pseudo_header.extend_from_slice(&tcp_len.to_be_bytes());
    
    // Combiner pseudo-header + données TCP
    let mut checksum_data = Vec::new();
    checksum_data.extend_from_slice(&pseudo_header);
    checksum_data.extend_from_slice(tcp_data);
    
    checksum(&checksum_data, 1) // skipword = 1 pour ignorer le champ checksum existant
}

/// Vérifier si une adresse IP est privée
pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            // 10.0.0.0/8
            ipv4.octets()[0] == 10 ||
            // 172.16.0.0/12
            (ipv4.octets()[0] == 172 && ipv4.octets()[1] >= 16 && ipv4.octets()[1] <= 31) ||
            // 192.168.0.0/16
            (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168) ||
            // 127.0.0.0/8 (localhost)
            ipv4.octets()[0] == 127
        },
        IpAddr::V6(ipv6) => {
            // IPv6 localhost et private ranges
            ipv6.octets()[0] == 0xfe && (ipv6.octets()[1] & 0xc0) == 0x80 || // fe80::/10
            ipv6.octets()[0] == 0xfc || ipv6.octets()[0] == 0xfd // fc00::/7
        }
    }
}

/// Convertir des bytes en hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Extraire un port depuis des bytes (big-endian)
pub fn extract_port(bytes: &[u8]) -> u16 {
    if bytes.len() >= 2 {
        u16::from_be_bytes([bytes[0], bytes[1]])
    } else {
        0
    }
}
