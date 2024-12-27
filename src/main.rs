extern crate ffmpeg_next as ffmpeg;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use rayon::prelude::*;
use rdev::{listen, Event};
use scrap::{Capturer, Display};
use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn capture_screen(
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

                if let Ok(mut shared) = shared_buffer.lock() {
                    std::mem::swap(&mut *shared, &mut buffer);
                    condvar.notify_all();
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

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    let shared_buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    let condvar = Arc::new(Condvar::new());
    let stop_signal = Arc::new(Mutex::new(false));

    // screen capture thread
    let capture_buffer = Arc::clone(&shared_buffer);
    let condvar_clone = Arc::clone(&condvar);
    let stop_signal_clone = Arc::new(Mutex::new(false));
    thread::spawn(move || {
        if let Err(e) = capture_screen(capture_buffer, condvar_clone, stop_signal_clone) {
            eprintln!("Screen capture error: {e:?}");
        }
    });

    // global input monitoring thread
    thread::spawn(|| {
        if let Err(e) = listen(|_event: Event| {
            // println!("Global event: {event:?}");
        }) {
            eprintln!("Event error: {e:?}");
        }
    });

    // rendering window
    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: true,
        scale: Scale::X1,
        scale_mode: ScaleMode::Center,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window = Window::new("RSGLITCH", WIDTH, HEIGHT, window_options)?;

    window.set_target_fps(60);

    // rendering thread
    loop {
        if !window.is_open() || window.is_key_down(Key::Q) {
            *stop_signal.lock().unwrap() = true;
            break;
        }

        let mut shared = shared_buffer.lock().unwrap();

        while shared.is_empty() {
            shared = condvar.wait(shared).unwrap();
        }

        if let Err(e) = window.update_with_buffer(&shared, WIDTH, HEIGHT) {
            eprintln!("Error updating window with buffer: {e}");
            break;
        }

        shared.clear();
    }

    Ok(())
}
