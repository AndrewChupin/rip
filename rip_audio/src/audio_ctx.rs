use std::io::{Error, ErrorKind};
use crate::builder::StreamBuilder;
use crate::def::{AudioCore, DEFAULT_CHANNELS_COUNT, DEFAULT_INTERLEAVED, ADeviceId, ADeviceInfo};
use crate::stream::AudioStream;
use portaudio::{DeviceInfo, DeviceIndex};
use std::iter::FromIterator;
use crate::error::AudioError;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AudioType {
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

    pub fn new() -> Result<AudioContext, AudioError> {
        let port_a = match portaudio::PortAudio::new() {
            Ok(audio) => audio,
            Err(_) => return Err(AudioError::Core)
        };

        let context = AudioContext { core: port_a };
        Ok(context)
    }

    pub fn default_device(&self, a_type: AudioType) -> Result<AudioDevice, AudioError> {
        let a_core = &self.core;
        match a_type { // match

            AudioType::Output => match a_core.default_output_device() { // match
                Ok(device) => Ok(
                    AudioDevice {
                        id: device,
                        info: match self.core.device_info(device) { // match
                            Ok(device) => device,
                            Err(_) => return Err(AudioError::Device)
                        }
                    }
                ),
                Err(_) => return Err(AudioError::Device)
            },

            AudioType::Input => match a_core.default_input_device() { // match
                Ok(device) => Ok(
                    AudioDevice {
                        id: device,
                        info: match self.core.device_info(device) { // match
                            Ok(device) => device,
                            Err(_) => return Err(AudioError::Device)
                        }
                    }
                ),
                Err(_) => return Err(AudioError::Device)
            }
        }
    }
}
