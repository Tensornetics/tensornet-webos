use tensornet_core::Kernel;
use tensornet_core::device_drivers::{KeyboardDriver, MouseDriver, AudioDriver};
use tensornet_system_libraries::{FileSystem, Network, Graphics};

fn main() {
    let mut kernel = Kernel::new();

    // Load device drivers
    let keyboard_driver = Arc::new(Mutex::new(KeyboardDriver::new(
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    )));
    kernel.register_device_driver("keyboard".to_string(), keyboard_driver.clone());

    let mouse_driver = Arc::new(Mutex::new(MouseDriver::new(
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    )));
    kernel.register_device_driver("mouse".to_string(), mouse_driver.clone());

    let audio_driver = Arc::new(Mutex::new(AudioDriver::new(
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    )));
    kernel.register_device_driver("audio".to_string(), audio_driver.clone());

    // Initialize system libraries
    let filesystem = FileSystem::new();
    let network = Network::new();
    let graphics = Graphics::new();

    // Application code goes here
    let mut buffer = [0; 1024];
    let mut running = true;
    while running {
        let mut keyboard_driver = kernel.get_device_driver("keyboard").unwrap().lock().unwrap();
        let n = keyboard_driver.read(&mut buffer).unwrap();
        if n > 0 {
            let s = std::str::from_utf8(&buffer[0..n]).unwrap();
            if s == "q" {
                running = false;
            }
        }
    }
}
