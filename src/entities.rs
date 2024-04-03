#[derive(PartialEq, Eq)]
pub enum DeathCause {
    Enemy,
    Ground,
    Fuel,
}

#[derive(PartialEq, Eq)]
pub enum PlayerStatus {
    Dead(DeathCause),
    Alive,
    Quit,
}

pub enum EntityStatus {
    Alive,
    DeadBody,
    Dead,
}

#[derive(PartialEq, Eq)]
pub enum GameMode {
    Normal,
    God,
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
    pub status: EntityStatus,
}

impl Enemy {
    pub fn new(column: u16, line: u16, status: EntityStatus) -> Enemy {
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
    pub status: EntityStatus,
}

impl Fuel {
    pub fn new(column: u16, line: u16, status: EntityStatus) -> Fuel {
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
}
