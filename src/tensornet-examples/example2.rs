use tensornet::{Kernel, SystemLibrary};

struct ExampleLibrary;

impl SystemLibrary for ExampleLibrary {
    fn init(&mut self) -> Result<(), String> {
        // Initialize the library here
        Ok(())
    }

    fn foo(&self) -> i32 {
        // Implement the foo function
        0
    }

    fn bar(&self, arg: i32) -> String {
        // Implement the bar function
        "".to_string()
    }
}

fn main() {
    let mut kernel = Kernel::new();
    let example_library = Arc::new(Mutex::new(ExampleLibrary));
    kernel.register_system_library("example", example_library);
}
