use device_query::{DeviceQuery, DeviceState};
use scrap::{Capturer, Display};
use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

extern crate raylib;

pub fn capture_screen(
    shared_buffer: Arc<Mutex<Vec<u8>>>,
    condvar: Arc<Condvar>,
    stop_signal: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn Error>> {
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    loop {
        if *stop_signal.lock().unwrap() {
            break;
        }

        match capturer.frame() {
            Ok(frame) => {
                let mut buffer = frame.to_vec();
                if let Ok(mut shared) = shared_buffer.lock() {
                    std::mem::swap(&mut *shared, &mut buffer);
                    condvar.notify_one();
                } else {
                    eprintln!("Failed o lock shared buffer for writing");
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("Would block error: {e}");
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                eprintln!("Error capturing frame: {e}");
                break;
            }
        }
    }

    Ok(())
}

pub fn capture_input_states() -> ([f32; 256], [f32; 2], [f32; 2]) {
    let device_state = DeviceState::new();
    let keys = device_state.get_keys();
    let mouse = device_state.get_mouse();

    let mut key_states = [0f32; 256];
    for key in keys {
        key_states[key as usize] = 1.0;
    }

    // normalize mouse coords
    let mouse_coords = [
        (mouse.coords.0 as f32) / 1960.0 - 1.0 + 0.035,
        (mouse.coords.1 as f32) / 1080.0,
    ];

    let mut mouse_key_states = [0f32; 2];
    if mouse.button_pressed[1] == true {
        mouse_key_states[0] = 1.0;
    }
    if mouse.button_pressed[3] == true {
        mouse_key_states[1] = 1.0;
    }

    (key_states, mouse_coords, mouse_key_states)
}
