use std::{collections::VecDeque, io::Stdout, thread, time::Duration};
use std::time::SystemTime;

use rand::{rngs::ThreadRng, thread_rng};

use crate::{
    canvas::Canvas,
    entities::{Bullet, Enemy, Fuel, Location, Player, PlayerStatus},
    handle_pressed_keys,
};

mod drawings;
mod physics;

pub enum WorldStatus {
    Fluent,
    Paused,
}

pub struct World {
    canvas: Canvas,
    pub status: WorldStatus,
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

    pub start: SystemTime,
    pub last_pause_time: SystemTime,
    pub pause_seconds: u64,
}

impl World {
    pub fn new(maxc: u16, maxl: u16) -> World {
        let start_time = SystemTime::now();

        World {
            status: WorldStatus::Fluent,
            canvas: Canvas::new(maxc, maxl),
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

            start: start_time,
            last_pause_time: start_time,
            pause_seconds: 0,
        }
    }

    pub fn game_loop(&mut self, stdout: &mut Stdout, slowness: u64) -> Result<(), std::io::Error> {
        while self.player.status == PlayerStatus::Alive {
            handle_pressed_keys(self);
            match self.status {
                WorldStatus::Fluent => {
                    self.physics();
                    self.draw_on_canvas();
                }
                WorldStatus::Paused => self.pause_screen(),
            }

            self.canvas.draw_map(stdout)?;
            thread::sleep(Duration::from_millis(slowness));
        }

        Ok(())
    }
} // end of World implementation.
