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
use rip_audio::def::{DEFAULT_FRAMES, DEFAULT_RATE, DEFAULT_CHANNELS_COUNT, DEFAULT_SIZE_STEREO};
use std::collections::VecDeque;
use opus::{Channels, Application};

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
            let mut i_flow = i_stream.flow_input::<i16>(flow_settings).unwrap();

            let mut opus = opus::Encoder::new(
                DEFAULT_RATE as u32,
                Channels::Stereo,
                Application::Voip
            ).unwrap();

            /*let mut decoder = opus::Decoder::new(
                DEFAULT_RATE as u32,
                Channels::Stereo
            ).unwrap();
            let mut buf_float = [0 as i16; DEFAULT_SIZE_STEREO];*/

            let mut buffer= [0 as u8; DEFAULT_SIZE_STEREO];
            i_flow.start();

            loop {
                let in_frames = wait_for_stream(|| i_flow.read_available(), "Read");
                if in_frames > 0 {
                    let mut buf = i_flow.read(DEFAULT_FRAMES);
/*
                    println!("original size {}", buf.len());
                    for i in 0..buf.len() {
                        print!(" {}", buf[i]);
                    }
                    println!("");*/


                    match opus.encode(buf, &mut buffer) {
                        Ok(size) => {
                            /*println!("encode size {}", size);
                            for i in 0..buffer.len() {
                                print!(" {}", buffer[i]);
                            }
                            println!("");*/

                            let writable = &mut buffer[0..size-1];

                            /*let result = decoder.decode(
                                &writable,
                                &mut buf_float,
                                false
                            );

                            if let Ok(size) = result {
                                println!("decode size {}", size);
                                for i in 0..buf_float.len() {
                                    print!(" {}", buf_float[i]);
                                }
                                println!("");
                            }*/

                            writer.write(writable).await;
                        },
                        Err(e) => println!("error {}", e.description())
                    };
                }
            }
        }
    );
}

