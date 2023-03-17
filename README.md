```
 _ __ ___  _ __ ___  ___
| '_ ` _ \| '_ ` _ \/ __|
| | | | | | | | | | \__ \
|_| |_| |_|_| |_| |_|___/

```

## Mms
Minimal Mpd/Music Selector

This is a minimal selector for mpd, it just selects music, and plays it,
with no tag manager, no image rendering.

If you want something more Featureful, check out
[ncmpcpp](https://github.com/ncmpcpp/ncmpcpp) or [gompd](https://github.com/fhs/gompd)

### Installation

For now, the unique way to install is by cloning the project and building it by
yourself, if for some reason the mms receive some attention, I will launch it in
repositories like the arch AUR, and others

#### Guide

1. Clone the project `git clone https://github.com/joao-vitor-sr/mms`
2. Enters the dir `cd mms`
3. Build `cargo build --release`
4. Move/copy the bin at your `$PATH` by `cp target/release/mms <your path here>`

## Usage

Open your favorite terminal emulator and type mms, for the help menu type `?`
on the TUI

## TODO

1. Today the mpd address is hard coded on the codebase, it's useful to be able
to set it up
