# Benchmarks for Ohua and Rust-STM

This repository contains a number of benchmarks that are used to compare the performance of Ohua (in Rust) against other parallelism approaches such as [rust-stm](https://github.com/Marthog/rust-stm).


## Running the benchmarks

To run an ohua-based benchmark (and thus, for the sake of comparability all other benchmarks), you have to use `nightly` Rust until the `proc_macro_hygiene` feature has been stabilized.
Just run

```
rustup override set nightly
```

in the root directory of the repository to set this for all submodules.

For information on how to run the inidividual benchmarks, consult the individual `README`s in the subfolders.
