use std::net::TcpStream;
use std::io::Read;
use super::codec;

pub fn read_status(stream: &mut TcpStream) {
    loop {
        let (length, id) = super::read_header(stream);
        if id == 0 {
            send_status(stream);
        } else if id == 1 {
            send_ping(stream);
        }
    }
}

fn send_ping(stream: &mut TcpStream) {
    let mut payload = [0; 8];
    stream.read(&mut payload).unwrap();
    super::send_packet(stream, 1, &payload);
}

fn send_status(stream: &mut TcpStream) {
    let response = codec::encode_string("{
    \"version\": {
        \"name\": \"1.13.2\",
        \"protocol\": 404
    },
    \"players\": {
        \"max\": 100,
        \"online\": 5,
        \"sample\": [
            {
                \"name\": \"Cobol72\",
                \"id\": \"f2b92aaf-40cc-4272-ad73-179f1b624658\"
            }
        ]
    },
    \"description\": {
        \"text\": \"Hello world\"
    }
}");

    super::send_packet(stream, 0, &response);
}