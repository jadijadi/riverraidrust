use std::{cmp::Ordering::*, io::{stdout, Stdout, Write}, time::Duration};
use std::{thread, time};
use rand::{thread_rng, Rng};

use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode}, style::Print, terminal::{disable_raw_mode, enable_raw_mode, size, Clear}, ExecutableCommand, QueueableCommand
};

#[derive(PartialEq, Eq)]
enum PlayerStatus {
    Dead,
    Alive,
    Animation,
    Paused
}


struct Location {
    c: u16,
    l: u16
}

struct Enemy {
    location: Location
}

struct Bullet {
    location: Location,
    energy: u16,
}

struct World {
    player_location: Location,
    map: Vec<(u16, u16)>,
    maxc: u16,
    maxl: u16,
    status: PlayerStatus,
    next_right: u16,
    next_left: u16,
    ship: String,
    enemy: Vec<Enemy>,
    bullet: Vec<Bullet>,
}

impl World {

    fn new (maxc: u16, maxl: u16) -> World {
        World {
            player_location: Location{
                c: maxc / 2,
                l: maxl - 1
            },
            map: vec![(maxc/2-5, maxc/2+5); maxl as usize],
            maxc,
            maxl,
            status: PlayerStatus::Alive,
            next_left: maxc / 2 - 7,
            next_right: maxc / 2 + 7,
            ship: 'P'.to_string(),
            enemy: vec![],
            bullet: vec![],
        }
    }

}

fn hit_margin(loc1: &Location,loc2: &Location, l_margin:u16, c_margin:u16) -> bool{
    return 
    (loc1.c == loc2.c && loc1.l == loc2.l) || 
    (loc1.c + c_margin == loc2.c && loc1.l == loc2.l) || 
    (loc1.c == loc2.c && loc1.l + l_margin == loc2.l) ||
    (loc1.c - c_margin == loc2.c && loc1.l == loc2.l) || 
    (loc1.c == loc2.c && loc1.l - l_margin == loc2.l) 
    ;
}
fn hit(loc1: &Location,loc2: &Location) -> bool{
    return loc1.c == loc2.c && loc1.l == loc2.l;
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;

    // draw the map
    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?
            .queue(Print("+".repeat(world.map[l].0 as usize)))?
            .queue(MoveTo(world.map[l].1, l as u16))?
            .queue(Print("+".repeat((world.maxc - world.map[l].1) as usize)))?;
    }

    // draw enemies
    for e in &world.enemy {
        sc.queue(MoveTo(e.location.c, e.location.l))?
        .queue(Print("E"))?;       
    }

    // draw bullet
    for b in &world.bullet {
        sc.queue(MoveTo(b.location.c, b.location.l))?
            .queue(Print("|"))?
            .queue(MoveTo(b.location.c, b.location.l-1))?
            .queue(Print("^"))?;
    }

    // draw the player
    sc.queue(MoveTo(world.player_location.c, world.player_location.l))?
        .queue(Print(world.ship.as_str()))?
        .flush()?;

    Ok(())
}


fn physics(world: &mut World) {
    let mut rng = thread_rng();

    // check if player hit the ground
    if world.player_location.c < world.map[world.player_location.l as usize].0 ||
        world.player_location.c >= world.map[world.player_location.l as usize].1 {
        world.status = PlayerStatus::Dead;
        return;
    }

    // check enemy hit something
    for i in (0..world.enemy.len()).rev() {
        if hit(&world.enemy[i].location,&world.player_location) {
            world.status = PlayerStatus::Dead;
            return;
        };
        for j in (0..world.bullet.len()).rev() {
            
            if hit_margin(&world.enemy[i].location,&world.bullet[j].location,1,0) {
                world.enemy.remove(i);
            }
        }
    }

    // move the map downward
    for l in (1..world.map.len()).rev() {
        world.map[l] = world.map[l - 1];
    }

    let (left, right) = &mut world.map[0];
    match world.next_left.cmp(left) {
        Greater => *left += 1,
        Less => *left -= 1,
        Equal => {},
    };
    match world.next_right.cmp(right) {
        Greater => *right += 1,
        Less => *right -= 1,
        Equal => {},
    };

    // TODO: below randoms may 1) go outside of range
    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7 {
        world.next_left = rng.gen_range(world.next_left-5..world.next_left+5)
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7  {
        world.next_right = rng.gen_range(world.next_right-5..world.next_right+5)
    }

    if world.next_right.abs_diff(world.next_left) < 3 {
        world.next_right += 3;
    }

    // create a new enemy; maybe
    if rng.gen_range(0..10) >= 9 {
        let new_enemy = Enemy {
            location: Location{
                l: 0,
                c: rng.gen_range(world.map[0].0..world.map[0].1)
            }
        };
        world.enemy.push(new_enemy);
    }

    // move enemies on the river
    for i in (0..world.enemy.len()).rev() {
        world.enemy[i].location.l += 1;
        if world.enemy[i].location.l >= world.maxl {
            world.enemy.remove(i);
        }
    }

    // move the bullets
    for i in (0..world.bullet.len()).rev() {
        if world.bullet[i].energy == 0 || world.bullet[i].location.l <= 2{
            world.bullet.remove(i);
        } else {
            world.bullet[i].location.l -= 2;
            world.bullet[i].energy -= 1;
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

    while world.status == PlayerStatus::Alive || world.status == PlayerStatus::Paused {

        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();

            while poll(Duration::from_millis(0)).unwrap() {
                let _ = read();
            }

            match key {
                Event::Key(event) => {
                    // I'm reading from keyboard into event
                    match event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('w') => if world.status != PlayerStatus::Paused && world.player_location.l > 1 { world.player_location.l -= 1 },
                        KeyCode::Char('s') => if world.status != PlayerStatus::Paused && world.player_location.l < maxl - 1 { world.player_location.l += 1 },
                        KeyCode::Char('a') => if world.status != PlayerStatus::Paused && world.player_location.c > 1 { world.player_location.c -= 1 },
                        KeyCode::Char('d') => if world.status != PlayerStatus::Paused && world.player_location.c < maxc - 1 { world.player_location.c += 1},
                        KeyCode::Up => if world.status != PlayerStatus::Paused && world.player_location.l > 1 { world.player_location.l -= 1 },
                        KeyCode::Down => if world.status != PlayerStatus::Paused && world.player_location.l < maxl - 1 { world.player_location.l += 1 },
                        KeyCode::Left => if world.status != PlayerStatus::Paused && world.player_location.c > 1 { world.player_location.c -= 1 },
                        KeyCode::Right => if world.status != PlayerStatus::Paused && world.player_location.c < maxc - 1 { world.player_location.c += 1},
                        KeyCode::Char('p') => {
                            if world.status == PlayerStatus::Alive { world.status = PlayerStatus::Paused; }
                            else if world.status == PlayerStatus::Paused { world.status = PlayerStatus::Alive; }
                        },
                        KeyCode::Char(' ') => if world.status != PlayerStatus::Paused && world.bullet.len() == 0 {
                            let bullet = Bullet {
                                location: Location{
                                    c: world.player_location.c,
                                    l: world.player_location.l - 1,
                                },
                                energy: world.maxl / 4,
                            };
                            world.bullet.push(bullet);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if world.status != PlayerStatus::Paused
        {
            physics(&mut world);
            draw(&sc, &world)?;
        }
        thread::sleep(time::Duration::from_millis(slowness));
    }

    // game is finished

    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.queue(MoveTo(maxc / 2, maxl / 2))?;
    sc.queue(Print("Good game! Thanks.\n"))?;
    thread::sleep(time::Duration::from_millis(3000));
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.execute(Show)?;
    Ok(())
}
