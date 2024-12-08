use serial2::{CharSize, FlowControl, Parity, SerialPort, Settings, StopBits};
use std::io::{Read, Result};

pub struct Serial {
    pub port: SerialPort,
    pub buffer: Vec<u8>,
}

impl Serial {
    pub fn open(path: &str, baud_rate: u32) -> Result<Serial> {
        let port = SerialPort::open(path, |mut settings: Settings| {
            settings.set_raw();
            settings.set_baud_rate(baud_rate)?;
            settings.set_char_size(CharSize::Bits7);
            settings.set_stop_bits(StopBits::Two);
            settings.set_parity(Parity::Odd);
            settings.set_flow_control(FlowControl::RtsCts);
            Ok(settings)
        })?;
        Ok(Serial {
            port,
            buffer: Vec::new(),
        })
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

