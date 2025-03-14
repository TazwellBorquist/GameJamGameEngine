extern crate gl;
extern crate sdl2;

mod engine;
#[macro_use]
mod logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
