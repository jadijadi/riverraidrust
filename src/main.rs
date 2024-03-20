use std::io::stdout;
use stout_ext::StdoutExt;

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand,
};

mod drawable;
mod entities;
mod events;
mod physics;
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
    let slowness = 100;
    let mut world = World::new(sc, maxc, maxl);

    // show welcoming banner
    world.welcome_screen()?;

    // Main game loop
    // - Events
    // - Physics
    // - Drawing
    world.game_loop(slowness)?;

    // game is finished
    world.clear_screen()?;
    world.goodbye_screen()?;

    let mut sc = world.stdout;
    sc.clear_all()?.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
