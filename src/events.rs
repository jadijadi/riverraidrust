use crossterm::event::{poll, read, Event, KeyCode};

use std::{
    io::{Stdout, Write},
    time::Duration,
};

use crate::{
    stout_ext::StdoutExt,
    world::{Bullet, PlayerStatus, World},
};

pub fn handle_pressed_keys(world: &mut World) {
    if poll(Duration::from_millis(10)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        match key {
            Event::Key(event) => {
                // I'm reading from keyboard into event
                match event.code {
                    KeyCode::Char('q') => world.player.status = PlayerStatus::Quit,
                    KeyCode::Char('w') => {
                        if world.player.status == PlayerStatus::Alive && world.player.location.l > 1
                        {
                            world.player.location.l -= 1
                        }
                    }
                    KeyCode::Char('s') => {
                        if world.player.status == PlayerStatus::Alive
                            && world.player.location.l < world.maxl - 1
                        {
                            world.player.location.l += 1
                        }
                    }
                    KeyCode::Char('a') => {
                        if world.player.status == PlayerStatus::Alive && world.player.location.c > 1
                        {
                            world.player.location.c -= 1
                        }
                    }
                    KeyCode::Char('d') => {
                        if world.player.status == PlayerStatus::Alive
                            && world.player.location.c < world.maxc - 1
                        {
                            world.player.location.c += 1
                        }
                    }
                    KeyCode::Up => {
                        if world.player.status == PlayerStatus::Alive && world.player.location.l > 1
                        {
                            world.player.location.l -= 1
                        }
                    }
                    KeyCode::Down => {
                        if world.player.status == PlayerStatus::Alive
                            && world.player.location.l < world.maxl - 1
                        {
                            world.player.location.l += 1
                        }
                    }
                    KeyCode::Left => {
                        if world.player.status == PlayerStatus::Alive && world.player.location.c > 1
                        {
                            world.player.location.c -= 1
                        }
                    }
                    KeyCode::Right => {
                        if world.player.status == PlayerStatus::Alive
                            && world.player.location.c < world.maxc - 1
                        {
                            world.player.location.c += 1
                        }
                    }
                    KeyCode::Char('p') => {
                        if world.player.status == PlayerStatus::Alive {
                            world.player.status = PlayerStatus::Paused;
                        } else if world.player.status == PlayerStatus::Paused {
                            world.player.status = PlayerStatus::Alive;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if world.player.status == PlayerStatus::Alive && world.bullet.is_empty() {
                            let new_bullet = Bullet::new(
                                world.player.location.c,
                                world.player.location.l - 1,
                                world.maxl / 4,
                            );
                            world.bullet.push(new_bullet);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn pause_screen(sc: &mut Stdout, world: &World) -> Result<(), std::io::Error> {
    let pause_msg1: &str = "╔═══════════╗";
    let pause_msg2: &str = "║Game Paused║";
    let pause_msg3: &str = "╚═══════════╝";

    sc.draw((world.maxc / 2 - 6, world.maxl / 2 - 1), pause_msg1)?
        .draw((world.maxc / 2 - 6, world.maxl / 2), pause_msg2)?
        .draw((world.maxc / 2 - 6, world.maxl / 2 + 1), pause_msg3)?
        .flush()
}
