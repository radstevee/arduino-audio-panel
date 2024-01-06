use std::sync::Arc;
use tokio::sync::Mutex;
use crate::serial::Serial;

pub struct Potentiometer {
    pub id: u8,
    pub value: u8,
    serial: Arc<Mutex<Serial>>,
    serial_lock: Mutex<()>,
}

impl Potentiometer {
    pub fn new(serial: Arc<Mutex<Serial>>, id: u8) -> Arc<Mutex<Self>> {
        let pot = Arc::new(Mutex::new(Potentiometer {
            id,
            value: 0,
            serial,
            serial_lock: Mutex::new(()),
        }));

        let pot_clone = Arc::clone(&pot);

        tokio::spawn(async move {
            loop {
                let mut pot_lock = pot_clone.lock().await;

                // Get the lock for serial communication within pot lock
                let _serial_guard = pot_lock.serial_lock.lock().await;

                // Drop the serial guard before setting the value
                drop(_serial_guard);

                if let Some(value) = pot_lock.read().await {
                    // Now, we can safely set the value without conflicting borrows
                    pot_lock.set_value(value);
                }
                // Pot lock is dropped automatically at the end of the loop.
            }
        });

        pot
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub async fn read(&mut self) -> Option<u8> {
        let line = self.serial.lock().await.read_line()?;
        let mut parts = line.split('=');
        let id = parts.next().unwrap();
        let value = parts.next().unwrap();
        if id == format!("pot{}", self.id) {
            let u8_value: u8 = value.trim_end().parse().unwrap();
            Some(u8_value)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub async fn get_value(pot: Arc<Mutex<Self>>) -> u8 {
        let pot_lock = pot.lock().await;
        let value = pot_lock.value;
        // Lock is automatically released when pot lock goes out of scope.
        value
    }
}