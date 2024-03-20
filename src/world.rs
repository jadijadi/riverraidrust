use crossterm::{cursor::MoveTo, style::Print, terminal::Clear, QueueableCommand};

use std::{
    collections::VecDeque,
    io::{Stdout, Write},
};

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

pub struct Location {
    pub c: u16,
    pub l: u16,
}

impl Location {
    pub fn new(c: u16, l: u16) -> Location {
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

pub struct World {
    pub player_location: Location,
    pub map: VecDeque<(u16, u16)>,
    pub maxc: u16,
    pub maxl: u16,
    pub status: PlayerStatus,
    pub next_right: u16,
    pub next_left: u16,
    pub ship: String,
    pub enemy: Vec<Enemy>,
    pub fuel: Vec<Fuel>,
    pub bullet: Vec<Bullet>,
    pub gas: u16,
    pub score: u16,
    pub death_cause: DeathCause,
}

impl World {
    pub fn new(maxc: u16, maxl: u16) -> World {
        World {
            player_location: Location::new(maxc / 2, maxl - 1),
            map: VecDeque::from(vec![(maxc / 2 - 5, maxc / 2 + 5); maxl as usize]),
            maxc,
            maxl,
            status: PlayerStatus::Alive,
            next_left: maxc / 2 - 7,
            next_right: maxc / 2 + 7,
            ship: 'P'.to_string(),
            enemy: Vec::new(),
            bullet: Vec::new(),
            fuel: Vec::new(),
            score: 0,
            gas: 1700,
            death_cause: DeathCause::None,
        }
    }

    pub fn draw(&mut self, mut sc: &Stdout) -> std::io::Result<()> {
        sc.queue(Clear(crossterm::terminal::ClearType::All))?;

        // draw the map
        for l in 0..self.map.len() {
            sc.queue(MoveTo(0, l as u16))?
                .queue(Print("+".repeat(self.map[l].0 as usize)))?
                .queue(MoveTo(self.map[l].1, l as u16))?
                .queue(Print("+".repeat((self.maxc - self.map[l].1) as usize)))?;
        }

        sc.queue(MoveTo(2, 2))?
            .queue(Print(format!(" Score: {} ", self.score)))?
            .queue(MoveTo(2, 3))?
            .queue(Print(format!(" Fuel: {} ", self.gas / 100)))?;

        // draw fuel
        for index in (0..self.fuel.len()).rev() {
            match self.fuel[index].status {
                EnemyStatus::Alive => {
                    sc.queue(MoveTo(
                        self.fuel[index].location.c,
                        self.fuel[index].location.l,
                    ))?
                    .queue(Print("F"))?;
                }
                EnemyStatus::DeadBody => {
                    sc.queue(MoveTo(
                        self.fuel[index].location.c,
                        self.fuel[index].location.l,
                    ))?
                    .queue(Print("$"))?;
                    self.fuel[index].status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    self.fuel.remove(index);
                }
            };
        }

        // draw enemies
        for index in (0..self.enemy.len()).rev() {
            match self.enemy[index].status {
                EnemyStatus::Alive => {
                    sc.queue(MoveTo(
                        self.enemy[index].location.c,
                        self.enemy[index].location.l,
                    ))?
                    .queue(Print("E"))?;
                }
                EnemyStatus::DeadBody => {
                    sc.queue(MoveTo(
                        self.enemy[index].location.c,
                        self.enemy[index].location.l,
                    ))?
                    .queue(Print("X"))?;
                    self.enemy[index].status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    self.enemy.remove(index);
                }
            };
        }

        // draw bullet
        for b in &self.bullet {
            sc.queue(MoveTo(b.location.c, b.location.l))?
                .queue(Print("|"))?
                .queue(MoveTo(b.location.c, b.location.l - 1))?
                .queue(Print("^"))?;
        }

        // draw the player
        sc.queue(MoveTo(self.player_location.c, self.player_location.l))?
            .queue(Print(self.ship.as_str()))?
            .flush()?;

        Ok(())
    }
} // end of World implementation.
