#![feature(async_await, async_closure)]

use std::thread::sleep;
use std::time::Duration;
use rip_net::peer::{UdpPeer, ReadIO, WriteIO};
use std::thread;
use futures::executor::block_on;
use std::fs::read;

static HOST: &str = "192.168.0.240:5000";
static HOST1: &str = "192.168.0.240:5001";

fn main() {
    start();
}


fn start() {
    let addr = HOST.parse().unwrap();
    let to_addr = HOST1.parse().unwrap();

    let mut peer = UdpPeer::create(&addr, &to_addr);

    peer.split(
        async move |mut reader: ReadIO| {
            loop {
                println!("start reading");
                let mut buf = [0; 24];
                let result = reader.read(&mut buf).await;

                match result {
                    Ok(res) => println!("server read success {}", String::from_utf8(buf.to_vec()).unwrap()),
                    Err(err) => println!("server read error {}", err)
                }
                sleep(Duration::from_secs(2));
            }
        },
        async move |mut writer: WriteIO| {
            loop {
                println!("send to server");
                writer.write(b"Hello from server").await;
                sleep(Duration::from_secs(2));
            }
        }
    );
}
