extern crate sdl2;


mod sr {

    extern crate sdl2;

    use sdl2::sdl::Sdl;
    use sdl2::video::{ Window, WindowPos };


    pub enum Error {
        Sdl2Error(String)
    }

    pub struct Game {
        pub context: sdl2::sdl::Sdl,
        pub window: Window
    }

    impl Game { }

    pub fn init() -> Result<Game, Error> {
        let context = match sdl2::init(sdl2::INIT_EVERYTHING) {
            Ok(context) => context,
            Err(message) => return Err(Error::Sdl2Error(
                format!("Couldn't initialize SDL context: {}", message)))
        };

        let window = match Window::new(&context, "Space Rust",
                                       WindowPos::PosCentered,
                                       WindowPos::PosCentered,
                                       800, 600,
                                       sdl2::video::OPENGL
                                       | sdl2::video::SHOWN) {
            Ok(window) => window,
            Err(message) => return Err(Error::Sdl2Error(
                format!("Couldn't initialize SDL window: {}", message)))
    };

        Ok(Game { context: context, window: window })
    }

}

fn main() {
    // Initialize sdl2
    let game = match sr::init() {
        Ok(context) => context,
        Err(error) => panic!(error)
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
