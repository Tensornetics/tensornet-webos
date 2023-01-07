use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Kernel {
    device_drivers: HashMap<String, Arc<Mutex<dyn DeviceDriver>>>,
}

impl Kernel {
    fn new() -> Self {
        Self {
            device_drivers: HashMap::new(),
        }
    }

    fn register_device_driver(&mut self, name: String, driver: Arc<Mutex<dyn DeviceDriver>>) {
        self.device_drivers.insert(name, driver);
    }

    fn get_device_driver(&self, name: &str) -> Option<Arc<Mutex<dyn DeviceDriver>>> {
        self.device_drivers.get(name).cloned()
    }
}

trait DeviceDriver {
    fn read(&self, buffer: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, buffer: &[u8]) -> Result<usize, Error>;
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    DeviceError(String),
}
