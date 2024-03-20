use std::io::Stdout;

use crate::{stout_ext::StdoutExt, Bullet, Enemy, EnemyStatus, Fuel, Player};

pub trait Drawable {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error>;
}

impl Drawable for Enemy {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        match self.status {
            EnemyStatus::Alive => {
                sc.draw(self, "E")?;
            }
            EnemyStatus::DeadBody => {
                sc.draw(self, "X")?;
            }
            EnemyStatus::Dead => {}
        };

        Ok(())
    }
}

impl Drawable for Fuel {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        match self.status {
            EnemyStatus::Alive => {
                sc.draw(self, "F")?;
            }
            EnemyStatus::DeadBody => {
                sc.draw(self, "$")?;
            }
            EnemyStatus::Dead => {}
        };

        Ok(())
    }
}

impl Drawable for Bullet {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        sc.draw(self, "|")?
            .draw((self.location.c, self.location.l - 1), "^")?;
        Ok(())
    }
}

impl Drawable for Player {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        sc.draw(self, "P")?;
        Ok(())
    }
}
