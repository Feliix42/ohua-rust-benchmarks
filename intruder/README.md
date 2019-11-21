# Intruder Benchmark

This benchmark was originally part of the [STAMP benchmark suite](https://doi.org/10.1109/IISWC.2008.4636089) proposed by Minh et. al. and has been ported to Rust.
It's implemented in 3 variants:

- in sequential form without any multithreading
- as Ohua algorithm
- parallelized using STM, ported from the implementation of the paper authors.

## Executing the benchmarks

You can use the command line arguments to manipulate the benchmark parameters. They are all defaulting to the same parameters to make them comparable.
Run them using

```
cargo run --release --bin sequential --features "cli" -- [FLAGS]
cargo run --release --bin ohua --features "cli ohua" -- [FLAGS]
cargo run --release --bin stm --features "cli transactional" -- [FLAGS]
```

respectively.
