use rdev::{EventType, Key};
use std::sync::{Arc, Mutex};

pub fn listen_to_events(buffer: Arc<Mutex<Vec<Key>>>) {
    let _ = rdev::listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            let buffer = Arc::clone(&buffer);
            callback_keyboard_event(buffer, key);
        }
    });
}

fn callback_keyboard_event(buffer: Arc<Mutex<Vec<Key>>>, key: Key) {
    // TODO: Alt Gr + ) => ]

    let mut buffer = buffer.lock().unwrap();
    buffer.push(key);
}
