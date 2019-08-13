#![feature(async_await, async_closure)]

use std::thread::sleep;
use std::time::Duration;
use rip_net::peer::{UdpPeer, ReadIO, WriteIO};
use rip::def::{HOST, HOST1};
use std::thread;
use futures::executor::block_on;
use std::fs::read;
use rip_audio::flow::{wait_for_stream, FlowSettings};
use rip_audio::audio_ctx::AudioContext;
use rip_audio::builder::StreamBuilder;
use rip_audio::def::{DEFAULT_FRAMES, DEFAULT_RATE};

fn main() {
    start();
}

fn start() {
    let addr = HOST.parse().unwrap();
    let to_addr = HOST1.parse().unwrap();

    let mut peer = UdpPeer::create(&addr, &to_addr);

    peer.split(
        async move |mut reader: ReadIO| {
            let context = AudioContext::new().unwrap();

            let o_stream = StreamBuilder::default_output(&context).unwrap();
            let flow_settings = FlowSettings::new(DEFAULT_FRAMES, DEFAULT_RATE);
            let mut o_flow = o_stream.flow_output::<u8>(flow_settings).unwrap();

            let mut buf = [0 as u8; 256];

            o_flow.start();

            loop {
                let result = reader.read(&mut buf).await;
                if let Ok(size) = result {
                    println!("Get frame from server {}", size);
                    let out_frames = wait_for_stream(|| o_flow.write_available(), "Write");
                    o_flow.write(256, |out| {
                        for i in 0..255 {
                            out[i] = buf[i];
                        }
                    })
                }
            }
        },
        async move |mut writer: WriteIO| {}
    );
}
