use std::{cmp::Ordering::*, io::{stdout, Stdout, Write}, time::Duration};
use std::{thread, time};
use std::num::Wrapping;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode}, style::Print, terminal::{enable_raw_mode, size, Clear}, ExecutableCommand, QueueableCommand
};

#[derive(PartialEq, Eq)]
enum PlayerStatus {
    Dead,
    Alive,
    Animation,
    Paused,
    Quit
}

struct Location{
    c: u16,
    l: u16
}

struct Enemy {
    location: Location
}

impl Enemy {

    fn new(loc: Location) -> Enemy {
        Enemy {
            location: loc
        }
    }
    
}

struct Bullet {
    location: Location,
    energy: u16,
}

impl Bullet {
    
    fn new(loc: Location, energy: u16) -> Bullet {
        Bullet {
            location: loc,
            energy
        }
    }

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
            enemy: Vec::new(),
            bullet: Vec::new(),
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

/// check if player hit the ground
fn check_player_status(world: &mut World) {

    if world.player_location.c < world.map[world.player_location.l as usize].0 ||
        world.player_location.c >= world.map[world.player_location.l as usize].1 {
        world.status = PlayerStatus::Dead;
    }

}

/// check enemy hit something
fn check_enemy_status(world: &mut World) {

    for index in (0..world.enemy.len()).rev() {

        if hit(&world.enemy[index].location,&world.player_location) {
            world.status = PlayerStatus::Dead;
            return;
        };

        // 
        for j in (0..world.bullet.len()).rev() {
            if hit_margin(&world.enemy[index].location,&world.bullet[j].location,1,0) {
                world.enemy.remove(index);
            }
        }
    }

}

/// Update the map
fn update_map(rng: &mut ThreadRng, world: &mut World) {
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

    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7  {
        world.next_left = rng.gen_range(world.next_left.saturating_sub(5)..world.next_left+5);
        if world.next_left == 0 {
            world.next_left = 1;
        }
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7  {
        world.next_right = rng.gen_range(world.next_right-5..world.next_right+5);
        if world.next_right > world.maxc {
            world.next_right = Wrapping(world.maxc).0 - 1;
        }
    }

    if world.next_right.abs_diff(world.next_left) < 3 {
        world.next_right += 3;
    }
}

/// Create a new enemy
fn create_enemy(rng: &mut ThreadRng, world: &mut World) {

    // Possibility
    if rng.gen_range(0..10) >= 9 {
        world.enemy.push(
            Enemy::new(
                Location{
                    c:rng.gen_range(world.map[0].0..world.map[0].1),
                    l:0,
                }
            )
        );
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
        if world.bullet[index].energy == 0 || world.bullet[index].location.l <= 2{
            world.bullet.remove(index);
        } else {
            world.bullet[index].location.l -= 2;
            world.bullet[index].energy -= 1;
        }
    }   

}

fn welcome_screen(mut sc: &Stdout, world: &World) {
    let welcome_msg: &str = "██████╗ ██╗██╗   ██╗███████╗██████╗ ██████╗  █████╗ ██╗██████╗     ██████╗ ██╗   ██╗███████╗████████╗\n\r██╔══██╗██║██║   ██║██╔════╝██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝\n\r██████╔╝██║██║   ██║█████╗  ██████╔╝██████╔╝███████║██║██║  ██║    ██████╔╝██║   ██║███████╗   ██║   \n\r██╔══██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║  ██║    ██╔══██╗██║   ██║╚════██║   ██║   \n\r██║  ██║██║ ╚████╔╝ ███████╗██║  ██║██║  ██║██║  ██║██║██████╔╝    ██║  ██║╚██████╔╝███████║   ██║   \n\r╚═╝  ╚═╝╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═════╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   \n";
    let _ = sc.queue(Clear(crossterm::terminal::ClearType::All));
    let _ = sc.queue(MoveTo(0, 2));
    let _ = sc.queue(Print(welcome_msg));
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

    // move the map Downward
    update_map(&mut rng, world);

    // create new enemy
    create_enemy(&mut rng, world);
    
    // Move elements along map movements
    move_enemies(world);
    move_bullets(world);
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
                    KeyCode::Char('w') => if world.status == PlayerStatus::Alive && world.player_location.l > 1 { world.player_location.l -= 1 },
                    KeyCode::Char('s') => if world.status == PlayerStatus::Alive && world.player_location.l < world.maxl - 1 { world.player_location.l += 1 },
                    KeyCode::Char('a') => if world.status == PlayerStatus::Alive && world.player_location.c > 1 { world.player_location.c -= 1 },
                    KeyCode::Char('d') => if world.status == PlayerStatus::Alive && world.player_location.c < world.maxc - 1 { world.player_location.c += 1},
                    KeyCode::Up => if world.status == PlayerStatus::Alive && world.player_location.l > 1 { world.player_location.l -= 1 },
                    KeyCode::Down => if world.status == PlayerStatus::Alive && world.player_location.l < world.maxl - 1 { world.player_location.l += 1 },
                    KeyCode::Left => if world.status == PlayerStatus::Alive && world.player_location.c > 1 { world.player_location.c -= 1 },
                    KeyCode::Right => if world.status == PlayerStatus::Alive && world.player_location.c < world.maxc - 1 { world.player_location.c += 1},
                    KeyCode::Char('p') => {
                        if world.status == PlayerStatus::Alive { world.status = PlayerStatus::Paused; }
                        else if world.status == PlayerStatus::Paused { world.status = PlayerStatus::Alive; }
                    },
                    KeyCode::Char(' ') => if world.status == PlayerStatus::Alive && world.bullet.is_empty() {
                        let new_bullet = Bullet::new(Location{
                            c: world.player_location.c,
                            l: world.player_location.l - 1
                        }, world.maxl / 4);
                        world.bullet.push(new_bullet);
                    },
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
            draw(&sc, &world)?;
        }
        thread::sleep(time::Duration::from_millis(slowness));
    }

    // game is finished

    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    goodbye_screen(&sc, &world);
    
    sc.queue(Clear(crossterm::terminal::ClearType::All))?
        .execute(Show)?;    
    Ok(())
}
