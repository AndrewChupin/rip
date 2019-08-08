use crate::def::{InputSource, OutputSource};
use portaudio::Sample;


pub struct InputFlow<T> where T : ?Sized + Sample + 'static {
    pub source: InputSource<T>,
    pub settings: FlowSettings
}

pub struct OutputFlow<T> where T : ?Sized + Sample + 'static {
    pub source: OutputSource<T>,
    pub settings: FlowSettings
}

#[derive(Copy, Clone, PartialEq)]
pub struct FlowSettings {
    pub frame: u32,
    pub hz: f64
}

impl FlowSettings {

    pub fn new(frame: u32, hz: f64) -> Self {
        return FlowSettings { frame, hz };
    }

}
