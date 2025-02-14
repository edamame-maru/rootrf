# rootrf
`rootrf` is an open source utility that finds roots of simple polynomials.

## Installation
Users are recomended to build from source. A small number of binary packages are available for downloade on the [releases](https://github.com/edamame-maru/rootrf/releases) page. 

### Prerequisites
Download `rustup` and install Rust.

On Unix, run
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
and follow the on-screen instructions.

On Windows or macOS, follow the instructions on the main [installation page](https://www.rust-lang.org/tools/install

### Build
Run
```
$ cargo install --git https://github.com/edamame-maru/rootrf
```
to automatically download and compile the sources. A resulting executable is placed in `~/.cargo/bin/`. 

## Usage
Run
```
$ rootrf
```
as a standard user. `cargo` automatically prepends `. "$HOME/.cargo/env"` to your `.bashrc`.
