Chip-8 emulator
===

This is a Chip-8 emulator written in Rust. The library `ggez` is used to draw "pixels" onto the screen and to play sounds. Currently playing sounds is not supported.


Resources
===

These are references for Chip-8 specifications and libraries used for this emulator.

+ [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
+ [Cowgod's Chip-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
+ [ggez](https://ggez.rs/)
+ [The Rust Rand Book](https://rust-random.github.io/book)

The public domain games are taken from [Zophar.net](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)


Requirements
===

To build and run the emulator on your system, please install [Rust](https://www.rust-lang.org).


Usage
===

Clone this repository and change directory into the cloned folder, then run:

```
cargo run --release /path/to/rom
```
