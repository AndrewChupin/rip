use crate::stream::AudioStream;
use portaudio::Time;
use std::io::ErrorKind;
use rip_core::builder::Builder;
use crate::def::{DEFAULT_FRAMES, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, ADeviceId};
use crate::audio_ctx::{AudioContext, FlowType};


pub struct ABuilder<'a> {
    pub(crate) context: &'a AudioContext,
    pub(crate) device: Option<ADeviceId>,
    pub(crate) channel_count: Option<i32>,
    pub(crate) interleaved: Option<bool>,
    pub(crate) latency: Option<Time>,
    pub(crate) stream_type: FlowType
}


impl <'a> ABuilder<'a> {

    pub fn builder(ctx: &AudioContext, a_type: FlowType) -> ABuilder {
        return ABuilder {
            context: ctx,
            device: None,
            channel_count: None,
            interleaved: None,
            latency: None,
            stream_type: a_type
        };
    }

    pub fn default(ctx: &AudioContext, a_type: FlowType) -> Result<AudioStream, ErrorKind> {
        let audio = &ctx.core;

        let device = ctx
            .default_device(a_type)
            .unwrap();

        let stream = AudioStream {
            context: ctx,
            device: device.id,
            channel_count: DEFAULT_CHANNELS_COUNT,
            interleaved: DEFAULT_INTERLEAVED,
            latency: device.info.default_low_output_latency,
            stream_type: a_type
        };

        Ok(stream)
    }

    pub fn device<'r>(&'r mut self, device: ADeviceId) -> &'r mut ABuilder<'a> {
        self.device = Some(device);
        self
    }

    pub fn channel_count<'r>(&'r mut self, channel_count: i32) -> &'r mut ABuilder<'a> {
        self.channel_count = Some(channel_count);
        self
    }

    pub fn interleaved<'r>(&'r mut self, interleaved: bool) -> &'r mut ABuilder<'a> {
        self.interleaved = Some(interleaved);
        self
    }

    pub fn latency<'r>(&'r mut self, latency: Time) -> &'r mut ABuilder<'a> {
        self.latency = Some(latency);
        self
    }
}


impl <'a> Builder<AudioStream<'a>, ErrorKind> for ABuilder<'a>  {

    fn build(&self) -> Result<AudioStream<'a>, ErrorKind> {
        let context = self.context;
        let audio = &context.core;

        let device = match self.device {
            Some(device) => device,
            None => match audio.default_input_device() {
                Ok(device) => device,
                Err(_) => return Err(ErrorKind::NotFound)
            }
        };

        let channel_count = match self.channel_count {
            Some(channel_count) => channel_count,
            None => DEFAULT_CHANNELS_COUNT
        };

        let interleaved = match self.interleaved {
            Some(interleaved) => interleaved,
            None => DEFAULT_INTERLEAVED
        };

        let latency = match self.latency {
            Some(latency) => latency,
            None => match audio.device_info(device) {
                Ok(info) => info.default_low_input_latency,
                Err(_) => return Err(ErrorKind::NotFound)
            }
        };

        let stream_type = self.stream_type;

        Ok(AudioStream { context , device, channel_count, interleaved, latency, stream_type })
    }
}
