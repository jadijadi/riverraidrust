use rand:: thread_rng;
use std::io::stdout;
use std::{thread, time};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand, QueueableCommand,
};

mod physics;
mod world;
mod greeting;
mod events;

use world::world::{*};
use physics::physics::{*};
use greeting::greeting::{*};
use events::events::{*};


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

    if world.gas >= 1 {
        world.gas -= 1;
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
    welcome_screen(&sc, &world);

    while world.status == PlayerStatus::Alive || world.status == PlayerStatus::Paused {
        handle_pressed_keys(&mut world);
        if world.status != PlayerStatus::Paused {
            physics(&mut world);
            world.draw(&sc)?;
        } else {
            pause_screen(&sc, &world);
        }
        thread::sleep(time::Duration::from_millis(slowness));
    }
    
    

    // game is finished
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    goodbye_screen(&sc, &world);
    sc.queue(Clear(crossterm::terminal::ClearType::All))?
        .execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}