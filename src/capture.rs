use rdev::{EventType, Key};
use std::sync::{Arc, Mutex};

pub fn listen_to_events(buffer: Arc<Mutex<Vec<Key>>>) {
    println!("Listening to events");
    let _ = rdev::listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let buffer = Arc::clone(&buffer);
                callback_keyboard_event(buffer, key);
            }
            _ => {} // Ignore other events
        };
    });
}

fn callback_keyboard_event(buffer: Arc<Mutex<Vec<Key>>>, key: Key) {
    // TODO: Alt Gr + ) => ]

    let mut buffer = buffer.lock().unwrap();
    buffer.push(key);
}