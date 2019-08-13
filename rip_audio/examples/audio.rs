use rip_audio::audio_ctx::{AudioContext, AudioType};
use rip_audio::builder::StreamBuilder;
use rip_core::builder::Builder;
use portaudio::{StreamParameters, DuplexStreamSettings, PortAudio, Error, Stream, StreamAvailable};
use std::borrow::Borrow;
use rip_audio::def::{DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, DEFAULT_RATE, DEFAULT_FRAMES};
use std::collections::VecDeque;

use portaudio as pa;
use std::iter::Zip;
use rip_audio::error::AudioError;
use rip_audio::flow::{FlowSettings, wait_for_stream};

fn main() {
   // run_example().unwrap();
}

// Zip I -> O O -> I


fn run_example() -> Result<(), AudioError> {

    let context = AudioContext::new()?;

    let i_device = context.default_device(AudioType::Input)?;
    let o_device = context.default_device(AudioType::Output)?;

    let i_stream1 = StreamBuilder::default_input(&context)?;
    let o_stream1 = StreamBuilder::default_output(&context)?;

    let flow_settings = FlowSettings::new(DEFAULT_FRAMES, DEFAULT_RATE);

    let i_flow = i_stream1.flow_input::<u8>(flow_settings)?;
    let o_flow = o_stream1.flow_output::<u8>(flow_settings)?;

    let mut buffer: VecDeque<u8> = VecDeque::with_capacity(
        flow_settings.frame as usize * DEFAULT_CHANNELS_COUNT as usize
    );

    let mut i_stream = i_flow.source;
    let mut o_stream = o_flow.source;

    i_stream.start();
    o_stream.start();


    // Now start the main read/write loop! In this example, we pass the input buffer directly to
    // the output buffer, so watch out for feedback.
    'stream: loop {

        // How many frames are available on the input stream?
        let in_frames = wait_for_stream(|| i_stream.read_available(), "Read");

        // If there are frames available, let's take them and add them to our buffer.
        if in_frames > 0 {
            let input_samples = i_stream.read(in_frames).unwrap();
            buffer.extend(input_samples.into_iter());
            println!("Read {:?} frames from the input stream.", in_frames);
        }

        // How many frames are available for writing on the output stream?
        let out_frames = wait_for_stream(|| o_stream.write_available(), "Write");

        // How many frames do we have so far?
        let buffer_frames = (buffer.len() / DEFAULT_CHANNELS_COUNT as usize) as u32;

        // If there are frames available for writing and we have some to write, then write!
        if out_frames > 0 && buffer_frames > 0 {

            // If we have more than enough frames for writing, take them from the start of the buffer.
            // Otherwise if we have less, just take what we can for now.
            let write_frames = if buffer_frames >= out_frames { out_frames } else { buffer_frames };
            let n_write_samples = write_frames as usize * DEFAULT_CHANNELS_COUNT as usize;

            o_stream.write(write_frames, |output| {
                for i in 0..n_write_samples {
                    output[i] = buffer.pop_front().unwrap();
                }
                println!("Wrote {:?} frames to the output stream.", out_frames);
            }).unwrap();
        }
    }

    println!("end");

    Ok(())
}
