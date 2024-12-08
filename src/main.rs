mod audio;
mod parts;
mod serial;

use crate::audio::{create_sink, destroy_sink, set_volume};
use parts::potentiometer::Potentiometer;
use serial::Serial;
use serial2::SerialPort;
use std::{fs, path::Path, sync::Arc};
use tokio::sync::Mutex;

fn parse_volume(volume: u8) -> f32 {
    let volume = volume as f32;
    let volume = volume / 100.0;
    // Fix 1 to 1.0
    if volume >= 1.0 {
        return 1.0;
    }
    // Fix 0 to 0.0
    if volume <= 0.0 {
        return 0.0;
    }
    volume
}

fn setup_sinks(names: Vec<&str>) {
    for name in names {
        destroy_sink(name.to_string());
        create_sink(name.to_string());
    }
}

#[tokio::main]
async fn main() {
    setup_sinks(vec!["Microphone", "Spotify", "Discord", "Game"]);

    let ports = fs::read_dir("/dev/")
        .unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| {
            path.to_str().unwrap().contains("ttyUSB") || path.to_str().unwrap().contains("ttyACM")
        })
        .collect::<Vec<_>>();

    let port = ports.first().unwrap().to_str().unwrap();

    let serial = Serial::open(port, 115200).unwrap();
    let serial_arc = Arc::new(Mutex::new(serial));

    let pot0 = Potentiometer::new(serial_arc.clone(), 0);
    let pot1 = Potentiometer::new(serial_arc.clone(), 1);
    let pot2 = Potentiometer::new(serial_arc.clone(), 2);
    let pot3 = Potentiometer::new(serial_arc.clone(), 3);

    loop {
        set_volume("Microphone".to_string(), pot0.lock().await.value);
        set_volume("Spotify".to_string(), pot1.lock().await.value);
        set_volume("Discord".to_string(), pot2.lock().await.value);
        set_volume("Game".to_string(), pot3.lock().await.value);
    }
}
