pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn run(&mut self) {
        let mut frame_count = 0;
        loop {
            println!("New frame {}", frame_count);

            // **********
            std::thread::sleep(std::time::Duration::from_millis(16));
            frame_count += 1;
        }
    }
}
