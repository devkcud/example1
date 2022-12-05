# example1

_example1_ is a (esolang) parody of _brainfuck_ written in Rust. It will work in any file.

> It was made to learn rust fs.  
> _**MADE ON/FOR LINUX**_

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
hey.
```

> btw you can set it to your `/usr/local/bin` using `make env_add`

### Using cargo

Build using:

```bash
cargo build --release
# or run an example
cargo run -q -- ./examples/test
```

## Limitations

- No spaces;
  - Use "$@" to create spaces.
- Limit of 32 chars;
- Max 255 bytes for each index.

## Commands

| | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|   -:   | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :--: | :--: | :--: | :--: | :--: | :--: |
| Value | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0  | 0  | 0  | 0  | 0  | 0  |
| Cursor | ^ |   |   |   |   |   |   |   |   |   |    |    |    |    |    |    |

| Command | Action
| -:      | :-
| `>`     | Move the cursor to the next index
| `<`     | Move the cursor to the previous index
| `)`     | Add +1
| `]`     | Add +10
| `}`     | Add +100
| `(`     | Remove -1
| `[`     | Remove -10
| `{`     | Remove -100
| `!`     | Set value of the current index to 0
| `$`     | Reset all values
| `&`     | Get index
| `.`     | Set index to 0
| `@`     | Get value
| `#`     | Get list
| `=`     | Get output
| `'`     | Copy the current list
| `"`     | Use the copied list
| `~`     | Reverse the list (need to be set again with `"`)

> Everything else than those commands is considered a comment.
