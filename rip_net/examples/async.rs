#![feature(async_await)]

use futures::{SinkExt, Stream};
use std::{env, error::Error, net::SocketAddr};
use tokio::{
    codec::{FramedRead, FramedWrite},
    io,
    sync::{mpsc, oneshot},
};
use rip_net::def::{HOST1, HOST};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        run().await.unwrap();
        tx.send(()).unwrap();
    });

    rx.await.map_err(Into::into)
}

// Currently, we need to spawn the initial future due to https://github.com/tokio-rs/tokio/issues/1356
async fn run() -> Result<(), Box<dyn Error>> {
    // Determine if we're going to run in TCP or UDP mode
    let tcp = false;

    // Parse what address we're going to connect to
    let addr = HOST.parse::<SocketAddr>()?;

    let stdin = stdin();
    let stdout = FramedWrite::new(io::stdout(), codec::Bytes);

    udp::connect(&addr).await?;

    Ok(())
}

// Temporary work around for stdin blocking the stream
fn stdin() -> impl Stream<Item = Result<Vec<u8>, io::Error>> + Unpin {
    let mut stdin = FramedRead::new(io::stdin(), codec::Bytes);

    let (mut tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        tx.send_all(&mut stdin).await.unwrap();
    });

    rx
}

mod tcp {
    use super::codec;
    use futures::{future, Sink, SinkExt, Stream, StreamExt};
    use std::{error::Error, io, net::SocketAddr};
    use tokio::{
        codec::{FramedRead, FramedWrite},
        net::TcpStream,
    };

    pub async fn connect(
        addr: &SocketAddr,
        stdin: impl Stream<Item = Result<Vec<u8>, io::Error>> + Unpin,
        mut stdout: impl Sink<Vec<u8>, Error = io::Error> + Unpin,
    ) -> Result<(), Box<dyn Error>> {
        let (r, w) = TcpStream::connect(addr).await?.split();
        let sink = FramedWrite::new(w, codec::Bytes);
        let mut stream = FramedRead::new(r, codec::Bytes).filter_map(|i| match i {
            Ok(i) => future::ready(Some(i)),
            Err(e) => {
                println!("failed to read from socket; error={}", e);
                future::ready(None)
            }
        });

        match future::join(stdin.forward(sink), stdout.send_all(&mut stream)).await {
            (Err(e), _) | (_, Err(e)) => Err(e.into()),
            _ => Ok(()),
        }
    }
}

mod udp {
    use futures::{future, Sink, SinkExt, Stream, StreamExt};
    use std::{error::Error, io, net::SocketAddr};
    use tokio::net::udp::{
        split::{UdpSocketRecvHalf, UdpSocketSendHalf},
        UdpSocket,
    };

    pub async fn connect(
        addr: &SocketAddr
    ) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(&addr)?;
        socket.connect(addr)?;
        let (mut r, mut w) = socket.split();
        recv( &mut r).await?;
        Ok(())
    }

    async fn send(
        mut stdin: impl Stream<Item = Result<Vec<u8>, io::Error>> + Unpin,
        writer: &mut UdpSocketSendHalf,
    ) -> Result<(), io::Error> {
        while let Some(item) = stdin.next().await {
            let buf = item?;
            writer.send(&buf[..]).await?;
        }

        Ok(())
    }

    async fn recv(
        reader: &mut UdpSocketRecvHalf,
    ) -> Result<(), io::Error> {
        loop {
            let mut buf = vec![0; 1024];
            let n = reader.recv(&mut buf[..]).await?;
        }
    }
}

mod codec {

}
