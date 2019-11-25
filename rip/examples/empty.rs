use rip_core::codec::Codec;
use rip_audio::error::AudioError;

fn main() {

}

struct Hello;

impl Codec<f32, u8> for Hello {
    fn encode(from: f32, to: u8) -> Result<u32, Box<AudioError>> {}
    fn decode(from: u8, to: f32) -> Result<u32, Box<AudioError>> {}
}