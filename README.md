# i3-open-next-ws
A companion utility to i3wm for managing workspaces.
I found myself wanting to move windows to their own workspaces, without caring which number that workspace has, so I wrote this simple tool to pick first unused workspace automagically.

## Installation:
```sh
cargo install --git https://github.com/JohnDowson/i3-open-next-ws.git
# add ~/.cargo/bin/ to path
```

## Usage:
```sh
i3-open-next-ws [OPTIONS] [WORKSPACE]
OPTIONS:
    -e, --exec <COMMAND>       Execute COMMAND on a new workspace
    -f, --focus                Focus first unused workspace
    -m, --move                 Move focused window to the first unused workspace
```
This will move currently focused window to first unused workspace:
```sh
$ i3-open-next-ws --move
```
This will focus workspace 5
```sh
$ i3-open-next-ws --focus 5
```
This will open chromium on workspace 3
```sh
$ i3-open-next-ws --exec 3
```
Commands can be combined: this will move currently focused window to the first unused workspace AND focus that workspace
```sh
$ i3-open-next-ws --focus --move
```