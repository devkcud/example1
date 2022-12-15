# example1

_**example1**_ is a (esolang) parody of _brainfuck_ written in Rust. It will work in any file.

> It was made to learn rust fs.  
> _**MADE ON/FOR LINUX**_

## Table of Contents

- [example1](#example1)
  - [Table of Contents](#table-of-contents)
  - [Build](#build)
    - [Requirements](#requirements)
    - [Using make](#using-make)
    - [Using cargo](#using-cargo)
  - [Documentation](#documentation)
  - [Known issues](#known-issues)

---

## Build

### Requirements

- Rust/Cargo;
- Make;
- /usr/local/bin (required only if you want it to be a part of your system).

### Using make

> It's my first time doing a makefile

Just run:

```bash
$ make compile
$ make try
yCa
```

> You can set it to your local binaries (`/usr/local/bin`) using `make env_add`.

### Using cargo

Build using:

```bash
cargo build --release
# or run an example
cargo run -q -- ./samples/test
```

---

## Documentation

You can view the documentation in [here](DOCS.md)

---

## Known issues

_Empty_.
