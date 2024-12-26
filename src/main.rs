use device_query::{mouse_state::MouseState, DeviceQuery, DeviceState, Keycode};
use scrap::{Capturer, Display};
use std::sync::mpsc::Receiver;
use std::thread;
use std::{error::Error, fmt::Debug};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{StartCause, WindowEvent},
    event_loop::{self, ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct RSGlitch {
    window: Option<Window>,
}

impl<T: 'static + Debug> ApplicationHandler<T> for RSGlitch {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = match event_loop
                .create_window(WindowAttributes::default().with_title("RSGLITCH"))
            {
                Ok(window) => Some(window),
                Err(e) => panic!("Error creating window: {e}"),
            };
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &self.window {
            if window.id() == window_id {
                match event {
                    WindowEvent::CloseRequested => {
                        self.window = None;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn monitor_input() {
    let device_state = DeviceState::new();
    let mut last_mouse_state: Option<MouseState> = None;

    loop {
        let keys = device_state.get_keys();
        let mouse_state = device_state.get_mouse();
        println!("Keys: {keys:?}");
        println!("Last mouse state: {last_mouse_state:?}");
        println!("mouse state: {mouse_state:?}");
        for key in keys {
            match key {
                _ => {}
            }
        }

        last_mouse_state = Some(mouse_state);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // global input monitoring
    thread::spawn(|| {
        monitor_input();
    });

    // setup screen capturing
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    // setup rendering window
    let event_loop = EventLoop::new().unwrap();
    match event_loop.run_app(&mut (RSGlitch { window: None })) {
        Ok(_) => Ok(()),
        Err(e) => panic!("Event loop failure"),
    }
}
