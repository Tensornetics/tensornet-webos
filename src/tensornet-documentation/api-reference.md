Tensornet API Reference
The Tensornet API provides access to the core functionality of the operating system, including device management, system libraries, and application execution. In this reference guide, we will provide an overview of the available API functions and how to use them.

Kernel
The Kernel struct provides access to the core functionality of Tensornet. It is responsible for managing device drivers and system libraries, and provides an interface for running applications.

Kernel::new() -> Kernel
Creates a new instance of the Kernel struct.

Kernel::register_device_driver(name: String, driver: Arc<Mutex<dyn DeviceDriver>>)
Registers a new device driver with the kernel. The name parameter is used to identify the driver, and the driver parameter is an Arc to a mutex-wrapped device driver object.

Kernel::get_device_driver(name: &str) -> Option<Arc<Mutex<dyn DeviceDriver>>>
Retrieves a registered device driver by its name. Returns None if the driver could not be found.

DeviceDriver
The DeviceDriver trait is implemented by all device drivers in Tensornet. It provides a common interface for interacting with hardware devices.

fn init(&mut self) -> Result<(), String>
Initializes the device driver.

fn read(&mut self, buffer: &mut [u8]) -> Result<usize, String>
Reads data from the device into the given buffer. Returns the number of bytes read.

fn write(&mut self, buffer: &[u8]) -> Result<usize, String>
Writes data from the given buffer to the device. Returns the number of bytes written.

FileSystem
The FileSystem struct provides access to the file system on the device.

FileSystem::new() -> FileSystem
Creates a new instance of the FileSystem struct.

FileSystem::open_file(path: &str) -> Result<File, String>
Opens the file at the given path and returns a File object. Returns an error if the file could not be opened.

FileSystem::create_directory(path: &str) -> Result<(), String>
Creates a new directory at the given path. Returns an error if the directory could not be created.

Network
The Network struct provides access to the device's network capabilities.

Network::new() -> Network
Creates a new instance of the Network struct.