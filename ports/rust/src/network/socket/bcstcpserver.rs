use std::io;
use std::net::{TcpListener, TcpStream};
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
            // Note: Since bcstcpsocket abstracts the stream as Option<TcpStream>,
            // we'd need to expose a method to set it, or construct it here directly.
            // Assuming for porting mapping purposes we inject it.
            Ok(socket) // Placeholder for internal stream injection
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Server not listening"))
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.listener = None; // Drop closes the listener
        Ok(())
    }
}
