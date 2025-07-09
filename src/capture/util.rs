use pnet::util::{MacAddr, checksum, ipv4_checksum, ipv6_checksum};
use std::net::IpAddr;

/// Constantes réseau de base
pub const ETHERNET_HEADER_LEN: usize = 14;
pub const IPV4_HEADER_LEN: usize = 20;

/// Types de protocoles Ethernet
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

/// Formatage d'adresse MAC (utilise MacAddr de pnet::util)
pub fn format_mac_addr(mac: &MacAddr) -> String {
    format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", 
            mac.octets()[0], mac.octets()[1], mac.octets()[2],
            mac.octets()[3], mac.octets()[4], mac.octets()[5])
}

/// Represents an error which occured whilst parsing a MAC address
pub enum ParseMacAddrErr {
    TooManyComponents,
    TooFewComponents,
    InvalidComponent,
}

/// Parsing d'adresse MAC
pub fn parse_mac_addr(mac_str: &str) -> Result<MacAddr, ParseMacAddrErr> {
    MacAddr::parse_str(mac_str)
        .map_err(|e| match e {
            pnet::util::ParseMacAddrErr::TooManyComponents => ParseMacAddrErr::TooManyComponents,
            pnet::util::ParseMacAddrErr::TooFewComponents => ParseMacAddrErr::TooFewComponents,
            pnet::util::ParseMacAddrErr::InvalidComponent => ParseMacAddrErr::InvalidComponent,
        })
}

/// Calcul de checksum simple (utilise checksum de pnet::util)
pub fn calculate_checksum(data: &[u8]) -> u16 {
    checksum(data, 0) // pas de skipword
}

/// Conversion bytes vers hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Extraction de port depuis bytes (big-endian)
pub fn extract_port(bytes: &[u8]) -> u16 {
    if bytes.len() >= 2 {
        u16::from_be_bytes([bytes[0], bytes[1]])
    } else {
        0
    }
}
