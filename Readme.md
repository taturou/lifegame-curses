# LifeGmae

This is yet another 'Conway's Game of Life'.

![random](doc/images/lifegame-anime-random.gif)
![manual](doc/images/lifegame-anime-manual.gif)

## Feature

* Made by Rust langurage
* TUI (Text User Interface) by curses library
* Usable not only keyboard but also mouse

## Require

* Rust
    * A system programming language.
* Cursive
    * A TUI (Text User Interface) library for Rust.
* A curses library
    * A terminal control library that is supported by Cursive.

## Build

How to build on Ubuntu linux.

1. Clone from GitHub.

```bash
$ cd /your/favorite/path/
$ git clone https://github.com/taturou/lifegame-rust-cli.git
```

2. Build by cargo

```bash
$ cd lifegame-rust-cli
$ cargo build
```

3. Run

```bash
$ cargo run
```
