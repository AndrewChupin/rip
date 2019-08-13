
extern crate portaudio;
use portaudio::{Time, Stream, StreamParameters, InputStreamSettings,
                OutputStreamSettings, Blocking, Flow, Sample};
use crate::audio_ctx::{AudioContext, AudioType};
use crate::def::{DEFAULT_FRAMES, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED};
use rip_core::builder::Builder;
use std::io::{Error, ErrorKind};
use crate::def::ADeviceId;
use portaudio::stream::{Flags, InputSettings};
use crate::flow::{InputFlow, OutputFlow, FlowSettings};
use crate::error::AudioError;


#[derive(Copy, Clone, Debug)]
pub struct AudioSettings {
    pub device: ADeviceId,
    pub channels: i32,
    pub is_interleaved: bool,
    pub latency: Time,
    pub stream_type: AudioType
}


pub struct AudioStream<'a> {
    pub(crate) context: &'a AudioContext,
    pub settings: AudioSettings
}


impl <'a> AudioStream<'a> {

    pub fn flow_input<T>(&self, attr: FlowSettings) -> Result<InputFlow<T>, AudioError>
        where T : ?Sized + Sample + 'static {

        let core = &self.context.core;

        let params = StreamParameters::<T>::new(
            self.settings.device,
            self.settings.channels,
            self.settings.is_interleaved,
            self.settings.latency
        );

        let settings = InputStreamSettings::new(params, attr.hz, attr.frame);

        let source = match core.open_blocking_stream(settings) {
            Ok(source) => source,
            Err(_) => return Err(AudioError::Stream)
        };

        return Ok(InputFlow { source, settings: attr })
    }

    pub fn flow_output<T>(&self, attr: FlowSettings) -> Result<OutputFlow<T>, AudioError>
        where T : ?Sized + Sample + 'static {

        let core = &self.context.core;

        let params = StreamParameters::<T>::new(
            self.settings.device,
            self.settings.channels,
            self.settings.is_interleaved,
            self.settings.latency
        );

        let settings = OutputStreamSettings::new(params, attr.hz, attr.frame);

        let source = match core.open_blocking_stream(settings) {
            Ok(source) => source,
            Err(_) => return Err(AudioError::Stream)
        };

        return Ok(OutputFlow { source, settings: attr })
    }

    pub fn non_blocking<T>(&self, frame: i32, hz: f64) {
        let core = &self.context.core;

        match self.settings.stream_type {
            AudioType::Input => {}
            AudioType::Output => {}
        };
    }
}
