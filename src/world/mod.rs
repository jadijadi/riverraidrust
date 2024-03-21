use std::{collections::VecDeque, io::Stdout, thread, time::Duration};

use rand::{rngs::ThreadRng, thread_rng};

use crate::{
    entities::{Bullet, Enemy, Fuel, Location, Player, PlayerStatus},
    handle_pressed_keys,
};

mod drawings;
mod physics;

pub struct World {
    pub player: Player,
    pub map: VecDeque<(u16, u16)>,
    pub maxc: u16,
    pub maxl: u16,
    pub next_right: u16,
    pub next_left: u16,
    pub enemies: Vec<Enemy>,
    pub fuels: Vec<Fuel>,
    pub bullets: Vec<Bullet>,
    pub rng: ThreadRng, // Local rng for the whole world
}

impl World {
    pub fn new(maxc: u16, maxl: u16) -> World {
        World {
            player: Player {
                location: Location::new(maxc / 2, maxl - 1),
                status: PlayerStatus::Alive,
                score: 0,
                gas: 1700,
            },
            map: VecDeque::from(vec![(maxc / 2 - 5, maxc / 2 + 5); maxl as usize]),
            maxc,
            maxl,
            next_left: maxc / 2 - 7,
            next_right: maxc / 2 + 7,
            enemies: Vec::new(),
            bullets: Vec::new(),
            fuels: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn game_loop(&mut self, stdout: &mut Stdout, slowness: u64) -> Result<(), std::io::Error> {
        while self.player.status == PlayerStatus::Alive
            || self.player.status == PlayerStatus::Paused
        {
            handle_pressed_keys(self);
            if self.player.status != PlayerStatus::Paused {
                self.physics();
                self.draw(stdout)?;
            } else {
                self.pause_screen(stdout)?;
            }
            thread::sleep(Duration::from_millis(slowness));
        }

        Ok(())
    }
} // end of World implementation.
