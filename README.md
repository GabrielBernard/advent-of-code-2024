# Advent of Code (AoC) 2024

This is my attempt at solving the [AoC 2024](https://adventofcode.com/2024) in [Rust](https://www.rust-lang.org/).

The repository contains a dev container built for the challenge with Rust version 1.82.

The project is structured as [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
which means each `day-xx` folder (workspace) represents the challenge of that day and each of these folders
contain a `Cargo.toml` file listing the dependencies used (if any) to solve that challenge.

Each folder is missing an `INPUT.txt` file for the challenge of that day.
The code for the solution is in the `src` sub-folder of each day.

As an example, to run the solution of the `day-01` challenge, use:

```bash
cargo run -p day-01
```

## Why aren't the statement and INPUT.txt files in this repository?

The author of the advent of code event has a section about this in the [about page](https://adventofcode.com/2024/about)
of the event where it mentions:

> Can I copy/redistribute part of Advent of Code? **Please don't.**
>
> Advent of Code is free to use, not free to copy. If you're posting a code repository somewhere,
> please don't include parts of Advent of Code like the puzzle text or your inputs.
> If you're making a website, please don't make it look like Advent of Code or name it something similar.
