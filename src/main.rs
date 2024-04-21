use std::io::stdout;
use stout_ext::StdoutExt;

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand,
};

mod canvas;
mod drawable;
mod entities;
mod events;
mod stout_ext;
mod world;

use events::*;
use world::*;

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    let slowness = 60; // XXX oh, issue #85 was just solved here before...
    let mut world = World::new(maxc, maxl);

    // show welcoming banner
    world.welcome_screen(&mut sc)?;

    // Main game loop
    // - Events
    // - Physics
    // - Drawing
    world.game_loop(&mut sc, slowness)?;

    // game is finished
    world.clear_screen(&mut sc)?;
    world.goodbye_screen(&mut sc)?;

    sc.clear_all()?.execute(Show)?;
    disable_raw_mode()?;
    println!(""); // XXX ??? no "%" at the end of the program
    Ok(())
}
