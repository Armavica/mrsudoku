# Monadic Rust Sudoku Solver

Inspired by the Haskell implementation described in this [article
series](https://abhinavsarkar.net/posts/fast-sudoku-solver-in-haskell-1/).

The main difference is that I am using `u16` as a bitfield to store digits
whenever the original implementation uses a list.

It is approximately 33 times faster:  7.7s for all the 49151 sudokus,
instead of 257s for the Haskell implementation.

It is somewhat optimized for speed and not so much for code length,
which makes it 410 sloc long, compared to the 150 sloc of Haskell.
