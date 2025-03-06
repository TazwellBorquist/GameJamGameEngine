extern crate sdl2;

use std::error::Error;
use std::rc::Rc;

mod renderer;
mod sprite_sdl;
mod assets;
mod entities;

use sdl2::event::Event;

/// Initializes and starts the engine
pub fn run() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init().unwrap();

    let render = renderer::Renderer::new(&sdl, 800, 800);
    let texture_creator = render.canvas.borrow().texture_creator();
    let asset_bucket = assets::Assets::new(&texture_creator);

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        match event_pump.poll_event() {
            Some(
                Event::KeyDown { 
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _ }
            ) => {
                if keycode.is_some() && keycode.unwrap() == sdl2::keyboard::Keycode::ESCAPE {
                    break 'running;
                }
            }
            _ => continue,

        }
    }

    Ok(())
}