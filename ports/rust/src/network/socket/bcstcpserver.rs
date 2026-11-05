use std::io;
use std::net::TcpListener;
use super::bcstcpsocket::BcsTcpSocket;

// BcsTcpServer abstracts a network listener socket for Rust
pub struct BcsTcpServer {
    listener: Option<TcpListener>,
}

impl BcsTcpServer {
    pub fn new() -> Self {
        BcsTcpServer { listener: None }
    }

    pub fn listen(&mut self, port: &str) -> io::Result<()> {
        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(address)?;
        self.listener = Some(listener);
        Ok(())
    }

    pub fn accept(&self) -> io::Result<BcsTcpSocket> {
        if let Some(ref listener) = self.listener {
            let (stream, _) = listener.accept()?;
            let mut socket = BcsTcpSocket::new();
            socket.stream = Some(stream);
            Ok(socket)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Server not listening"))
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.listener = None; // Drop closes the listener
        Ok(())
    }
}
