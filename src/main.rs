use clap::Parser;
use i3_ipc::{Connect, I3};
use std::{collections::HashSet, process::Command};
use swayipc::{
    reply::{Event, WindowChange},
    Connection, EventType, Fallible,
};

#[derive(Parser)]
#[clap(version = "1.2")]
/// A companion utility to i3wm for managing workspaces.
/// Call without arguments to get first unused workspace number on STDOUT
struct Opts {
    /// Move focused window to the first unused workspace
    #[clap(short, long)]
    _move: bool,
    /// Focus first unused workspace
    #[clap(short, long)]
    focus: bool,
    /// Execute COMMAND on a new workspace
    #[clap(short, long, value_name = "COMMAND")]
    exec: Vec<String>,
    /// overrides workspace selection
    #[clap()]
    workspace: Option<i32>,
}

// fn opts() -> (bool, bool, bool) {
//     let matches = App::new("i3-open-next-ws")
//         .version(crate_version!())
//         .about("A companion utility to i3wm for managing workspaces.\nCall without arguments to get first unused workspace number on STDOUT")
//         .arg(Arg::new("move").short('m').long("move").about("Move focused window to the first unused workspace"))
//         .arg(Arg::new("focus").short('f').long("focus").about("Move focused window to the first unused workspace"))
//         .arg(Arg::new("exec").short('e').long("exec").takes_value(true).value_name("COMMAND").about("Execute COMMAND on a new workspace"))
//         .get_matches();
//     (
//         matches.is_present("move"),
//         matches.is_present("focus"),
//         matches.is_present("exec"),
//     )
// }

fn main() -> Fallible<()> {
    let opts = Opts::parse();
    let mut conn = Connection::new()?;
    let ws = if let Some(ws) = opts.workspace {
        ws
    } else {
        let mut wss: HashSet<_> = match conn.get_workspaces() {
            // This is a dirty workaround the fact that swayipc's get_workspaces fails on i3
            // and i3-ipc's event subscription is broken overall.
            Ok(wss) => wss.iter().map(|ws| ws.num).collect(),
            Err(_) => I3::connect()?
                .get_workspaces()?
                .iter()
                .map(|ws| ws.num as i32)
                .collect(),
        };
        {
            let mut n = 1;
            loop {
                if wss.insert(n) {
                    break;
                }
                n += 1;
            }
            n
        }
    };
    match (opts._move, !opts.exec.is_empty(), opts.focus) {
        (_, true, true) => {
            exec_on(&mut conn, opts.exec, ws)?;
            conn.run_command(format!("workspace number {}", ws))?;
        }
        (_, true, false) => exec_on(&mut conn, opts.exec, ws)?,
        (true, _, false) => {
            conn.run_command(format!("move to workspace number {}", ws))?;
        }
        (true, _, true) => {
            conn.run_command(format!("move to workspace number {}", ws))?;
            conn.run_command(format!("workspace number {}", ws))?;
        }
        (false, false, true) => {
            conn.run_command(format!("workspace number {}", ws))?;
        }
        (false, false, false) => println!("{}", ws),
    };
    Ok(())
}

fn exec_on(conn: &mut Connection, cmd: Vec<String>, ws: i32) -> Fallible<()> {
    let mut cmd = match cmd.len() {
        1 => Command::new(cmd.first().unwrap()),
        2.. => {
            let mut command = Command::new(cmd.first().unwrap());
            command.args(&cmd[1..]);
            command
        }
        _ => return Ok(()),
    };
    let mut ev = Connection::new()?.subscribe(&[EventType::Window])?;
    cmd.spawn()?;
    while let Some(Ok(ev)) = ev.next() {
        match ev {
            Event::Window(w) if w.change == WindowChange::New => {
                if let Some(w) = w.container.window {
                    conn.run_command(format!("[id={}] move to workspace number {}", w, ws))?;
                    return Ok(());
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
