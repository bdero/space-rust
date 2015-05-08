extern crate sdl2;

use sdl2::video::{ Window, WindowPos };


mod sr {

    extern crate sdl2;


    pub enum Error {
        Sdl2Error(String)
    }

    pub struct Game {
        pub context: sdl2::sdl::Sdl
    }

    impl Game { }

    pub fn init() -> Result<Game, Error> {
        match sdl2::init(sdl2::INIT_EVERYTHING) {
            Ok(context) => Ok(Game { context: context }),
            Err(message) => Err(Error::Sdl2Error(
                format!("Couldn't initialize SDL context: {}", message)))
        }
    }

}

fn main() {
    // Initialize sdl2
    let game = match sr::init() {
        Ok(context) => context,
        Err(error) => panic!(error)
    };

    // Create a window
    let window = match Window::new(&game.context, "Space Rust",
                                   WindowPos::PosCentered,
                                   WindowPos::PosCentered,
                                   800, 600,
                                   sdl2::video::OPENGL | sdl2::video::SHOWN) {
        Ok(window) => window,
        Err(error) => panic!("Couldn't initialize SDL window: {}", error)
    };

    // Start the event loop
    let mut event_pump = game.context.event_pump();

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
