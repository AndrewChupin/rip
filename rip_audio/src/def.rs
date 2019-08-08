use portaudio::{PortAudio, DeviceIndex, DeviceInfo, Sample, Stream, Blocking, Output, Flow, Input};


// Audio Default
pub const DEFAULT_RATE: f64 = 44_100.0;
pub const DEFAULT_FRAMES: u32 = 256;

pub const DEFAULT_CHANNELS_COUNT: i32 = 2;
pub const DEFAULT_INTERLEAVED: bool = true;


// AudioCore
pub(crate) type AudioCore = PortAudio;


// AudioDevice
pub type ADeviceId = DeviceIndex;
pub type ADeviceInfo<'a> = DeviceInfo<'a>;


// AudioSource
pub type InputSource<T>
    where T : ?Sized + Sample + 'static
= Stream<Blocking<<Output<T> as Flow>::Buffer>, Input<T>>;

pub type OutputSource<T>
    where T : ?Sized + Sample + 'static
= Stream<Blocking<<Output<T> as Flow>::Buffer>, Output<T>>;
