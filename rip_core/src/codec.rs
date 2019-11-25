use std::error::Error;

pub trait Encoder<F, T> {
    fn encode(from: F, to: T) -> Result<u32, Box<dyn Error>>;
}

pub trait Decoder<F, T> {
    fn decode(from: T, to: F) -> Result<u32, Box<dyn Error>>;
}

pub trait Codec<F, T> : Encoder<F, T> + Decoder<T, F>  {
    fn encode(from: F, to: T) -> Result<u32, Box<dyn Error>>;
    fn decode(from: T, to: F) -> Result<u32, Box<dyn Error>>;
}
