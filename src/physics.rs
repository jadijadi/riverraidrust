use rand::Rng;
use std::num::Wrapping;

use crate::{
    entities::{DeathCause, Enemy, EntityStatus, Fuel, PlayerStatus},
    world::World,
};

/// check if player hit the ground
pub fn check_player_status(world: &mut World) {
    if world.player.location.c < world.map[world.player.location.l as usize].0
        || world.player.location.c >= world.map[world.player.location.l as usize].1
    {
        world.player.status = PlayerStatus::Dead(DeathCause::Ground);
    }

    if world.player.gas == 0 {
        world.player.status = PlayerStatus::Dead(DeathCause::Fuel);
    }
}

/// check enemy hit something
pub fn check_enemy_status(world: &mut World) {
    // Remove dead
    world
        .enemies
        .retain(|f| !matches!(f.status, EntityStatus::Dead));

    for enemy in world.enemies.iter_mut().rev() {
        match enemy.status {
            EntityStatus::Alive if world.player.location.hit(&enemy.location) => {
                world.player.status = PlayerStatus::Dead(DeathCause::Enemy);
            }
            EntityStatus::DeadBody => {
                enemy.status = EntityStatus::Dead;
            }
            _ => {}
        }

        for bullet in world.bullets.iter().rev() {
            if bullet.location.hit_with_margin(&enemy.location, 1, 0, 1, 0) {
                enemy.status = EntityStatus::DeadBody;
                world.player.score += 10;
            }
        }
    }
}

/// Update the map
pub fn update_map(world: &mut World) {
    use std::cmp::Ordering::*;

    // move the map downward using VecDeque
    world.map.pop_back();
    let (mut left, mut right) = world.map[0];
    match world.next_left.cmp(&left) {
        Greater => left += 1,
        Less => left -= 1,
        Equal => {}
    };

    match world.next_right.cmp(&right) {
        Greater => right += 1,
        Less => right -= 1,
        Equal => {}
    };

    if world.next_left == world.map[0].0 && world.rng.gen_range(0..10) >= 7 {
        world.next_left = world
            .rng
            .gen_range(world.next_left.saturating_sub(5)..world.next_left + 5);
        if world.next_left == 0 {
            world.next_left = 1;
        }
    }

    if world.next_right == world.map[0].1 && world.rng.gen_range(0..10) >= 7 {
        world.next_right = world
            .rng
            .gen_range(world.next_right - 5..world.next_right + 5);
        if world.next_right > world.maxc {
            world.next_right = Wrapping(world.maxc).0 - 1;
        }
    }

    if world.next_right.abs_diff(world.next_left) < 3 {
        world.next_right += 3;
    }
    world.map.push_front((left, right))
}

/// Move enemies on the river
pub fn move_enemies(world: &mut World) {
    world.enemies.retain_mut(|enemy| {
        enemy.location.l += 1;
        // Retain enemies within the screen
        enemy.location.l < world.maxl
    });
}

/// Move Bullets
pub fn move_bullets(world: &mut World) {
    for index in (0..world.bullets.len()).rev() {
        if world.bullets[index].energy == 0 || world.bullets[index].location.l <= 2 {
            world.bullets.remove(index);
        } else {
            world.bullets[index].location.l -= 2;
            world.bullets[index].energy -= 1;

            if world.bullets[index].location.c
                < world.map[world.bullets[index].location.l as usize].0
                || world.bullets[index].location.c
                    >= world.map[world.bullets[index].location.l as usize].1
            {
                world.bullets.remove(index);
            }
        }
    }
}

/// check if fuel is hit / moved over
pub fn check_fuel_status(world: &mut World) {
    // Remove dead
    world
        .fuels
        .retain(|f| !matches!(f.status, EntityStatus::Dead));

    for fuel in world.fuels.iter_mut().rev() {
        match fuel.status {
            EntityStatus::Alive if world.player.location.hit(&fuel.location) => {
                fuel.status = EntityStatus::DeadBody;
                world.player.gas += 200;
            }
            EntityStatus::DeadBody => {
                fuel.status = EntityStatus::Dead;
            }
            _ => {}
        }

        for bullet in world.bullets.iter().rev() {
            if bullet.location.hit_with_margin(&fuel.location, 1, 0, 1, 0) {
                fuel.status = EntityStatus::DeadBody;
                world.player.score += 20;
            }
        }
    }
}

/// Create a new fuel; maybe
pub fn create_fuel(world: &mut World) {
    // Possibility
    if world.rng.gen_range(0..100) >= 99 {
        world.fuels.push(Fuel::new(
            world.rng.gen_range(world.map[0].0..world.map[0].1),
            0,
            EntityStatus::Alive,
        ));
    }
}

/// Create a new enemy
pub fn create_enemy(world: &mut World) {
    // Possibility
    if world.rng.gen_range(0..10) >= 9 {
        world.enemies.push(Enemy::new(
            world.rng.gen_range(world.map[0].0..world.map[0].1),
            0,
            EntityStatus::Alive,
        ));
    }
}

/// Move fuels on the river
pub fn move_fuel(world: &mut World) {
    world.fuels.retain_mut(|fuel| {
        fuel.location.l += 1;
        // Retain fuels within the screen
        fuel.location.l < world.maxl
    });
}
