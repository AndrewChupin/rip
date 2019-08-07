
extern crate portaudio;
use portaudio::{DeviceIndex, Time, Stream};
use crate::audio_ctx::{AudioContext, FlowType};
use crate::{DEFAULT_FRAMES, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED};
use rip_core::builder::Builder;
use std::io::{Error, ErrorKind};
use crate::def::ADeviceId;


pub struct AudioStream<'a> {
    pub(crate) context: &'a AudioContext,
    pub device: ADeviceId,
    pub channel_count: i32,
    pub interleaved: bool,
    pub latency: Time,
    pub stream_type: FlowType
}

impl <'a> AudioStream<'a> {

    fn blocking(&self, frame: i32, hz: f64) {
        let core = &self.context.core;

        match self.stream_type {
            FlowType::Input => {

            }
            FlowType::Output => {

            }
        };
    }

    fn non_blocking(&self, frame: i32, hz: f64) {
        let core = &self.context.core;

        match self.stream_type {
            FlowType::Input => {

            }
            FlowType::Output => {

            }
        };
    }
}
