# Labyrinth Benchmark

This benchmark was originally part of the [STAMP benchmark suite](https://doi.org/10.1109/IISWC.2008.4636089) proposed by Minh et. al. and has been ported to Rust.
It's implemented in 3 variants:

- in sequential form without any multithreading
- as Ohua algorithm
- parallelized using STM

## Executing the benchmarks

You can use the individual input files from the [original](https://github.com/kozyraki/stamp/tree/21986e2c05eb42afc3242473cd73baf1b73c78a7/labyrinth/inputs) benchmark in the `inputs/` directory or produce your own.
To run the benchmarks, run

```
cargo run --release --bin simple_sequential --features "cli bench" -- [INPUT]
cargo run --release --bin ohua --features "cli bench ohua" -- [INPUT]
cargo run --release --bin stm --features "cli bench transactional" -- [INPUT]
[...]
```

respectively.

## Running split benchmarks

The partial loop unroll benchmarks can be generated and executed by following these steps:
```
cd src/modified_algos/generator
# the range below is just an example!
python3 generate.py 2 3 4 5 6 7 8 9 10
cd ../../../
chmod u+x bench-split-versions.sh
./bench-split-versions.sh
```

