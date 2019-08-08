use crate::stream::{AudioStream, AudioSettings};
use portaudio::Time;
use std::io::ErrorKind;
use rip_core::builder::Builder;
use crate::def::{DEFAULT_FRAMES, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, ADeviceId};
use crate::audio_ctx::{AudioContext, AudioType};
use crate::error::AudioError;


pub struct StreamBuilder<'a> {
    pub(crate) context: &'a AudioContext,
    pub(crate) device: Option<ADeviceId>,
    pub(crate) channel_count: Option<i32>,
    pub(crate) is_interleaved: Option<bool>,
    pub(crate) latency: Option<Time>,
    pub(crate) stream_type: AudioType
}


impl <'a> StreamBuilder<'a> {

    pub fn builder_output(ctx: &AudioContext) -> StreamBuilder {
        return StreamBuilder {
            context: ctx,
            device: None,
            channel_count: None,
            is_interleaved: None,
            latency: None,
            stream_type: AudioType::Output
        };
    }

    pub fn builder_input(ctx: &AudioContext) -> StreamBuilder {
        return StreamBuilder {
            context: ctx,
            device: None,
            channel_count: None,
            is_interleaved: None,
            latency: None,
            stream_type: AudioType::Input
        };
    }

    pub fn default_input(ctx: &AudioContext) -> Result<AudioStream, AudioError> {
        let audio = &ctx.core;

        let result = ctx
            .default_device(AudioType::Input);

        let device = match result {
            Ok(device) => device,
            Err(_) => return Err(AudioError::Device)
        };

        let stream = AudioStream {
            context: ctx,
            settings: AudioSettings {
                device: device.id,
                channels: DEFAULT_CHANNELS_COUNT,
                is_interleaved: DEFAULT_INTERLEAVED,
                latency: device.info.default_low_input_latency,
                stream_type: AudioType::Input
            }
        };

        Ok(stream)
    }

    pub fn default_output(ctx: &AudioContext) -> Result<AudioStream, AudioError> {
        let audio = &ctx.core;

        let result = ctx
            .default_device(AudioType::Output);

        let device = match result {
            Ok(device) => device,
            Err(_) => return Err(AudioError::Device)
        };

        let stream = AudioStream {
            context: ctx,
            settings: AudioSettings {
                device: device.id,
                channels: DEFAULT_CHANNELS_COUNT,
                is_interleaved: DEFAULT_INTERLEAVED,
                latency: device.info.default_low_output_latency,
                stream_type: AudioType::Output
            }
        };

        Ok(stream)
    }

    pub fn device<'r>(&'r mut self, device: ADeviceId) -> &'r mut StreamBuilder<'a> {
        self.device = Some(device);
        self
    }

    pub fn channel_count<'r>(&'r mut self, channel_count: i32) -> &'r mut StreamBuilder<'a> {
        self.channel_count = Some(channel_count);
        self
    }

    pub fn interleaved<'r>(&'r mut self, is_interleaved: bool) -> &'r mut StreamBuilder<'a> {
        self.is_interleaved = Some(is_interleaved);
        self
    }

    pub fn latency<'r>(&'r mut self, latency: Time) -> &'r mut StreamBuilder<'a> {
        self.latency = Some(latency);
        self
    }
}


impl <'a> Builder<AudioStream<'a>, AudioError> for StreamBuilder<'a>  {

    fn build(&self) -> Result<AudioStream<'a>, AudioError> {
        let ctx = self.context;
        let audio = &ctx.core;
        let stream_type = self.stream_type;

        let device = match self.device {
            Some(device) => device,
            None => match ctx.default_device(stream_type) {
                Ok(device) => device.id,
                Err(_) => return Err(AudioError::Device)
            }
        };

        let channel_count = match self.channel_count {
            Some(channel_count) => channel_count,
            None => DEFAULT_CHANNELS_COUNT
        };

        let is_interleaved = match self.is_interleaved {
            Some(is_interleaved) => is_interleaved,
            None => DEFAULT_INTERLEAVED
        };

        let latency = match self.latency {
            Some(latency) => latency,
            None => match audio.device_info(device) {
                Ok(info) => info.default_low_input_latency,
                Err(_) => return Err(AudioError::Device)
            }
        };

        Ok(AudioStream {
            context: ctx ,
            settings: AudioSettings {
                device,
                channels: channel_count, is_interleaved, latency, stream_type
            }
        })
    }
}
