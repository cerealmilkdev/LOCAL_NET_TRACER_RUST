use chrono::{DateTime, Utc};
use pnet::packet::{ethernet::{EtherType, MappedArp}, ip::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket};
use pnet::packet::Packet;

pub struct Packet {
    pub timestamp: DateTime,
    pub lenght: usize,
    pub data: Vec<u8>,
    pub ethernet: Option<EthernetPacket>,
    pub ip: Option<IpPacket>,
    pub transport: Option<TransportPacket>,
}

/// Network header

pub struct EthernetHeader {
    pub dst_mac: MacAddr,
    pub src_mac: MacAddr,
    pub ether_type: EtherType,
}

pub struct IpHeader {
    pub version: u8,
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub protocol: u8,
}

/// Packets parsing 
pub fn parse_ethernet_frame(data: &[u8]) -> Result<EthernetHeader, ParseError> {
    // Vérifier la taille minimale
    if data.len() < ETHERNET_HEADER_LEN {
        return Err(ParseError::InvalidLength);
    }
    
    // Parser les adresses MAC (6 bytes chacune)
    let dst_mac = MacAddr::new(data[0], data[1], data[2], data[3], data[4], data[5]);
    let src_mac = MacAddr::new(data[6], data[7], data[8], data[9], data[10], data[11]);
    
    // Parser le type Ethernet (2 bytes)
    let ether_type = u16::from_be_bytes([data[12], data[13]]).into();
    
    Ok(EthernetHeader {
        dst_mac,
        src_mac,
        ether_type,
    })
}

pub fn parse_ip_packet(data: &[u8]) -> Result<IpHeader, ParseError> {
    // Vérifier la taille minimale
    if data.len() < IP_HEADER_LEN {
        return Err(ParseError::InvalidLength);
    }
    
    // Parser le version (1 byte)
    let version = data[0] >> 4;
    
    // Parser l'adresse IP source (4 bytes)
    let src_ip = IpAddr::V4(Ipv4Addr::new(data[12], data[13], data[14], data[15]));
    
}

/// type error

pub enum ParseError {
    InvalidLength,
    UnsupportedProtocol,
    ChecksumMismatch,
}
