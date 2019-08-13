#![feature(async_await, async_closure)]

use std::net::{SocketAddr};
use futures::AsyncReadExt;
use std::{io, thread};
use tokio::net::UdpSocket;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use tokio::net::udp::split::{UdpSocketRecvHalf, UdpSocketSendHalf};
use tokio::future::Future;
use tokio::runtime::Runtime;
use tokio::net::udp::split::ReuniteError;
use std::sync::Arc;

/*struct UdpRwIO;

impl RwIO for UdpRwIO
    where
    RwIO::RFut : Future + Send + 'static,
    RwIO::WFut : Future + Send + 'static {


    fn read() -> RwIO::RFut {

    }

    fn write() -> RwIO::WFut {

    }
}


pub trait ReadIO {
    type RFut : Future + Send + 'static;
    fn read() -> Self::RFut;
}

pub trait WriteIO {
    type WFut : Future + Send + 'static;
    fn write() -> Self::WFut;
}

pub trait RwIO : ReadIO + WriteIO
    where Self::RFut : Future + Send + 'static,
          Self::WFut : Future + Send + 'static {

    fn read() -> Self::RFut;
    fn write() -> Self::WFut;
}*/

pub struct ReadIO {
    receiver: UdpSocketRecvHalf
}

pub struct WriteIO {
    sender: UdpSocketSendHalf
}

impl ReadIO {

    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.receiver.recv(buf).await
    }
}

impl WriteIO {

    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sender.send(buf).await
    }
}


pub struct UdpPeer {
    pub socket: Option<UdpSocket>
}

impl UdpPeer {

    pub fn create(addr: &SocketAddr, addr_to: &SocketAddr) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();
        dbg!("Binded socket to: {}", addr);
        sleep(Duration::from_secs(5));
        socket.connect(addr_to);
        dbg!("Connected to: {}", addr_to);

        return UdpPeer { socket: Some(socket) };
    }

    pub fn split<FR, FW, TR, TW>(self, cb_r: TR, cb_w: TW)
        where FR : Future + Send + 'static,
              FW : Future + Send + 'static,
              TR : Send + 'static + FnOnce(ReadIO) -> FR,
              TW : Send + 'static + FnOnce(WriteIO) -> FW {
        match self.socket {
            Some(socket) => {
                let (reader, writer) = socket.split();
                let join1 = UdpPeer::launch_read(reader, cb_r);
                let join2 = UdpPeer::launch_send(writer, cb_w);

                join1.join();
                join2.join();
            }
            None => {}
        }
    }

    fn launch_read<F, T>(reader: UdpSocketRecvHalf, f: T) -> JoinHandle<()>
        where F : Future + Send + 'static,
              T : Send + 'static + FnOnce(ReadIO) -> F {

        println!("launch_read");
        thread::spawn(move || {
            Runtime::new()
                .and_then(|rt: Runtime| {
                    rt.block_on(f(ReadIO { receiver: reader }));
                    Result::Ok(())
                });
        })
    }

    fn launch_send<F, T>(writer: UdpSocketSendHalf, f: T) -> JoinHandle<()>
        where F : Future + Send + 'static,
              T : Send + 'static + FnOnce(WriteIO) -> F {
        println!("launch_read");
        thread::spawn(move || {
            Runtime::new()
                .and_then(|rt: Runtime| {
                    rt.block_on(f(WriteIO { sender: writer }));
                    Result::Ok(())
                });
        })
    }
}
