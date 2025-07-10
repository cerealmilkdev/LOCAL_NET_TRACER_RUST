pub struct WifiHeader {
    pub frame_control: u16,
    pub duration: u16,
    pub addr1: [u8; 6],
    pub addr2: [u8; 6],
    pub addr3: [u8; 6],
    pub sequence: u16,   
}

pub fn parse_wifi_frame(data: &[u8]) -> Result<WifiHeader, ParseError> {
    let mut reader = Reader::new(data);
    let frame_control = reader.read_u16::<BigEndian>()?;
    let duration = reader.read_u16::<BigEndian>()?;
    let addr1 = reader.read_array::<6>()?;
    let addr2 = reader.read_array::<6>()?;
    let addr3 = reader.read_array::<6>()?;
    let sequence = reader.read_u16::<BigEndian>()?;
}