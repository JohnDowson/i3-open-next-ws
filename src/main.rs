use clap::{clap_app, crate_authors, crate_version};
use i3_ipc::{Connect, I3};
use std::io::{self, stdout, Write};

fn opts() -> (bool, bool) {
    let matches = clap_app!(i3_open_next_ws =>
        (version: crate_version!())
        (author: crate_authors!())
        (@arg move: -m --move)
        (@arg focus: -f --focus)
    )
    .get_matches();
    (matches.is_present("move"), matches.is_present("focus"))
}

fn main() -> io::Result<()> {
    let opts = opts();
    let mut i3 = I3::connect()?;
    let wss: Vec<_> = i3.get_workspaces()?.iter().map(|ws| ws.num).collect();
    let nr = wss.windows(2).find(|ab| match ab {
        [a, b] if a + 1 != *b => true,
        [_, _] => false,
        [_] => true,
        _ => unreachable!(),
    });
    let next_ws = match nr {
        Some([n]) => n,
        Some([n, _]) => n,
        None => wss.last().unwrap_or(&0),
        Some(_) => unreachable!(),
    } + 1;
    match opts {
        (true, true) => {
            i3.run_command(format!("move to workspace {}", next_ws))?;
            i3.run_command(format!("workspace {}", next_ws))?;
        }
        (false, true) => {
            i3.run_command(format!("workspace {}", next_ws))?;
        }
        (true, false) => {
            i3.run_command(format!("move to workspace {}", next_ws))?;
        }

        (false, false) => {write!(&mut stdout(), "{}", next_ws)?;},
    };
    Ok(())
}
