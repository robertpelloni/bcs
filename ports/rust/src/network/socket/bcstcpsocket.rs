use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

// BcsTcpSocket abstracts a network client connection for Rust
pub struct BcsTcpSocket {
    stream: Option<TcpStream>,
}

impl BcsTcpSocket {
    pub fn new() -> Self {
        BcsTcpSocket { stream: None }
    }

    pub fn connect_to_host(&mut self, host: &str, port: &str) -> io::Result<()> {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(address)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if let Some(ref mut stream) = self.stream {
            stream.write(data)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Socket not connected"))
        }
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        if let Some(ref mut stream) = self.stream {
            let mut reader = BufReader::new(stream);
            let mut buffer = String::new();
            reader.read_line(&mut buffer)?;
            Ok(buffer)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Socket not connected"))
        }
    }

    pub fn disconnect(&mut self) -> io::Result<()> {
        self.stream = None; // Drops the connection
        Ok(())
    }
}
