# Luxtorpeda

[![luxtorpeda discord](https://img.shields.io/discord/514567252864008206.svg?label=discord)](https://discord.gg/8mFhUPX)

Compatibility tool to run games on Steam using native Linux engines. [Read more.](https://github.com/dreamer/steam-dos/wiki/Luxtorpeda)

This is a sister project of [steam-dos](https://github.com/dreamer/steam-dos/).

Official mirrors:
[GitHub](https://github.com/dreamer/luxtorpeda),
[GitLab](https://gitlab.com/luxtorpeda/luxtorpeda).

## Installation (using tarball)

TBD

## Installation (from source)

0. Download the latest version of Rust: https://www.rust-lang.org/
1. Close Steam.
2. Clone the repository and use makefile to trigger `cargo build` and install resulting binary:

       $ git clone https://github.com/dreamer/luxtorpeda.git
       $ cd luxtorpeda
       $ make user-install

3. Start Steam.
4. In game properties window select "Force the use of a specific Steam Play
   compatibility tool" and select "Luxtorpeda&nbsp;(dev)".

## Supported titles

TODO: list games

## Development

You can use `cargo` as with any Rust project; `make` serves only as a convenient
frontend for packaging and triggering longer `cargo` commands.

TODO: Add documentation about packaging games for Luxtorpeda.
