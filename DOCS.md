# Docs

## Table of Contents

- [Docs](#docs)
  - [Table of Contents](#table-of-contents)
  - [Commands](#commands)
    - [Word Commands](#word-commands)
    - [Example](#example)
  - [Limitations](#limitations)

## Commands

| Index | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | ..31
|   -:   | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :--: | :--: | :--: | :--: | :--: | :--: | :--:
| Value | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0  | 0  | 0  | 0  | 0  | 0  | ..
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
| `       | Start a word command (see more: [Word Commands](#word-commands))
| `0-9`   | Set the index to the specific number (Only support 0-9)

> Everything else than those commands is considered a comment.

---

### Word Commands

`Word Commands` are even simpler than normal commands.

| Command | Description              | Usage                            | Example
| -:      | :-                       | :-                               | :-
| swap    | Will swap two values     | \`swap <From: Index> <To: Index> | \`swap 3 1
| rem     | Set the given index to 0 | \`rem <From: Index>              | \`rem 1

### Example

The following example will give you a better understanding:

**Objective:** Form the word `yCa`.

```txt
1   ]]]]]]] (((
2 } ]]      )
3 } ]       (((
4 }         (((

`swap 3 1
`rem 1

=
```

This code will swap the index 3 with the index 1, then remove the index 1. So it will return `yCa`. (Original: `Cyka`)  
For the sake of explanation here is an unformatted version of the previous code:

```txt
1]]]]]]](((2}]])3}](((4}(((
`swap 3 1
`rem 1
=
```

**RULES OF EXISTENCE:**

- The `Word Command` needs to be in a new line;
- The `Word Command` needs to be an actual command.

---

## Limitations

- No spaces;
  - Use "$=" to create spaces.
- Limit of 32 chars;
- No support for special characters;
- Max 255 bytes for each index.
