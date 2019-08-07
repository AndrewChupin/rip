use std::io::{Error, ErrorKind};
use crate::builder::ABuilder;
use crate::def::{AudioCore, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, ADeviceId, ADeviceInfo};
use crate::stream::AudioStream;
use portaudio::{DeviceInfo, DeviceIndex};
use std::iter::FromIterator;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FlowType {
    Input, Output
}


#[derive(Debug, PartialEq, Clone)]
pub struct AudioDevice<'a> {
    pub id: ADeviceId,
    pub info: ADeviceInfo<'a>
}

pub struct AudioContext {
    pub(crate) core: AudioCore
}


impl AudioContext {

    pub fn new() -> Result<AudioContext, ErrorKind> {
        let port_a = match portaudio::PortAudio::new() {
            Ok(audio) => audio,
            Err(_) => return Err(ErrorKind::Interrupted)
        };

        let context = AudioContext { core: port_a };
        Ok(context)
    }

    pub fn default_device(&self, a_type: FlowType) -> Result<AudioDevice, ErrorKind> {
        let a_core = &self.core;
        match a_type {
            FlowType::Output => match a_core.default_output_device() {
                Ok(device) => Ok(AudioDevice {
                    id: device,
                    info: self.core.device_info(device).unwrap()
                }),
                Err(_) => return Err(ErrorKind::NotFound)
            },
            FlowType::Input => match a_core.default_input_device() {
                Ok(device) => Ok(AudioDevice {
                    id: device,
                    info: self.core.device_info(device).unwrap()
                }),
                Err(_) => return Err(ErrorKind::NotFound)
            }
        }
    }
}
