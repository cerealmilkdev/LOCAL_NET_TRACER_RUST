pub struct Packet {
    pub timestamp: DateTime,
    pub lenght: usize,
    pub data: Vec<8>,
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
    pub dist_ip: IpAddr,
    pub protocol: u8,
}

/// Packets parsing 

pub fn parse_ethernet_frame(data: &[u8]) -> Result<EthernetHeader, ParseError>
pub fn parse_ip_packet(data: &[u8]) -> Result<IpHeader, ParseError>

/// type error

pub enum ParseError {
    InvalidLength,
    UnsupportedProtocol,
    ChecksumMismatch,
}