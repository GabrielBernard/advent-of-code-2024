# Advent of Code (AoC) 2024

This is my attempt at solving the [AoC 2024](https://adventofcode.com/2024) in [Rust](https://www.rust-lang.org/).

The repository contains a dev container built for the challenge with Rust version 1.82.

The project is structured as [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
which means each `day-xx` folder (workspace) represents the challenge of that day and each of these folders
contain a `Cargo.toml` file listing the dependencies used (if any) to solve that challenge.

Each folder contains the `STATEMENT` of that day (followed by `_PARTX.txt` if the challenge had multiple parts),
any `INPUT.txt` file of the challenge, and the code for the solution in the `src` sub-folder.

As an example, to run the solution of the `day-01` challenge, use:

```bash
cargo run -p day-01
```
