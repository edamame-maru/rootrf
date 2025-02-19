# rootrf
`rootrf` is an open source utility that finds roots of simple polynomials.

## Installation
Users are recomended to build from source. A small number of binary packages for `x86-64` Linux are available for download on the [releases](https://github.com/edamame-maru/rootrf/releases) page. 

### Prerequisites
Download `rustup` and install Rust.

On Unix, run:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
and follow the on-screen instructions.

On Windows or macOS, follow the instructions on the main [installation page.](https://www.rust-lang.org/tools/install)

When using binaries, make sure to change permissions with:
```
$ chmod +x /path/to/rootrf
```
before execution.

### Build
```
$ cargo install --git https://github.com/edamame-maru/rootrf
```
will automatically download and compile the sources. The resulting executable is placed in `~/.cargo/bin/`. 

## Usage
`cargo` automatically prepends `. "$HOME/.cargo/env"` to your `.bashrc` post-build:
```
$ rootrf <arg1> <arg2>
```

### Syntax
`rootrf` finds roots for any polynomial. Follow `rootrf` with the coefficients of the polynomial by descending order. The last argument is the constant term. For example, $$2x^2 + 5x + 3$$ would be:

```
$ rootrf 2 5 3
f(x) = 2x^2 + 5x + 3
=> -1.0000001
```
By default, `rootrf` will output the root closest to zero.

## Uninstall
`cargo` cleans the Rust binary from `~/.cargo/bin/` with the following command:  
```
$ cargo uninstall rootrf
```
