use std::{
    collections::VecDeque,
    io::{Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::event::{poll, read};
use rand::thread_rng;

use crate::{
    drawable::Drawable,
    entities::{Bullet, DeathCause, Enemy, Fuel, Location, Player, PlayerStatus},
    handle_pressed_keys,
    physics::{self},
    stout_ext::StdoutExt,
};

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
        }
    }

    pub fn clear_screen<'a>(
        &'a self,
        stdout: &'a mut Stdout,
    ) -> Result<&mut Stdout, std::io::Error> {
        stdout.clear_all()
    }

    /// Game Physic Rules
    fn physics(&mut self) {
        let mut rng = thread_rng();

        // check if player hit the ground
        physics::check_player_status(self);

        // check enemy hit something
        physics::check_enemy_status(self);
        physics::check_fuel_status(self);

        // move the map Downward
        physics::update_map(&mut rng, self);

        // create new enemy
        physics::create_enemy(&mut rng, self);
        physics::create_fuel(&mut rng, self);

        // Move elements along map movements
        physics::move_enemies(self);
        physics::move_fuel(self);
        physics::move_bullets(self);

        if self.player.gas >= 1 {
            self.player.gas -= 1;
        }
    }

    fn draw(
        &self, // draw function should not change world's state
        stdout: &mut Stdout,
    ) -> std::io::Result<()> {
        self.clear_screen(stdout)?;

        // draw the map
        for l in 0..self.map.len() {
            stdout
                .draw((0, l as u16), "+".repeat(self.map[l].0 as usize))?
                .draw(
                    (self.map[l].1, l as u16),
                    "+".repeat((self.maxc - self.map[l].1) as usize),
                )?;
        }

        stdout
            .draw(2, format!(" Score: {} ", self.player.score))?
            .draw((2, 3), format!(" Fuel: {} ", self.player.gas / 100))?
            .draw((2, 4), format!(" Enemies: {} ", self.enemies.len()))?;

        // draw fuel
        for fuel in self.fuels.iter() {
            fuel.draw(stdout)?;
        }

        // draw enemies
        for enemy in self.enemies.iter() {
            enemy.draw(stdout)?;
        }

        // draw bullet
        for bullet in &self.bullets {
            bullet.draw(stdout)?;
        }

        // draw the player
        self.player.draw(stdout)?;

        // Flush everything to the screen.
        stdout.flush()?;

        Ok(())
    }

    fn pause_screen(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        let pause_msg1: &str = "╔═══════════╗";
        let pause_msg2: &str = "║Game Paused║";
        let pause_msg3: &str = "╚═══════════╝";

        stdout
            .draw((self.maxc / 2 - 6, self.maxl / 2 - 1), pause_msg1)?
            .draw((self.maxc / 2 - 6, self.maxl / 2), pause_msg2)?
            .draw((self.maxc / 2 - 6, self.maxl / 2 + 1), pause_msg3)?
            .flush()
    }

    pub fn welcome_screen(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        let welcome_msg: &str = "██████╗ ██╗██╗   ██╗███████╗██████╗ ██████╗  █████╗ ██╗██████╗     ██████╗ ██╗   ██╗███████╗████████╗\n\r██╔══██╗██║██║   ██║██╔════╝██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝\n\r██████╔╝██║██║   ██║█████╗  ██████╔╝██████╔╝███████║██║██║  ██║    ██████╔╝██║   ██║███████╗   ██║   \n\r██╔══██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║  ██║    ██╔══██╗██║   ██║╚════██║   ██║   \n\r██║  ██║██║ ╚████╔╝ ███████╗██║  ██║██║  ██║██║  ██║██║██████╔╝    ██║  ██║╚██████╔╝███████║   ██║   \n\r╚═╝  ╚═╝╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═════╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   \n";
        self.clear_screen(stdout)?;

        if self.maxc > 100 {
            stdout.draw((0, 2), welcome_msg)?;
        } else {
            stdout.draw((0, 2), "RiverRaid Rust")?;
        }

        stdout.draw((2, self.maxl - 2), "Press any key to continue...")?;
        stdout.flush()?;

        loop {
            if poll(Duration::from_millis(0)).unwrap() {
                read()?;
                break;
            }
        }
        self.clear_screen(stdout)?;

        Ok(())
    }

    pub fn goodbye_screen(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        let goodbye_msg1: &str = " ██████╗  ██████╗  ██████╗ ██████╗      ██████╗  █████╗ ███╗   ███╗███████╗██╗\n\r██╔════╝ ██╔═══██╗██╔═══██╗██╔══██╗    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██║\n\r██║  ███╗██║   ██║██║   ██║██║  ██║    ██║  ███╗███████║██╔████╔██║█████╗  ██║\n\r██║   ██║██║   ██║██║   ██║██║  ██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ╚═╝\n\r╚██████╔╝╚██████╔╝╚██████╔╝██████╔╝    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗██╗\n\r ╚═════╝  ╚═════╝  ╚═════╝ ╚═════╝      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝\n";
        let goodbye_msg2: &str = "████████╗██╗  ██╗ █████╗ ███╗   ██╗██╗  ██╗███████╗\n\r╚══██╔══╝██║  ██║██╔══██╗████╗  ██║██║ ██╔╝██╔════╝\n\r   ██║   ███████║███████║██╔██╗ ██║█████╔╝ ███████╗\n\r   ██║   ██╔══██║██╔══██║██║╚██╗██║██╔═██╗ ╚════██║\n\r   ██║   ██║  ██║██║  ██║██║ ╚████║██║  ██╗███████║██╗\n\r   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝\n";

        self.clear_screen(stdout)?
            .draw((0, 2), goodbye_msg1)?
            .draw((0, 10), goodbye_msg2)?;

        stdout.move_cursor((2, self.maxl - 5))?;
        if let PlayerStatus::Dead(cause) = &self.player.status {
            match cause {
                DeathCause::Ground => {
                    if self.maxc > 91 {
                        stdout.print("\r█▄█ █▀█ █░█   █▀▀ █▀█ ▄▀█ █▀ █░█ █▀▀ █▀▄   █ █▄░█   ▀█▀ █░█ █▀▀   █▀▀ █▀█ █▀█ █░█ █▄░█ █▀▄ ░\n\r░█░ █▄█ █▄█   █▄▄ █▀▄ █▀█ ▄█ █▀█ ██▄ █▄▀   █ █░▀█   ░█░ █▀█ ██▄   █▄█ █▀▄ █▄█ █▄█ █░▀█ █▄▀ ▄\n\r")?;
                    } else {
                        stdout.print("You crashed in the ground.")?;
                    }
                }
                DeathCause::Enemy => {
                    if self.maxc > 72 {
                        stdout.print("\r▄▀█ █▄░█   █▀▀ █▄░█ █▀▀ █▀▄▀█ █▄█   █▄▀ █ █░░ █░░ █▀▀ █▀▄   █▄█ █▀█ █░█ ░\n\r█▀█ █░▀█   ██▄ █░▀█ ██▄ █░▀░█ ░█░   █░█ █ █▄▄ █▄▄ ██▄ █▄▀   ░█░ █▄█ █▄█ ▄\n\r")?;
                    } else {
                        stdout.print("An enemy killed you.")?;
                    }
                }
                DeathCause::Fuel => {
                    if self.maxc > 69 {
                        stdout.print("\r█▄█ █▀█ █░█   █▀█ ▄▀█ █▄░█   █▀█ █░█ ▀█▀   █▀█ █▀▀   █▀▀ █░█ █▀▀ █░░ ░\n\r░█░ █▄█ █▄█   █▀▄ █▀█ █░▀█   █▄█ █▄█ ░█░   █▄█ █▀░   █▀░ █▄█ ██▄ █▄▄ ▄\n\r")?;
                    } else {
                        stdout.print("You ran out of fuel.")?;
                    }
                }
            }
        } else {
            unreachable!("Undead player has no death cause!")
        }

        stdout.move_cursor((2, self.maxl - 2))?;
        thread::sleep(Duration::from_millis(2000));
        stdout.print("Press any key to continue...")?;
        stdout.flush()?;
        loop {
            if poll(Duration::from_millis(0)).unwrap() {
                read()?;
                break;
            }
        }

        self.clear_screen(stdout)?;
        Ok(())
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
