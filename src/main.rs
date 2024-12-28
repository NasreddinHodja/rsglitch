use std::{
    error::Error,
    fs,
    sync::{Arc, Condvar, Mutex},
    thread,
};

extern crate raylib;
use raylib::prelude::*;

mod capture;
use capture::capture_screen;

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    let shared_buffer = Arc::new(Mutex::new(vec![0u8; WIDTH * HEIGHT * 4]));
    let condvar = Arc::new(Condvar::new());

    // screen capture thread
    let capture_buffer = Arc::clone(&shared_buffer);
    let condvar_clone = Arc::clone(&condvar);
    let stop_signal = Arc::new(Mutex::new(false));
    thread::spawn(move || {
        if let Err(e) = capture_screen(capture_buffer, condvar_clone, stop_signal) {
            eprintln!("Screen capture error: {e:?}");
        }
    });

    // raylib setup
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("RSGLITCH")
        .build();

    let vertex_shader_code = fs::read_to_string("shaders/vertex.vs")?;
    let fragment_shader_code = fs::read_to_string("shaders/fragment.fs")?;

    // load shader
    let mut shader = rl.load_shader_from_memory(
        &thread,
        Some(&vertex_shader_code),
        Some(&fragment_shader_code),
    );

    // load initial empty texture
    let mut texture = rl
        .load_texture_from_image(
            &thread,
            &Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK),
        )
        .unwrap();

    let texture_location = shader.get_shader_location("texture0");
    if texture_location != -1 {
        shader.set_shader_value_texture(texture_location, &texture);
    } else {
        eprintln!("Error: texture0 not found");
    }

    // main loop
    while !rl.window_should_close() {
        // update captured texture
        let mut shared = shared_buffer.lock().unwrap();
        shared = condvar.wait(shared).unwrap();

        texture.update_texture(&shared);

        // draw
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        {
            let mut shader_mode = draw.begin_shader_mode(&shader);

            shader_mode.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    Ok(())
}
