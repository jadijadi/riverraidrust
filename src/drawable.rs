use std::io::Stdout;

use crate::{
    entities::{Bullet, Enemy, EntityStatus, Fuel, Player},
    stout_ext::StdoutExt,
};

pub trait Drawable {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error>;
}

impl Drawable for Enemy {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        match self.status {
            EntityStatus::Alive => {
                sc.draw(self, "E")?;
            }
            EntityStatus::DeadBody => {
                sc.draw(self, "X")?;
            }
            EntityStatus::Dead => {}
        };

        Ok(())
    }
}

impl Drawable for Fuel {
    fn draw(&self, sc: &mut Stdout) -> Result<(), std::io::Error> {
        match self.status {
            EntityStatus::Alive => {
                sc.draw(self, "F")?;
            }
            EntityStatus::DeadBody => {
                sc.draw(self, "$")?;
            }
            EntityStatus::Dead => {}
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
