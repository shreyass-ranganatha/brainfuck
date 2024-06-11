# Brainfuck

An experimental `brainfuck` interpreter. 

> Built using these [Language Notes](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)

## Usage
Once the project has been built with Cargo (`cargo build`) > `cargo run` (or the generated executable at `target/debug/brainfuck`) can be used as so–

```zsh
cargo run "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
```

That should print `Hello World!`

## Prospects
I'll go about my experiments by naming them "variants." 

Each subsequent variant will introduce custom updates to the language, in terms of complexity, ease-of-use and grammar parsing methods.
