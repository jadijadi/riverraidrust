use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::num::Wrapping;
use std::{
    cmp::Ordering::*,
    collections::VecDeque,
    io::{stdout, Stdout, Write},
    time::Duration,
};
use std::{thread, time};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand, QueueableCommand,
};

#[derive(PartialEq, Eq)]
enum PlayerStatus {
    Dead,
    Alive,
    Animation,
    Quit,
    Paused,
}

enum EnemyStatus {
    Alive,
    DeadBody,
    Dead,
}

enum DeathCause {
    None,
    Enemy,
    Ground,
    Fuel,
}

struct Location {
    c: u16,
    l: u16,
}
struct Enemy {
    location: Location,
    status: EnemyStatus,
}

struct Fuel {
    location: Location,
    status: EnemyStatus,
}

impl Location {
    fn new(c: u16, l: u16) -> Location {
        Location { c, l }
    }

    // Checks if two locations are within a specified margin of each other
    fn hit_with_margin(
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
    fn hit(&self, other: &Location) -> bool {
        self.hit_with_margin(other, 0, 0, 0, 0)
    }
}

impl Fuel {
    fn new(column: u16, line: u16, status: EnemyStatus) -> Fuel {
        Fuel {
            location: Location::new(column, line),
            status: status,
        }
    }
}

impl Enemy {
    fn new(column: u16, line: u16, status: EnemyStatus) -> Enemy {
        Enemy {
            location: Location::new(column, line),
            status: status,
        }
    }
}

struct Bullet {
    location: Location,
    energy: u16,
}

impl Bullet {
    fn new(column: u16, line: u16, energy: u16) -> Bullet {
        Bullet {
            location: Location::new(column, line),
            energy,
        }
    }
}

struct World {
    player_location: Location,
    map: VecDeque<(u16, u16)>,
    maxc: u16,
    maxl: u16,
    status: PlayerStatus,
    next_right: u16,
    next_left: u16,
    ship: String,
    enemy: Vec<Enemy>,
    fuel: Vec<Fuel>,
    bullet: Vec<Bullet>,
    gas: u16,
    score: u16,
    death_cause: DeathCause,
}

impl World {
    fn new(maxc: u16, maxl: u16) -> World {
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
}

fn draw(mut sc: &Stdout, world: &mut World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;

    // draw the map
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

    // draw fuel
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

    // draw enemies
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

    // draw bullet
    for b in &world.bullet {
        sc.queue(MoveTo(b.location.c, b.location.l))?
            .queue(Print("|"))?
            .queue(MoveTo(b.location.c, b.location.l - 1))?
            .queue(Print("^"))?;
    }

    // draw the player
    sc.queue(MoveTo(world.player_location.c, world.player_location.l))?
        .queue(Print(world.ship.as_str()))?
        .flush()?;

    Ok(())
}

/// check if player hit the ground
fn check_player_status(world: &mut World) {
    if world.player_location.c < world.map[world.player_location.l as usize].0
        || world.player_location.c >= world.map[world.player_location.l as usize].1
    {
        world.status = PlayerStatus::Dead;
        world.death_cause = DeathCause::Ground;
    }

    if world.gas == 0 {
        world.status = PlayerStatus::Dead;
        world.death_cause = DeathCause::Fuel;
    }
}

/// check if fuel is hit / moved over
fn check_fuel_status(world: &mut World) {
    for index in (0..world.fuel.len()).rev() {
        if matches!(world.fuel[index].status, EnemyStatus::Alive)
            && world.player_location.hit(&world.fuel[index].location)
        {
            world.gas += 200;
        };
        for j in (0..world.bullet.len()).rev() {
            if world.bullet[j]
                .location
                .hit_with_margin(&world.fuel[index].location, 1, 0, 1, 0)
            {
                world.fuel[index].status = EnemyStatus::DeadBody;
                world.score += 20;
            }
        }
    }
}

/// check enemy hit something
fn check_enemy_status(world: &mut World) {
    for index in (0..world.enemy.len()).rev() {
        if matches!(world.enemy[index].status, EnemyStatus::Alive)
            && world.player_location.hit(&world.enemy[index].location)
        {
            world.status = PlayerStatus::Dead;
            world.death_cause = DeathCause::Enemy;
        };
        for j in (0..world.bullet.len()).rev() {
            if world.bullet[j]
                .location
                .hit_with_margin(&world.enemy[index].location, 1, 0, 1, 0)
            {
                world.enemy[index].status = EnemyStatus::DeadBody;
                world.score += 10;
            }
        }
    }
}

/// Update the map
fn update_map(rng: &mut ThreadRng, world: &mut World) {
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

/// Create a new fuel; maybe
fn create_fuel(rng: &mut ThreadRng, world: &mut World) {
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
fn create_enemy(rng: &mut ThreadRng, world: &mut World) {
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
fn move_fuel(world: &mut World) {
    for index in (0..world.fuel.len()).rev() {
        world.fuel[index].location.l += 1;
        if world.fuel[index].location.l >= world.maxl {
            world.fuel.remove(index);
        }
    }
}

/// Move enemies on the river
fn move_enemies(world: &mut World) {
    for index in (0..world.enemy.len()).rev() {
        world.enemy[index].location.l += 1;
        if world.enemy[index].location.l >= world.maxl {
            world.enemy.remove(index);
        }
    }
}

/// Move Bullets
fn move_bullets(world: &mut World) {
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

fn welcome_screen(mut sc: &Stdout, world: &World) {
    let welcome_msg: &str = "██████╗ ██╗██╗   ██╗███████╗██████╗ ██████╗  █████╗ ██╗██████╗     ██████╗ ██╗   ██╗███████╗████████╗\n\r██╔══██╗██║██║   ██║██╔════╝██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝\n\r██████╔╝██║██║   ██║█████╗  ██████╔╝██████╔╝███████║██║██║  ██║    ██████╔╝██║   ██║███████╗   ██║   \n\r██╔══██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║  ██║    ██╔══██╗██║   ██║╚════██║   ██║   \n\r██║  ██║██║ ╚████╔╝ ███████╗██║  ██║██║  ██║██║  ██║██║██████╔╝    ██║  ██║╚██████╔╝███████║   ██║   \n\r╚═╝  ╚═╝╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═════╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   \n";
    let _ = sc.queue(Clear(crossterm::terminal::ClearType::All));
    if world.maxc > 100 {
        let _ = sc.queue(MoveTo(0, 2));
        let _ = sc.queue(Print(welcome_msg));
    }
    else {
        let _ = sc.queue(MoveTo(0, 2));
        let _ = sc.queue(Print("RiverRaid Rust"));
    }
    let _ = sc.queue(MoveTo(2, world.maxl -2));

    let _ = sc.queue(Print("Press any key to continue..."));
    let _ = sc.flush();
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
            break;
        }
    }
    let _ = sc.queue(Clear(crossterm::terminal::ClearType::All));
}

fn goodbye_screen(mut sc: &Stdout, world: &World) {
    let goodbye_msg1: &str = " ██████╗  ██████╗  ██████╗ ██████╗      ██████╗  █████╗ ███╗   ███╗███████╗██╗\n\r██╔════╝ ██╔═══██╗██╔═══██╗██╔══██╗    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██║\n\r██║  ███╗██║   ██║██║   ██║██║  ██║    ██║  ███╗███████║██╔████╔██║█████╗  ██║\n\r██║   ██║██║   ██║██║   ██║██║  ██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ╚═╝\n\r╚██████╔╝╚██████╔╝╚██████╔╝██████╔╝    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗██╗\n\r ╚═════╝  ╚═════╝  ╚═════╝ ╚═════╝      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝\n";
    let goodbye_msg2: &str = "████████╗██╗  ██╗ █████╗ ███╗   ██╗██╗  ██╗███████╗\n\r╚══██╔══╝██║  ██║██╔══██╗████╗  ██║██║ ██╔╝██╔════╝\n\r   ██║   ███████║███████║██╔██╗ ██║█████╔╝ ███████╗\n\r   ██║   ██╔══██║██╔══██║██║╚██╗██║██╔═██╗ ╚════██║\n\r   ██║   ██║  ██║██║  ██║██║ ╚████║██║  ██╗███████║██╗\n\r   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝\n";
    let _ = sc.queue(Clear(crossterm::terminal::ClearType::All));

    let _ = sc.queue(MoveTo(0, 2));
    let _ = sc.queue(Print(goodbye_msg1));
    let _ = sc.queue(MoveTo(0, 10));
    let _ = sc.queue(Print(goodbye_msg2));
    let _ = sc.queue(MoveTo(2, world.maxl - 5));
    match world.death_cause {
        DeathCause::Ground => {
            if world.maxc > 91 {
                let _ = sc.queue(Print("\r█▄█ █▀█ █░█   █▀▀ █▀█ ▄▀█ █▀ █░█ █▀▀ █▀▄   █ █▄░█   ▀█▀ █░█ █▀▀   █▀▀ █▀█ █▀█ █░█ █▄░█ █▀▄ ░\n\r░█░ █▄█ █▄█   █▄▄ █▀▄ █▀█ ▄█ █▀█ ██▄ █▄▀   █ █░▀█   ░█░ █▀█ ██▄   █▄█ █▀▄ █▄█ █▄█ █░▀█ █▄▀ ▄\n\r"));
            } else {
                let _ = sc.queue(Print("You crashed in the ground."));
            }
        }
        DeathCause::Enemy => {
            if world.maxc > 72 {
                let _ = sc.queue(Print("\r▄▀█ █▄░█   █▀▀ █▄░█ █▀▀ █▀▄▀█ █▄█   █▄▀ █ █░░ █░░ █▀▀ █▀▄   █▄█ █▀█ █░█ ░\n\r█▀█ █░▀█   ██▄ █░▀█ ██▄ █░▀░█ ░█░   █░█ █ █▄▄ █▄▄ ██▄ █▄▀   ░█░ █▄█ █▄█ ▄\n\r"));
            } else {
                let _ = sc.queue(Print("An enemy killed you."));
            }
        }
        DeathCause::Fuel => {
            if world.maxc > 69 {
                let _ = sc.queue(Print("\r█▄█ █▀█ █░█   █▀█ ▄▀█ █▄░█   █▀█ █░█ ▀█▀   █▀█ █▀▀   █▀▀ █░█ █▀▀ █░░ ░\n\r░█░ █▄█ █▄█   █▀▄ █▀█ █░▀█   █▄█ █▄█ ░█░   █▄█ █▀░   █▀░ █▄█ ██▄ █▄▄ ▄\n\r"));
            } else {
                let _ = sc.queue(Print("You ran out of fuel."));
            }
        }
        _ => {}
    }

    let _ = sc.queue(MoveTo(2, world.maxl -2));
    let _ = sc.queue(Print("Press any key to continue..."));
    let _ = sc.flush();
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
            break;
        }
    }
    let _ = sc.queue(Clear(crossterm::terminal::ClearType::All));
}

/// Game Physic Rules
/// TODO: Move to Physics.rs module later
fn physics(world: &mut World) {
    let mut rng = thread_rng();

    // check if player hit the ground
    check_player_status(world);

    // check enemy hit something
    check_enemy_status(world);
    check_fuel_status(world);

    // move the map Downward
    update_map(&mut rng, world);

    // create new enemy
    create_enemy(&mut rng, world);
    create_fuel(&mut rng, world);

    // Move elements along map movements
    move_enemies(world);
    move_fuel(world);
    move_bullets(world);

    if world.gas >= 1 {
        world.gas -= 1;
    }
}

fn handle_pressed_keys(world: &mut World) {
    if poll(Duration::from_millis(10)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        match key {
            Event::Key(event) => {
                // I'm reading from keyboard into event
                match event.code {
                    KeyCode::Char('q') => world.status = PlayerStatus::Quit,
                    KeyCode::Char('w') => {
                        if world.status == PlayerStatus::Alive && world.player_location.l > 1 {
                            world.player_location.l -= 1
                        }
                    }
                    KeyCode::Char('s') => {
                        if world.status == PlayerStatus::Alive
                            && world.player_location.l < world.maxl - 1
                        {
                            world.player_location.l += 1
                        }
                    }
                    KeyCode::Char('a') => {
                        if world.status == PlayerStatus::Alive && world.player_location.c > 1 {
                            world.player_location.c -= 1
                        }
                    }
                    KeyCode::Char('d') => {
                        if world.status == PlayerStatus::Alive
                            && world.player_location.c < world.maxc - 1
                        {
                            world.player_location.c += 1
                        }
                    }
                    KeyCode::Up => {
                        if world.status == PlayerStatus::Alive && world.player_location.l > 1 {
                            world.player_location.l -= 1
                        }
                    }
                    KeyCode::Down => {
                        if world.status == PlayerStatus::Alive
                            && world.player_location.l < world.maxl - 1
                        {
                            world.player_location.l += 1
                        }
                    }
                    KeyCode::Left => {
                        if world.status == PlayerStatus::Alive && world.player_location.c > 1 {
                            world.player_location.c -= 1
                        }
                    }
                    KeyCode::Right => {
                        if world.status == PlayerStatus::Alive
                            && world.player_location.c < world.maxc - 1
                        {
                            world.player_location.c += 1
                        }
                    }
                    KeyCode::Char('p') => {
                        if world.status == PlayerStatus::Alive {
                            world.status = PlayerStatus::Paused;
                        } else if world.status == PlayerStatus::Paused {
                            world.status = PlayerStatus::Alive;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if world.status == PlayerStatus::Alive && world.bullet.is_empty() {
                            let new_bullet = Bullet::new(
                                world.player_location.c,
                                world.player_location.l - 1,
                                world.maxl / 4,
                            );
                            world.bullet.push(new_bullet);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    let slowness = 100;
    let mut world = World::new(maxc, maxl);

    // show welcoming banner
    welcome_screen(&sc, &world);

    while world.status == PlayerStatus::Alive || world.status == PlayerStatus::Paused {
        handle_pressed_keys(&mut world);
        if world.status != PlayerStatus::Paused {
            physics(&mut world);
            draw(&sc, &mut world)?;
        }
        thread::sleep(time::Duration::from_millis(slowness));
    }

    // game is finished
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    goodbye_screen(&sc, &world);
    sc.queue(Clear(crossterm::terminal::ClearType::All))?
        .execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
