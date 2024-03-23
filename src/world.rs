pub mod world {
    
    use std::collections::VecDeque;

    #[derive(PartialEq, Eq)]
    pub enum PlayerStatus {
        Dead,
        Alive,
        Animation,
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
            buttom: u16,
            left: u16,
        ) -> bool {
            (other.l > self.l || self.l - other.l <= buttom)
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
                status: status,
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
                status: status,
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
    } // end of World implementation.
}