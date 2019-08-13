use crate::def::{InputSource, OutputSource};
use portaudio::Sample;
use portaudio::{Error, StreamAvailable};
use portaudio::stream::Available;



pub struct InputFlow<T> where T : ?Sized + Sample + 'static {
    pub source: InputSource<T>,
    pub settings: FlowSettings
}

unsafe impl <T> Send for InputFlow<T> where T : ?Sized + Sample + 'static {}

impl <T> InputFlow<T> where T : ?Sized + Sample + 'static {

    pub fn start(&mut self) {
        self.source.start();
    }

    pub fn read(&self, frames: u32) -> &[T] {
        self.source.read(frames).unwrap()
    }

    pub fn read_available(&self) -> Result<Available, Error> {
        self.source.read_available()
    }
}



pub struct OutputFlow<T> where T : ?Sized + Sample + 'static {
    pub source: OutputSource<T>,
    pub settings: FlowSettings
}

unsafe impl <T> Send for OutputFlow<T> where T : ?Sized + Sample + 'static {}

impl <T> OutputFlow<T> where T : ?Sized + Sample + 'static {

    pub fn start(&mut self) {
        self.source.start();
    }

    pub fn write<WF>(&mut self, frames: u32, write_fn: WF)
        where WF : FnOnce(&mut [T]) {
        self.source.write(frames, write_fn);
    }

    pub fn write_available(&self) -> Result<Available, Error> {
        self.source.write_available()
    }
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


pub fn wait_for_stream<F>(f: F, name: &str) -> u32
    where F: Fn() -> Result<StreamAvailable, Error> {
    'waiting_for_stream: loop {
        match f() {
            Ok(available) => match available {
                StreamAvailable::Frames(frames) => {
                    if frames > 0 {
                        println!("frame {}", frames);
                        return frames as u32
                    }
                },
                StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                StreamAvailable::OutputUnderflowed => println!("Output stream has underflowed"),
            },
            Err(err) => panic!("An error occurred while waiting for the {} stream: {}", name, err),
        }
    }
}