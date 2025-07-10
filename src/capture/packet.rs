pub struct Packet {
    pub timestamp: DateTime,
    pub lenght: usize,
    pub data: Vec<8>,
    pub ethernet: Option<EthernetPacket>,
    pub ip: Option<IpPacket>,
    pub transport: Option<TransportPacket>,
}