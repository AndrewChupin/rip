use portaudio::{PortAudio, DeviceIndex, DeviceInfo};

pub(crate) const DEFAULT_RATE: f64 = 44_100.0;
pub(crate) const DEFAULT_FRAMES: u32 = 256;

pub(crate) const DEFAULT_CHANNELS_COUNT: i32 = 2;
pub(crate) const DEFAULT_INTERLEAVED: bool = true;

pub(crate) type AudioCore = PortAudio;

pub type ADeviceId = DeviceIndex;
pub type ADeviceInfo<'a> = DeviceInfo<'a>;
