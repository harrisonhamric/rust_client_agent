use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use sysinfo::{CpuExt, System, SystemExt};

fn handle_connection(mut stream: TcpStream) {
    let mut system = System::new_all();
    system.refresh_cpu();

    let mut cpu_load = None;
    for cpu in system.cpus() {
        cpu_load = Some(cpu.cpu_usage());
    }

    let cpu_load = match cpu_load {
        Some(load) => load,
        None => {
            eprintln!("Error: No CPUs found");
            return;
        }
    };

    let message = format!("CPU Usage: {:.2}%", cpu_load);
    stream.write(message.as_bytes()).expect("Failed to write to server");
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    loop {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(|| {
                handle_connection(stream);
            });
        }
    }
}
