extern crate sdl2;

use sdl2::video::{ Window, WindowPos };

fn main() {
    // Initialize sdl2
    let sdl_context = match sdl2::init(sdl2::INIT_EVERYTHING) {
        Ok(sdl_context) => sdl_context,
        Err(error) => panic!("Couldn't initialize SDL context: {}", error)
    };

    // Create a window
    let window = match Window::new(&sdl_context, "Space Rust",
                                   WindowPos::PosCentered,
                                   WindowPos::PosCentered,
                                   800, 600,
                                   sdl2::video::OPENGL | sdl2::video::SHOWN) {
        Ok(window) => window,
        Err(error) => panic!("Couldn't initialize SDL window: {}", error)
    };

    // Start the event loop
    let mut event_pump = sdl_context.event_pump();

    'update: loop {
        // Poll events
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => break 'update,
                _ => ()
            }
        }

        // Update game
    }
}
