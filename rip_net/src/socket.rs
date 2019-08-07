

pub struct Socket<'a> {
    pub address: &'a str,
}


pub trait Connect {
    fn start<C, E>(&self) -> Result<C, E>;
    fn stop<C, E>(&self) -> Result<C, E>;
}
