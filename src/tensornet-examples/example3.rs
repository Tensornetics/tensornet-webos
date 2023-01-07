use tensornet::{Kernel, App};

struct ExampleApp;

impl App for ExampleApp {
    fn init(&mut self) -> Result<(), String> {
        // Initialize the app here
        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        // Run the app's main loop here
        Ok(())
    }
}

fn main() {
    let mut kernel = Kernel::new();
    let example_app = Arc::new(Mutex::new(ExampleApp));
    kernel.register_app("example", example_app);
    kernel.run_app("example").unwrap();
}
