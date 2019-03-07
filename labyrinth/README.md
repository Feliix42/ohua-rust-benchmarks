# Labyrinth Benchmark

This benchmark was originally part of the [STAMP benchmark suite]() proposed by Minh et. al. and has been ported to Rust.
It's implemented in 3 variants:

- in sequential form without any multithreading
- as Ohua algorithm
- parallelized using STM

## Executing the benchmarks

You can use the individual input files provided in the [original](https://github.com/kozyraki/stamp/tree/21986e2c05eb42afc3242473cd73baf1b73c78a7/labyrinth/inputs) version or produce your own.
To run the benchmarks, run

```
cargo run --release --bin simple_sequential --features "cli bench" -- [INPUT]
cargo run --release --bin ohua --features "cli bench ohua" -- [INPUT]
cargo run --release --bin stm --features "cli bench transactional" -- [INPUT]
```

respectively.
