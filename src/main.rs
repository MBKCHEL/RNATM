use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use sysinfo::{System, Components};

fn main() {
    let mut sys = System::new_all();
    let mut components = Components::new_with_refreshed_list();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let _stdout = io::stdout().into_raw_mode().unwrap();
        let stdin = io::stdin();

        for key in stdin.keys() {
            if let Ok(Key::Char('q')) = key {
                let _ = tx.send(());
                break;
            }
        }
    });

    sys.refresh_all();
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU name: {}", cpu.brand());
    }

    loop {
        if rx.try_recv().is_ok() {
            break;
        }

        sys.refresh_all();
        components.refresh(true);

        let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let cpu_usage = sys.global_cpu_usage();

        let cpu_temp = components.iter().find_map(|comp| {
            let label = comp.label().to_lowercase();
            if label.contains("cpu")
                || label.contains("core")
                || label.contains("package")
                || label.contains("k10temp")
                || label.contains("zenpower")
            {
                comp.temperature()
            } else {
                None
            }
        });

        if let Some(temp) = cpu_temp {
            println!("\r{}: {:.1}°C", "CPU Temp \x1B[K", temp);
            println!("\rCPU usage: {:.1}% \x1B[K", cpu_usage);
            println!("\rTotal RAM: {:.2} GB / Used RAM: {:.2} GB \x1B[K", total_memory, used_memory);

            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(500));

            print!("\x1B[3A");
        }
    }
}
