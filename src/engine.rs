extern crate sdl2;

use std::error::Error;

#[macro_use]
mod gamespace;

mod assets;

mod entity_manager;

//mod renderer;
//mod sprite_sdl;

/// Initializes and starts the engine
pub fn run() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let vid = sdl.video()?;

    let win = vid.window("title", 800, 800).build()?;
    let can = win.into_canvas().build()?;

    let tc = can.texture_creator();
    let assets = assets::AssetCollection::new(&tc);

    Ok(())
}
