pub mod events {

    use crate::world::world::{World, PlayerStatus, Bullet};

    use crossterm::{
        event::{poll, read, Event, KeyCode},
        cursor::MoveTo,
        style::Print,
        QueueableCommand
    };

    use std::{
        io::{Stdout, Write},
        time::Duration,
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
                        KeyCode::Char('q') => world.status = PlayerStatus::Quit,
                
                        KeyCode::Up | KeyCode::Char('w') => {
                            if world.status == PlayerStatus::Alive && world.player_location.l > 1 {
                                world.player_location.l -= 1
                            }
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            if world.status == PlayerStatus::Alive
                                && world.player_location.l < world.maxl - 1
                            {
                                world.player_location.l += 1
                            }
                        }
                        KeyCode::Left | KeyCode::Char('a') => {
                            if world.status == PlayerStatus::Alive && world.player_location.c > 1 {
                                world.player_location.c -= 1
                            }
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            if world.status == PlayerStatus::Alive
                                && world.player_location.c < world.maxc - 1
                            {
                                world.player_location.c += 1
                            }
                        }
                        KeyCode::Char('p') => {
                            if world.status == PlayerStatus::Alive {
                                world.status = PlayerStatus::Paused;
                            } else if world.status == PlayerStatus::Paused {
                                world.status = PlayerStatus::Alive;
                            }
                        }
                        KeyCode::Char(' ') => {
                            if world.status == PlayerStatus::Alive && world.bullet.is_empty() {
                                let new_bullet = Bullet::new(
                                    world.player_location.c,
                                    world.player_location.l - 1,
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

    pub fn pause_screen(mut sc: &Stdout , world: &World) {
        let pause_msg1: &str = "╔═══════════╗";
        let pause_msg2: &str = "║Game Paused║";
        let pause_msg3: &str = "╚═══════════╝";
        let _ = sc.queue(MoveTo(world.maxc / 2 - 6, world.maxl / 2 - 1));
        let _ = sc.queue(Print(pause_msg1));
        let _ = sc.queue(MoveTo(world.maxc / 2 - 6, world.maxl / 2));
        let _ = sc.queue(Print(pause_msg2));
        let _ = sc.queue(MoveTo(world.maxc / 2 - 6, world.maxl / 2 + 1));
        let _ = sc.queue(Print(pause_msg3));
        let _ = sc.flush();
    }
    
}
