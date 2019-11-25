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
use rip_audio::def::{DEFAULT_FRAMES, DEFAULT_RATE, DEFAULT_SIZE_STEREO};
use opus::{Channels, Application};

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
            let mut o_flow = o_stream.flow_output::<i16>(flow_settings).unwrap();


            let mut decoder = opus::Decoder::new(
                DEFAULT_RATE as u32,
                Channels::Stereo
            ).unwrap();

            let mut buf = [0 as u8; DEFAULT_SIZE_STEREO];
            let mut buf_float = [0 as i16; DEFAULT_SIZE_STEREO];

            o_flow.start();

            loop {
                let result = reader.read(&mut buf).await;
                if let Ok(size) = result {
                    //println!("new pocket size {}", size);

                    let readable = &buf[0..size-1];
                    let out_frames = wait_for_stream(|| o_flow.write_available(), "Write");
                    let result = decoder.decode(&readable, &mut buf_float, false);

                    match result {
                        Ok(size) => {
                            /*println!("decode size {}", buf_float.len());
                            for i in 0..buf_float.len() {
                                print!(" {}", buf_float[i]);
                            }
                            println!("");*/

                            o_flow.write(DEFAULT_FRAMES, |out| {
                                for i in 0..DEFAULT_SIZE_STEREO {
                                    out[i] = buf_float[i];
                                }
                            })
                        },
                        Err(e) => println!("error {}", e.description())
                    }
                }
            }
        },
        async move |mut writer: WriteIO| {}
    );
}
