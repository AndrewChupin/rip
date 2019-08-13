use bytes::{BufMut, BytesMut};
use std::io;
use tokio::codec::{Decoder, Encoder};


pub struct Bytes;


impl Decoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        if buf.len() > 0 {
            let len = buf.len();
            Ok(Some(buf.split_to(len).into_iter().collect()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.put(&data[..]);
        Ok(())
    }
}
