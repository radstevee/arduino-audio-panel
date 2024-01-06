use std::io::Read;
use serial2::SerialPort;

pub struct Serial {
    pub port: SerialPort,
    pub buffer: Vec<u8>,
}

impl Serial {
    pub fn open(path: &str, baud_rate: u32) -> Serial {
        let port = SerialPort::open(path, baud_rate).unwrap();
        Serial {
            port,
            buffer: Vec::new(),
        }
    }

    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        let mut byte: [u8; 1] = [0];
        loop {
            self.port.read_exact(&mut byte).unwrap();
            if byte[0] == b'\n' {
                break;
            }
            line.push(byte[0] as char);
        }
        Some(line)
    }
}