use rayon::prelude::*;
use scrap::{Capturer, Display};
use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

extern crate raylib;

pub fn capture_screen(
    shared_buffer: Arc<Mutex<Vec<u32>>>,
    condvar: Arc<Condvar>,
    stop_signal: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn Error>> {
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;
    let width = capturer.width();
    let height = capturer.height();

    loop {
        if *stop_signal.lock().unwrap() {
            break;
        }

        match capturer.frame() {
            Ok(frame) => {
                // convert frame from BGRA to ARGB for minifb
                let mut buffer = vec![0; width * height];
                buffer
                    .par_chunks_mut(width.into())
                    .enumerate()
                    .for_each(|(y, row)| {
                        for x in 0..width {
                            let idx = (y * width + x) * 4;
                            let b = frame[idx];
                            let g = frame[idx + 1];
                            let r = frame[idx + 2];
                            row[x] =
                                0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                        }
                    });

                // let buffer = frame.to_vec();
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
