pub mod draw {  
    use crossterm::{
        cursor::MoveTo,
        style::Print,
        terminal::Clear,
        QueueableCommand
    };

    use std::{
        collections::VecDeque,
        io::{Stdout, Write},
    };

    use crate::world::world::{
        World, 
        PlayerStatus, 
        Enemy, 
        EnemyStatus, 
        DeathCause, 
        Fuel
    };

    // draw the map
    pub fn draw_map(world: &mut World, mut sc: &Stdout) -> std::io::Result<()>  {
        for l in 0..world.map.len() {
            sc.queue(MoveTo(0, l as u16))?
                .queue(Print("+".repeat(world.map[l].0 as usize)))?
                .queue(MoveTo(world.map[l].1, l as u16))?
                .queue(Print("+".repeat((world.maxc - world.map[l].1) as usize)))?;
        }
    
        sc.queue(MoveTo(2, 2))?
            .queue(Print(format!(" Score: {} ", world.score)))?
            .queue(MoveTo(2, 3))?
            .queue(Print(format!(" Fuel: {} ", world.gas / 100)))?;
        
        Ok(())
    }    
    
    // draw fuel
    pub fn draw_fuel(world: &mut World, mut sc: &Stdout) -> std::io::Result<()> {
        for index in (0..world.fuel.len()).rev() {
            match world.fuel[index].status {
                EnemyStatus::Alive => {
                    sc.queue(MoveTo(
                        world.fuel[index].location.c,
                        world.fuel[index].location.l,
                    ))?
                    .queue(Print("F"))?;
                }
                EnemyStatus::DeadBody => {
                    sc.queue(MoveTo(
                        world.fuel[index].location.c,
                        world.fuel[index].location.l,
                    ))?
                    .queue(Print("$"))?;
                    world.fuel[index].status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    world.fuel.remove(index);
                }
            };
        }

        Ok(())
    }
    
    // draw enemies
    pub fn draw_enemies(world: &mut World, mut sc: &Stdout) -> std::io::Result<()>  {
        for index in (0..world.enemy.len()).rev() {
            match world.enemy[index].status {
                EnemyStatus::Alive => {
                    sc.queue(MoveTo(
                        world.enemy[index].location.c,
                        world.enemy[index].location.l,
                    ))?
                    .queue(Print("E"))?;
                }
                EnemyStatus::DeadBody => {
                    sc.queue(MoveTo(
                        world.enemy[index].location.c,
                        world.enemy[index].location.l,
                    ))?
                    .queue(Print("X"))?;
                    world.enemy[index].status = EnemyStatus::Dead;
                }
                EnemyStatus::Dead => {
                    world.enemy.remove(index);
                }
            };
        }
        
        Ok(())
    }

    
    // draw bullet
    pub fn draw_bullets(world : &mut World, mut sc: &Stdout) -> std::io::Result<()>  {
        for b in &world.bullet {
            sc.queue(MoveTo(b.location.c, b.location.l))?
                .queue(Print("|"))?
                .queue(MoveTo(b.location.c, b.location.l - 1))?
                .queue(Print("^"))?;
        }
        Ok(())
    }
    
    // draw the player
    pub fn draw_player(world : &mut World , mut sc: &Stdout) -> std::io::Result<()> {
        sc.queue(MoveTo(world.player_location.c, world.player_location.l))?
            .queue(Print(world.ship.as_str()))?
            .flush()?;
        Ok(())
    }
}
