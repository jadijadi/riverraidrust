use rand::thread_rng;
use std::io::stdout;
use std::{thread, time};
use stout_ext::StdoutExt;

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand,
};

mod drawable;
mod events;
mod greeting;
mod physics;
mod stout_ext;
mod world;

use events::*;
use greeting::*;
use physics::*;
use world::*;

/// Game Physic Rules
/// TODO: Move to Physics.rs module later
fn physics(world: &mut World) {
    let mut rng = thread_rng();

    // check if player hit the ground
    check_player_status(world);

    // check enemy hit something
    check_enemy_status(world);
    check_fuel_status(world);

    // move the map Downward
    update_map(&mut rng, world);

    // create new enemy
    create_enemy(&mut rng, world);
    create_fuel(&mut rng, world);

    // Move elements along map movements
    move_enemies(world);
    move_fuel(world);
    move_bullets(world);

    if world.player.gas >= 1 {
        world.player.gas -= 1;
    }
}

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    let slowness = 100;
    let mut world = World::new(maxc, maxl);

    // show welcoming banner
    welcome_screen(&mut sc, &world)?;

    while world.player.status == PlayerStatus::Alive || world.player.status == PlayerStatus::Paused
    {
        handle_pressed_keys(&mut world);
        if world.player.status != PlayerStatus::Paused {
            physics(&mut world);
            world.draw(&mut sc)?;
        } else {
            pause_screen(&mut sc, &world)?;
        }
        thread::sleep(time::Duration::from_millis(slowness));
    }

    // game is finished
    sc.clear_all()?;
    goodbye_screen(&mut sc, &world)?;
    sc.clear_all()?.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
