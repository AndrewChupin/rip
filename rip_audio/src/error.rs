use std::fmt;

#[derive(Debug)]
pub enum AudioError {
    Core,
    Device,
    Context,
    Stream
}
