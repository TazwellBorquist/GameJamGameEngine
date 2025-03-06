extern crate gl;
extern crate sdl2;

mod engine;
#[macro_use]
mod logger;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut assets = fs::read_dir("assets")?;
    for asset in assets {
        println!("{:?}", asset.unwrap().file_name());
    }

    // start the logger, not completely required but good practice
    logger_init!();
     
    // start the game
    let run = engine::run();
    if run.is_err() {
        log!("ERROR: {}", run.unwrap_err());
    }

    // close the logger
    logger_close!();
    Ok(())
}