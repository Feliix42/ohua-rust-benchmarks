rm Cargo.lock 
cargo clean

cargo build --release --bin sequential --features "cli"
cargo build --release --bin ohua --features "cli ohua"
cargo build --release --bin stm --features "cli transactional"


echo "Starting benchmarks at $(date)"

target/release/sequential --json --outdir first_test_20191122 --runs 30 -n 4096
echo -n "."
target/release/ohua --json --outdir first_test_20191122 --runs 30 -n 4096
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 1
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 2
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 3
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 4
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 6
echo -n "."
target/release/stm --json --outdir first_test_20191122 --runs 30 -n 4096 --threads 8
echo -n "."

echo ""

echo "Finished at $(date)"
