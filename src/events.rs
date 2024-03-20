use crossterm::event::{poll, read, Event, KeyCode};

use std::time::Duration;

use crate::{
    entities::{Bullet, PlayerStatus},
    world::World,
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
                        if world.player.status == PlayerStatus::Alive && world.bullets.is_empty() {
                            let new_bullet = Bullet::new(
                                world.player.location.c,
                                world.player.location.l - 1,
                                world.maxl / 4,
                            );
                            world.bullets.push(new_bullet);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
