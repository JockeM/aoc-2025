# 🎄 Advent of Code 2025

## Usage

```sh
cargo today
cargo scaffold <day>
cargo download <day>
cargo solve <day> [--submit}
cargo time <day> [--all]
cargo test --bin <day>
cargo read <day>
```

## Profile

Alloc profile

```sh
cargo solve 1 --dhat
```

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.
