use core::time;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read},
    style::Print,
    QueueableCommand,
};
use std::{
    io::{Stdout, Write},
    thread,
    time::Duration,
};

use crate::{
    stout_ext::StdoutExt,
    world::{DeathCause, World},
};

pub fn goodbye_screen(sc: &mut Stdout, world: &World) -> Result<(), std::io::Error> {
    let goodbye_msg1: &str = " ██████╗  ██████╗  ██████╗ ██████╗      ██████╗  █████╗ ███╗   ███╗███████╗██╗\n\r██╔════╝ ██╔═══██╗██╔═══██╗██╔══██╗    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██║\n\r██║  ███╗██║   ██║██║   ██║██║  ██║    ██║  ███╗███████║██╔████╔██║█████╗  ██║\n\r██║   ██║██║   ██║██║   ██║██║  ██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ╚═╝\n\r╚██████╔╝╚██████╔╝╚██████╔╝██████╔╝    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗██╗\n\r ╚═════╝  ╚═════╝  ╚═════╝ ╚═════╝      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝\n";
    let goodbye_msg2: &str = "████████╗██╗  ██╗ █████╗ ███╗   ██╗██╗  ██╗███████╗\n\r╚══██╔══╝██║  ██║██╔══██╗████╗  ██║██║ ██╔╝██╔════╝\n\r   ██║   ███████║███████║██╔██╗ ██║█████╔╝ ███████╗\n\r   ██║   ██╔══██║██╔══██║██║╚██╗██║██╔═██╗ ╚════██║\n\r   ██║   ██║  ██║██║  ██║██║ ╚████║██║  ██╗███████║██╗\n\r   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝\n";

    sc.clear_all()?
        .draw((0, 2), goodbye_msg1)?
        .draw((0, 10), goodbye_msg2)?;

    sc.move_cursor((2, world.maxl - 5))?;
    match world.player.death_cause {
        DeathCause::Ground => {
            if world.maxc > 91 {
                sc.print("\r█▄█ █▀█ █░█   █▀▀ █▀█ ▄▀█ █▀ █░█ █▀▀ █▀▄   █ █▄░█   ▀█▀ █░█ █▀▀   █▀▀ █▀█ █▀█ █░█ █▄░█ █▀▄ ░\n\r░█░ █▄█ █▄█   █▄▄ █▀▄ █▀█ ▄█ █▀█ ██▄ █▄▀   █ █░▀█   ░█░ █▀█ ██▄   █▄█ █▀▄ █▄█ █▄█ █░▀█ █▄▀ ▄\n\r")?;
            } else {
                sc.print("You crashed in the ground.")?;
            }
        }
        DeathCause::Enemy => {
            if world.maxc > 72 {
                sc.print("\r▄▀█ █▄░█   █▀▀ █▄░█ █▀▀ █▀▄▀█ █▄█   █▄▀ █ █░░ █░░ █▀▀ █▀▄   █▄█ █▀█ █░█ ░\n\r█▀█ █░▀█   ██▄ █░▀█ ██▄ █░▀░█ ░█░   █░█ █ █▄▄ █▄▄ ██▄ █▄▀   ░█░ █▄█ █▄█ ▄\n\r")?;
            } else {
                sc.print("An enemy killed you.")?;
            }
        }
        DeathCause::Fuel => {
            if world.maxc > 69 {
                sc.print("\r█▄█ █▀█ █░█   █▀█ ▄▀█ █▄░█   █▀█ █░█ ▀█▀   █▀█ █▀▀   █▀▀ █░█ █▀▀ █░░ ░\n\r░█░ █▄█ █▄█   █▀▄ █▀█ █░▀█   █▄█ █▄█ ░█░   █▄█ █▀░   █▀░ █▄█ ██▄ █▄▄ ▄\n\r")?;
            } else {
                sc.print("You ran out of fuel.")?;
            }
        }
        _ => {}
    }

    sc.queue(MoveTo(2, world.maxl - 2))?;
    thread::sleep(time::Duration::from_millis(2000));
    sc.queue(Print("Press any key to continue..."))?;
    sc.flush()?;
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            read()?;
            break;
        }
    }

    sc.clear_all()?;
    Ok(())
}

pub fn welcome_screen(sc: &mut Stdout, world: &World) -> Result<(), std::io::Error> {
    let welcome_msg: &str = "██████╗ ██╗██╗   ██╗███████╗██████╗ ██████╗  █████╗ ██╗██████╗     ██████╗ ██╗   ██╗███████╗████████╗\n\r██╔══██╗██║██║   ██║██╔════╝██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝\n\r██████╔╝██║██║   ██║█████╗  ██████╔╝██████╔╝███████║██║██║  ██║    ██████╔╝██║   ██║███████╗   ██║   \n\r██╔══██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║  ██║    ██╔══██╗██║   ██║╚════██║   ██║   \n\r██║  ██║██║ ╚████╔╝ ███████╗██║  ██║██║  ██║██║  ██║██║██████╔╝    ██║  ██║╚██████╔╝███████║   ██║   \n\r╚═╝  ╚═╝╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═════╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   \n";
    sc.clear_all()?;

    if world.maxc > 100 {
        sc.draw((0, 2), welcome_msg)?;
    } else {
        sc.draw((0, 2), "RiverRaid Rust")?;
    }

    sc.draw((2, world.maxl - 2), "Press any key to continue...")?;
    sc.flush()?;

    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            read()?;
            break;
        }
    }
    sc.clear_all()?;

    Ok(())
}
