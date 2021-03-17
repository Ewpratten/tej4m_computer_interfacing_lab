pub enum PacketType {
    Request = 0x01,
    Response = 0x02,
}

pub enum RequestType {
    RandomNumber = 0x01,
}

pub fn construct_request_packet(req_type: RequestType) -> [u8; 3] {
    return [PacketType::Request as u8, 1, req_type as u8];
}

pub fn construct_response_packet(value: u8) -> [u8; 3] {
    return [PacketType::Response as u8, 1, value];
}
