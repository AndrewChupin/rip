
mod def;
pub mod stream;
pub mod builder;
pub mod audio_ctx;

extern crate portaudio;
extern crate rip_core;

use portaudio as pa;
use portaudio::{DuplexStreamSettings, StreamParameters};
use crate::builder::ABuilder;
use crate::audio_ctx::{AudioContext, FlowType};
use rip_core::builder::Builder;
use crate::def::{DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, DEFAULT_RATE, DEFAULT_FRAMES};
use std::borrow::Borrow;


pub fn run() {

    let port_a = pa::PortAudio::new()
        .unwrap();

    let context = AudioContext::new()
        .unwrap();

    let input_device = context
        .default_device(FlowType::Input)
        .unwrap();

    let input_stream_def = ABuilder::default(&context, FlowType::Input)
        .unwrap();

    let input_stream = ABuilder::builder(&context, FlowType::Input)
        .device(input_device.id)
        .latency(0.)
        .interleaved(DEFAULT_INTERLEAVED)
        .channel_count(DEFAULT_CHANNELS_COUNT)
        .build()
        .unwrap();

    let input_device = port_a.default_input_device().unwrap();
    let input_info = port_a.device_info(input_device).unwrap();

    let output_device = port_a.default_output_device().unwrap();
    let output_info = port_a.device_info(output_device).unwrap();

    let in_params = StreamParameters::<u8>::new(
        input_device,
        DEFAULT_CHANNELS_COUNT,
        DEFAULT_INTERLEAVED,
        input_info.default_low_input_latency
    );

    let out_params = StreamParameters::new(
        output_device,
        DEFAULT_CHANNELS_COUNT,
        DEFAULT_INTERLEAVED,
        output_info.default_low_output_latency
    );

    let settings = DuplexStreamSettings::new(
        in_params,
        out_params,
        DEFAULT_RATE,
        DEFAULT_FRAMES
    );

    let mut count_down = 25.0;
    let mut maybe_last_time = None;

    let callback = move |pa::DuplexStreamCallbackArgs {
                             in_buffer,
                             out_buffer,
                             frames,
                             time,
                             ..
                         }| {
        let current_time = time.current;
        let prev_time = maybe_last_time.unwrap_or(current_time);
        let dt = current_time - prev_time;
        count_down -= dt;
        maybe_last_time = Some(current_time);

        assert_eq!(frames, DEFAULT_FRAMES as usize);

        // Pass the input straight to the output - BEWARE OF FEEDBACK!
        for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *output_sample = *input_sample;
        }

        if count_down > 0.0 {
            pa::Continue
        } else {
            pa::Complete
        }
    };

    let mut stream =  port_a.open_non_blocking_stream(settings, callback).unwrap();

    stream.start().unwrap();

    while let Ok(true) = stream.is_active() {}

    stream.stop().unwrap();
}