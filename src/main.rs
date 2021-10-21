use clap::{crate_version, App, Arg};
use std::collections::HashSet;
use std::io::{self, stdout, Write};
use swayipc::{Connection, Fallible};

fn opts() -> (bool, bool) {
    let matches = App::new("i3-open-next-ws")
        .version(crate_version!())
        .about("A companion utility to i3wm for managing workspaces.\nCall without arguments to get first unused workspace number on STDOUT")
        .arg(Arg::new("move").short('m').long("move").about("Move focused window to the first unused workspace"))
        .arg(Arg::new("focus").short('f').long("focus").about("Move focused window to the first unused workspace"))
        .get_matches();
    (matches.is_present("move"), matches.is_present("focus"))
}

fn main() -> Fallible<()> {
    let opts = opts();
    let mut wm = Connection::new()?;
    let mut wss: HashSet<_> = wm.get_workspaces()?.iter().map(|ws| ws.num).collect();
    let next_ws = {
        let mut n = 1;
        loop {
            if wss.insert(n) {
                break;
            }
            n += 1;
        }
        n
    };
    match opts {
        (true, true) => {
            wm.run_command(format!("move to workspace number {}", next_ws))?;
            wm.run_command(format!("workspace number {}", next_ws))?;
        }
        (false, true) => {
            wm.run_command(format!("workspace number {}", next_ws))?;
        }
        (true, false) => {
            wm.run_command(format!("move to workspace number {}", next_ws))?;
        }

        (false, false) => {
            write!(&mut stdout(), "{}", next_ws)?;
        }
    };
    Ok(())
}
