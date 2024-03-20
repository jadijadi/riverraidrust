use rand::{rngs::ThreadRng, Rng};
use std::num::Wrapping;

use crate::world::{DeathCause, Enemy, EnemyStatus, Fuel, PlayerStatus, World};

/// check if player hit the ground
pub fn check_player_status(world: &mut World) {
    if world.player.location.c < world.map[world.player.location.l as usize].0
        || world.player.location.c >= world.map[world.player.location.l as usize].1
    {
        world.player.status = PlayerStatus::Dead;
        world.player.death_cause = DeathCause::Ground;
    }

    if world.player.gas == 0 {
        world.player.status = PlayerStatus::Dead;
        world.player.death_cause = DeathCause::Fuel;
    }
}

/// check enemy hit something
pub fn check_enemy_status(world: &mut World) {
    for index in (0..world.enemy.len()).rev() {
        if matches!(world.enemy[index].status, EnemyStatus::Alive)
            && world.player.location.hit(&world.enemy[index].location)
        {
            world.player.status = PlayerStatus::Dead;
            world.player.death_cause = DeathCause::Enemy;
        };
        for j in (0..world.bullet.len()).rev() {
            if world.bullet[j]
                .location
                .hit_with_margin(&world.enemy[index].location, 1, 0, 1, 0)
            {
                world.enemy[index].status = EnemyStatus::DeadBody;
                world.player.score += 10;
            }
        }
    }
}

/// Update the map
pub fn update_map(rng: &mut ThreadRng, world: &mut World) {
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

    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7 {
        world.next_left = rng.gen_range(world.next_left.saturating_sub(5)..world.next_left + 5);
        if world.next_left == 0 {
            world.next_left = 1;
        }
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7 {
        world.next_right = rng.gen_range(world.next_right - 5..world.next_right + 5);
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
    for index in (0..world.enemy.len()).rev() {
        world.enemy[index].location.l += 1;
        if world.enemy[index].location.l >= world.maxl {
            world.enemy.remove(index);
        }
    }
}

/// Move Bullets
pub fn move_bullets(world: &mut World) {
    for index in (0..world.bullet.len()).rev() {
        if world.bullet[index].energy == 0 || world.bullet[index].location.l <= 2 {
            world.bullet.remove(index);
        } else {
            world.bullet[index].location.l -= 2;
            world.bullet[index].energy -= 1;

            if world.bullet[index].location.c < world.map[world.bullet[index].location.l as usize].0
                || world.bullet[index].location.c
                    >= world.map[world.bullet[index].location.l as usize].1
            {
                world.bullet.remove(index);
            }
        }
    }
}

/// check if fuel is hit / moved over
pub fn check_fuel_status(world: &mut World) {
    for index in (0..world.fuel.len()).rev() {
        if matches!(world.fuel[index].status, EnemyStatus::Alive)
            && world.player.location.hit(&world.fuel[index].location)
        {
            world.fuel[index].status = EnemyStatus::DeadBody;
            world.player.gas += 200;
        };
        for j in (0..world.bullet.len()).rev() {
            if world.bullet[j]
                .location
                .hit_with_margin(&world.fuel[index].location, 1, 0, 1, 0)
            {
                world.fuel[index].status = EnemyStatus::DeadBody;
                world.player.score += 20;
            }
        }
    }
}

/// Create a new fuel; maybe
pub fn create_fuel(rng: &mut ThreadRng, world: &mut World) {
    // Possibility
    if rng.gen_range(0..100) >= 99 {
        world.fuel.push(Fuel::new(
            rng.gen_range(world.map[0].0..world.map[0].1),
            0,
            EnemyStatus::Alive,
        ));
    }
}

/// Create a new enemy
pub fn create_enemy(rng: &mut ThreadRng, world: &mut World) {
    // Possibility
    if rng.gen_range(0..10) >= 9 {
        world.enemy.push(Enemy::new(
            rng.gen_range(world.map[0].0..world.map[0].1),
            0,
            EnemyStatus::Alive,
        ));
    }
}

/// Move fuels on the river
pub fn move_fuel(world: &mut World) {
    for index in (0..world.fuel.len()).rev() {
        world.fuel[index].location.l += 1;
        if world.fuel[index].location.l >= world.maxl {
            world.fuel.remove(index);
        }
    }
}
