use crate::obelisk::Obelisk;
use crate::entities::player;
use mio::Ready;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Error, ErrorKind};
use tokio::prelude::*;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::mem;

pub mod codec;
mod login;
mod play;
mod status;

pub struct Header {
    length: i32,
    id: i32
}

pub fn start(server: Arc<RwLock<Obelisk>>) {
    let address = SocketAddr::new("127.0.0.1".parse().unwrap(), 25565);
    let listener = TcpListener::bind(&address).expect("Unable to bind TcpListener");

    let network_loop = listener.incoming()
        .for_each(move |socket| {
            tokio::spawn(process(socket).then(|_| {
                Ok(())
            }));

            Ok(())
        }).map_err(|e| println!("{:?}", e));

    tokio::run(network_loop);
}

fn process(stream: TcpStream) -> impl Future<Item = (), Error = Error> {
    read_packet(stream)
}

/*
fn read_handshake(server: &Obelisk, stream: TcpStream) -> Result<Option<player::Player>, Error> {
    let _header = read_header(stream)?;
    let _version = codec::read_varint(stream)?;
    let _address = codec::read_string(stream)?;
    let _port = codec::read_ushort(stream)?;
    let state = codec::read_varint(stream)?;
    if state == 1 {
        status::read_status(server, stream)?;
        return Ok(None)
    } else if state == 2 {
        return Ok(Some(login::handle_login(stream)?));
    } else {
        return Err(Error::new(ErrorKind::InvalidData, "Handshake had invalid state"));
    }
}

fn send_packet(stream: &mut TcpStream, id: i32, data: &[u8]) -> Result<(), Error> {
    let mut packet = Vec::new();
    let id = codec::encode_varint(id);
    let length = codec::encode_varint((id.len() + data.len()) as i32);
    packet.extend_from_slice(&length);
    packet.extend_from_slice(&id);
    packet.extend_from_slice(data);
    stream.write(&packet)?;
    Ok(stream.flush()?)
}

fn read_header(stream: &TcpStream) -> Result<Header, Error> {
    let length = codec::read_varint(stream)?;
    let (id, id_size) = codec::read_varint_size(stream)?;

    Ok(Header {
        length: length - id_size,
        id
    })
}*/

fn read_packet(mut socket: TcpStream) -> impl Future<Item = (), Error = Error> {
    read_length(socket)
        .map(|(socket, len)| {
            println!("Received length: {}", len);
    })
}

fn read_length(mut socket: TcpStream) -> impl Future<Item = (TcpStream, i32), Error = Error> {
    FutureLength {
        socket: Some(socket)
    }
}

struct FutureLength {
    socket: Option<TcpStream>
}

impl Future for FutureLength {
    type Item = (TcpStream, i32);
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match &mut self.socket {
            None => panic!("Used FutureLength twice!"),
            Some(ref mut stream) => {
                let mut buf = vec![0; 5];
                futures::try_ready!(stream.poll_peek(&mut buf));

                let mut varint_size = 0;
                for b in buf {
                    varint_size += 1;
                    if b & 0b10000000 == 0 {
                        break;
                    }

                    if varint_size == 5 {
                        panic!("")
                    }
                }

                let mut num_bytes = vec![0; varint_size];
                futures::try_ready!(stream.poll_read(&mut num_bytes));
                println!("{:?}", num_bytes);

                let num = codec::read_varint(&mut num_bytes)?;
                let socket = mem::replace(&mut self.socket, None).unwrap();
                Ok(Async::Ready((socket, num)))
            }
        }
    }
}