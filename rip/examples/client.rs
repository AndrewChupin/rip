#![feature(async_await, async_closure)]

use std::thread::sleep;
use std::time::Duration;
use rip_net::peer::{UdpPeer, ReadIO, WriteIO};
use rip::def::{HOST, HOST1};
use std::thread;
use futures::executor::block_on;
use std::fs::read;
use rip_audio::audio_ctx::AudioContext;
use rip_audio::builder::StreamBuilder;
use rip_audio::flow::{FlowSettings, wait_for_stream};
use rip_audio::def::{DEFAULT_FRAMES, DEFAULT_RATE, DEFAULT_CHANNELS_COUNT};
use std::collections::VecDeque;

fn main() {
    start();
}


fn start() {
    // Network
    let addr = HOST1.parse().unwrap();
    let to_addr = HOST.parse().unwrap();
    let mut peer = UdpPeer::create(&addr, &to_addr);

    peer.split(
        async move |mut reader: ReadIO| {
            println!("start reading");
        },
        async move |mut writer: WriteIO| {
            let context = AudioContext::new().unwrap();

            let i_stream = StreamBuilder::default_input(&context).unwrap();
            dbg!({}, i_stream.settings);
            let flow_settings = FlowSettings::new(DEFAULT_FRAMES, DEFAULT_RATE);
            let mut i_flow = i_stream.flow_input::<u8>(flow_settings).unwrap();

            let mut buffer= [0 as u8; 256];
            i_flow.start();

            loop {
                println!("send to server");
                unsafe {
                    let in_frames = wait_for_stream(|| i_flow.read_available(), "Read");
                    if in_frames > 0 {
                        let mut buff = i_flow.read(in_frames);
                        println!("Read {:?} frames from the input stream.", in_frames);
                        writer.write(&mut buff).await;
                    }
                }
            }
        }
    );
}

