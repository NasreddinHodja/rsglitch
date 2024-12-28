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
