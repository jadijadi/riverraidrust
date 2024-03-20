use std::{
    collections::VecDeque,
    io::{Stdout, Write},
};

use crate::{drawable::Drawable, stout_ext::StdoutExt};

#[derive(PartialEq, Eq)]
pub enum PlayerStatus {
    Dead,
    Alive,
    Quit,
    Paused,
}

pub enum EnemyStatus {
    Alive,
    DeadBody,
    Dead,
}

pub enum DeathCause {
    None,
    Enemy,
    Ground,
    Fuel,
}

#[derive(Clone)]
pub struct Location {
    pub c: u16,
    pub l: u16,
}

impl Location {
    pub fn new(c: u16, l: u16) -> Self {
        Location { c, l }
    }

    // Checks if two locations are within a specified margin of each other
    pub fn hit_with_margin(
        &self,
        other: &Location,
        top: u16,
        right: u16,
        bottom: u16,
        left: u16,
    ) -> bool {
        (other.l > self.l || self.l - other.l <= bottom)
            && (self.l > other.l || other.l - self.l <= top)
            && (other.c > self.c || self.c - other.c <= left)
            && (self.c > other.c || other.c - self.c <= right)
    }

    // check if two locations is point to the same location
    pub fn hit(&self, other: &Location) -> bool {
        self.hit_with_margin(other, 0, 0, 0, 0)
    }
} // end of Location implementation.

pub struct Enemy {
    pub location: Location,
    pub status: EnemyStatus,
}

impl Enemy {
    pub fn new(column: u16, line: u16, status: EnemyStatus) -> Enemy {
        Enemy {
            location: Location::new(column, line),
            status,
        }
    }
} // end of Enemy implementation.

pub struct Bullet {
    pub location: Location,
    pub energy: u16,
}

impl Bullet {
    pub fn new(column: u16, line: u16, energy: u16) -> Bullet {
        Bullet {
            location: Location::new(column, line),
            energy,
        }
    }
} // end of Bullet implementation.

pub struct Fuel {
    pub location: Location,
    pub status: EnemyStatus,
}

impl Fuel {
    pub fn new(column: u16, line: u16, status: EnemyStatus) -> Fuel {
        Fuel {
            location: Location::new(column, line),
            status,
        }
    }
} // end of Fuel implementation.

pub struct Player {
    pub location: Location,
    pub status: PlayerStatus,
    pub gas: u16,
    pub score: u16,
    pub death_cause: DeathCause,
}

pub struct World {
    pub player: Player,
    pub map: VecDeque<(u16, u16)>,
    pub maxc: u16,
    pub maxl: u16,
    pub next_right: u16,
    pub next_left: u16,
    pub enemy: Vec<Enemy>,
    pub fuel: Vec<Fuel>,
    pub bullet: Vec<Bullet>,
}

impl World {
    pub fn new(maxc: u16, maxl: u16) -> World {
        World {
            player: Player {
                location: Location::new(maxc / 2, maxl - 1),
                status: PlayerStatus::Alive,
                score: 0,
                gas: 1700,
                death_cause: DeathCause::None,
            },
            map: VecDeque::from(vec![(maxc / 2 - 5, maxc / 2 + 5); maxl as usize]),
            maxc,
            maxl,
            next_left: maxc / 2 - 7,
            next_right: maxc / 2 + 7,
            enemy: Vec::new(),
            bullet: Vec::new(),
            fuel: Vec::new(),
        }
    }

    pub fn draw(&mut self, sc: &mut Stdout) -> std::io::Result<()> {
        sc.clear_all()?;

        // draw the map
        for l in 0..self.map.len() {
            sc.draw((0, l as u16), "+".repeat(self.map[l].0 as usize))?
                .draw(
                    (self.map[l].1, l as u16),
                    "+".repeat((self.maxc - self.map[l].1) as usize),
                )?;
        }

        sc.draw(2, format!(" Score: {} ", self.player.score))?
            .draw((2, 3), format!(" Fuel: {} ", self.player.gas / 100))?;

        // draw fuel
        self.fuel.retain_mut(|fuel| {
            match fuel.status {
                EnemyStatus::DeadBody => {
                    fuel.status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    return false;
                }
                _ => {}
            };
            let _ = fuel.draw(sc);
            true
        });

        // draw enemies
        self.enemy.retain_mut(|enemy| {
            match enemy.status {
                EnemyStatus::DeadBody => {
                    enemy.status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    return false;
                }
                _ => {}
            };
            let _ = enemy.draw(sc);
            true
        });

        // draw bullet
        for b in &self.bullet {
            b.draw(sc)?;
        }

        // draw the player
        self.player.draw(sc)?;

        // Flush everything to the screen.
        sc.flush()?;

        Ok(())
    }
} // end of World implementation.
