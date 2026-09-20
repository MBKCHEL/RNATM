use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();

    if let Some(cpu) = sys.cpus().first() {
        println!("\rCPU name: {}", cpu.brand());
    }

    loop {
        sys.refresh_all();

        let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        let cpu_usage = sys.global_cpu_usage();

        println!("CPU usage: {:.1}% \x1B[K", cpu_usage);

        println!(
            "\rTotal RAM: {:.2} GB / Used RAM: {:.2} GB   ",
            total_memory, used_memory
        );

        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(500));

        print!("\x1B[2A");
    }
}

