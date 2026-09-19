use sysinfo::{
    Components, Disks, Networks, System,
};
use std::thread;
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        print!("\rTotal RAM: {:.2} GB / Used RAM: {:.2} GB   ", total_memory, used_memory);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}