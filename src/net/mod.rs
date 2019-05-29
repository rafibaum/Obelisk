use crate::obelisk::Obelisk;
use crate::entities::player;
use bytes::{BytesMut, BufMut};
use futures::future::FutureResult;
use mio::Ready;
use tokio::codec::{Decoder, Encoder};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Error, ErrorKind};
use tokio::prelude::*;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::mem;
use std::convert::TryInto;

pub mod codec;
mod login;
mod play;
mod status;

struct Packet {
    id: i32,
    data: Vec<u8>
}

/*pub fn start(server: Arc<RwLock<Obelisk>>) {
    let address = SocketAddr::new("127.0.0.1".parse().unwrap(), 25565);
    let listener = TcpListener::bind(&address).expect("Unable to bind TcpListener");

    let network_loop = listener.incoming()
        .for_each(move |socket| {
            tokio::spawn(process(socket, NetState::Handshake).then(|_| {
                Ok(())
            }));

            Ok(())
        }).map_err(|e| println!("{:?}", e));

    tokio::run(network_loop);
}

enum NetState {
    Handshake,
    Status,
    Login,
    Play
}

fn process(stream: TcpStream, state: NetState) -> impl Future<Item = (), Error = Error> {
    match state {
        NetState::Handshake => {
            read_handshake(stream)
        },
        NetState::Status => {
            unimplemented!()
        },
        NetState::Login => {
            unimplemented!()
        },
        NetState::Play => {
            unimplemented!()
        }
    }
}


fn read_handshake(stream: TcpStream) -> impl Future<Item = NetState, Error = Error> {
    read_packet(stream).and_then(|(stream, mut packet)| {
        let _version = codec::read_varint(&mut packet.data)?;
        let _address = codec::read_string(&mut packet.data)?;
        let _port = codec::read_ushort(&mut packet.data);
        let state = codec::read_varint(&mut packet.data)?;

        if state == 1 {
            Ok(NetState::Status)
        } else if state == 2 {
            Ok(NetState::Login)
        } else {
            unimplemented!()
        }
    })
}


fn send_packet(mut stream: TcpStream, id: i32, mut data: Vec<u8>) -> impl Future<Item = TcpStream, Error = Error> {
    let mut packet = Vec::new();
    let id = codec::encode_varint(id);
    let length = codec::encode_varint((id.len() + data.len()) as i32);
    packet.extend_from_slice(&length);
    packet.extend_from_slice(&id);
    packet.append(&mut data);
    tokio::io::write_all(stream, packet).map(|(stream, packet)| { stream })
}

fn read_packet(mut socket: TcpStream) -> impl Future<Item = (TcpStream, Packet), Error = Error> {
    read_length(socket).and_then(|(socket, len)| {
        let data = Vec::with_capacity(len as usize);
        tokio::io::read_exact(socket, data)
    }).map(|(socket, mut data)| {
        let id = match codec::read_varint(&mut data) {
            Ok(num) => num,
            Err(e) => return futures::future::err(e)
        };

        let packet = Packet {
            id,
            data
        };

        futures::future::ok((socket, packet))
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
}*/

struct PacketCodec;

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut varint_size = 0;
        let mut finished = false;
        for b in src.iter() {
            varint_size += 1;
            if b & 0b10000000 == 0 {
                finished = true;
                break;
            }

            if varint_size == 5 {
                return Err(Error::new(ErrorKind::InvalidData, "Varint too long!"));
            }
        }

        if !finished {
            return Ok(None);
        }

        let mut bytes = src.to_vec();
        let len = codec::read_varint(&mut bytes)?;
        let bytes_missing = len - bytes.len() as i32;
        if bytes_missing == 0 {
            let id = codec::read_varint(&mut bytes)?;
            Ok(Some(Packet {
                id,
                data: bytes
            }))
        } else if bytes_missing > 0 {
            Ok(None)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Data longer than packet length"))
        }
    }
}

impl Encoder for PacketCodec {
    type Item = Packet;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let id = codec::encode_varint(item.id);
        let data = item.data;
        let length = id.len() + data.len();
        dst.reserve(length);

        let length = codec::encode_varint(length as i32);
        dst.put_slice(&length);
        dst.put_slice(&id);
        dst.put_slice(&data);

        Ok(())
    }
}