use crate::obelisk::Obelisk;
use bytes::{BytesMut, BufMut};
use tokio::codec::{Decoder, Encoder, Framed};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Error, ErrorKind};
use tokio::prelude::*;
use tokio::prelude::AsyncSink::{Ready, NotReady};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;

pub mod codec;
mod login;
mod play;
mod status;

pub struct Packet {
    id: i32,
    data: Vec<u8>
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Packet {
        Packet {
            id,
            data
        }
    }
}

pub fn start(server: Arc<RwLock<Obelisk>>) {
    let address = SocketAddr::new("127.0.0.1".parse().unwrap(), 25565);
    let listener = TcpListener::bind(&address).expect("Unable to bind TcpListener");

    let network_loop = listener.incoming()
        .for_each(move |socket| {
            let framed = Framed::new(socket, PacketCodec::new());
            tokio::spawn(PlayerSocket {
                server: server.clone(),
                stream: framed,
                state: NetState::Handshake,
                output: VecDeque::new()
            }.map_err(|e| println!("connection error: {:?}", e)));

            Ok(())
        }).map_err(|e| println!("accept error: {:?}", e));

    tokio::run(network_loop);
}

struct PacketCodec;

impl PacketCodec {
    fn new() -> PacketCodec {
        PacketCodec {}
    }
}

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

        let len = codec::read_varint(&mut src.split_to(varint_size).to_vec())?;
        let mut bytes = src.split_to(len as usize).to_vec();
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
enum NetState {
    Handshake,
    Status,
    Login,
    Play
}

pub struct PlayerSocket {
    server: Arc<RwLock<Obelisk>>,
    stream: Framed<TcpStream, PacketCodec>,
    state: NetState,
    output: VecDeque<Packet>
}

impl Future for PlayerSocket {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        // Check if a packet is available to read
        let mut none = false;
        loop {
            match self.stream.poll()? {
                Async::Ready(Some(mut packet)) => {
                    match self.state {
                        NetState::Handshake => {
                            self.read_handshake(&mut packet)?;
                        }
                        ,
                        NetState::Status => {
                            status::read_status(self, &packet);
                        },
                        NetState::Login => {

                        },
                        NetState::Play => {

                        }
                    };
                },
                Async::Ready(None) => {
                    none = true;
                    break
                },
                Async::NotReady => break
            }
        }


        while let Some(packet) = self.output.pop_front() {
            match self.stream.start_send(packet)? {
                NotReady(packet) => {
                    self.output.push_front(packet);
                    break;
                },
                Ready => ()
            }
        }

        futures::try_ready!(self.stream.poll_complete());

        if none {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl PlayerSocket {
    pub fn send_packet(&mut self, id: i32, data: Vec<u8>) {
        self.output.push_back(Packet::new(id, data));
    }

    fn read_handshake(&mut self, packet: &mut Packet) -> Result<(), Error> {
        let _version = codec::read_varint(&mut packet.data)?;
        let _address = codec::read_string(&mut packet.data)?;
        let _port = codec::read_ushort(&mut packet.data);
        let state = codec::read_varint(&mut packet.data)?;

        if state == 1 {
            self.state = NetState::Status;
        } else if state == 2 {
            self.state = NetState::Login;
        } else {
            unimplemented!()
        }

        Ok(())
    }
}