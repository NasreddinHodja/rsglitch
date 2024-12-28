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

fn update_texture_from_buffer(texture: &mut Texture2D, buffer: &Vec<u32>) {
    let byte_data: Vec<u8> = buffer
        .iter()
        .flat_map(|&pixel| {
            vec![
                ((pixel >> 16) & 0xFF) as u8, // Red
                ((pixel >> 8) & 0xFF) as u8,  // Green
                (pixel & 0xFF) as u8,         // Blue
                ((pixel >> 24) & 0xFF) as u8, // Alpha
            ]
        })
        .collect();

    texture.update_texture(&byte_data);
}

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    let shared_buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
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

    // create rectangle to render the texture on
    // let rec = Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32);

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

        update_texture_from_buffer(&mut texture, &shared);

        // draw
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        {
            let mut shader_mode = draw.begin_shader_mode(&shader);
            // shader_mode.draw_circle(100, 100, 100.0, Color::BLUE);
            shader_mode.draw_texture(&texture, 0, 0, Color::WHITE);
        }
        // draw.draw_texture(&texture, 0, 0, Color::WHITE);
        // draw.draw_circle(100, 100, 100.0, Color::BLUE);
    }

    Ok(())
}
