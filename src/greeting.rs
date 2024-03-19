


pub mod greeting {
    use std::{
        io::{Stdout, Write},
        time::Duration,
    };
    use crossterm::{
        cursor::MoveTo,
        event::{poll, read},
        style::Print,
        terminal::Clear,
        QueueableCommand
    };
    use crate::world::world::{World, DeathCause};
    

    pub fn goodbye_screen(mut sc: &Stdout, world: &World) {
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

    pub fn welcome_screen(mut sc: &Stdout, world: &World) {
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

}