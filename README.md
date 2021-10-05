# i3-open-next-ws
A companion utility to i3wm for managing workspaces.
## Installation:
```sh
cargo install --git https://github.com/JohnDowson/i3-open-next-ws.git
# add ~/.cargo/bin/ to path
```

## Usage:
```sh
# Those two flags can be combined
i3-open-next-ws --move # Moves focused window to first unused workspace
i3-open-next-ws --focus # Focuses first unused workspace
i3-open-next-ws # Writes the number of the first unused workspace to stdout
```
