mod serial;
mod parts;
mod audio;

use std::sync::Arc;
use tokio::sync::Mutex;
use parts::potentiometer::Potentiometer;
use serial::Serial;
use crate::audio::{create_sink, destroy_sink, set_volume};

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

#[tokio::main]
async fn main() {
    destroy_sink("Microphone".to_string());
    destroy_sink("Spotify".to_string());
    destroy_sink("Discord".to_string());
    destroy_sink("Game".to_string());

    create_sink("Microphone".to_string());
    create_sink("Spotify".to_string());
    create_sink("Discord".to_string());
    create_sink("Game".to_string());

    let serial = Serial::open("/dev/ttyUSB0", 115200);
    let serial_arc = Arc::new(Mutex::new(serial));

    let pot0 = Potentiometer::new(serial_arc.clone(), 0);
    let pot1 = Potentiometer::new(serial_arc.clone(), 1);
    let pot2 = Potentiometer::new(serial_arc.clone(), 2);
    let pot3 = Potentiometer::new(serial_arc.clone(), 3);

    loop {
        set_volume("Microphone".to_string(), parse_volume(pot0.lock().await.value));
        set_volume("Spotify".to_string(), parse_volume(pot1.lock().await.value));
        set_volume("Discord".to_string(), parse_volume(pot2.lock().await.value));
        set_volume("Game".to_string(), parse_volume(pot3.lock().await.value));
    }
}
