use crate::socket::{Socket, Connect};


pub struct TcpConnection<'a> {
    socket: Socket<'a>
}

impl TcpConnection {

    pub const fn new<T>(address: &T) -> TcpConnection
        where T : AsRef<&str> {
        return TcpConnection {
            socket: Socket(address)
        };
    }
}

impl Connect for TcpConnection {

    fn start<C, E>(&self) -> Result<C, E> {
        let con = TcpConnection::new("dsa");
    }

    fn stop<C, E>(&self) -> Result<C, E> {

    }
}
