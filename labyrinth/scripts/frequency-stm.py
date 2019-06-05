import sys
import math

def main(args):
    if len(args) < 2:
        print("[Error] Too few arguments!\nUsage: ./frequency-stm.py [sizes]")
        return

    # test parameters
    inputs = [
        "random-x32-y32-z3-n96.txt", "random-x48-y48-z3-n64.txt",
        "random-x128-y128-z5-n128.txt", "random-x256-y256-z5-n256.txt"
    ]
    runs = "30"

    bench_script = """#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin simple_sequential --features "cli bench"
# cargo build --release --bin stm --features "cli bench transactional"
cargo build --release --bin ohua-frequency --features "cli bench ohua"

"""

    for inp in inputs:
        stm_runs = ""
        freq_runs = ""
        for s in args[1:]:
            freq = math.ceil(int(inp.split('.')[0].split('-')[4][1:]) / 100 * int(s))
            # stm_runs += "target/release/stm --json --outdir stm_freq --runs {runs} inputs/{inp} --threads {n}\n".format(n=s, inp=inp, runs=runs)
            freq_runs += "target/release/ohua-frequency --json --outdir stm_freq --runs {runs} --frequency {n} inputs/{inp}\n".format(n=freq, inp=inp, runs=runs)

        bench_script += """# ------ {inp} ------
echo "Running benchmarks for {inp}"

echo -n "Sequential..."
# run sequential version
# target/release/simple_sequential inputs/{inp} --json -o stm_freq --runs {runs}

echo " done."

echo -n "STM..."
# run stm version
{stm_foo}

echo " done."

echo -n "Ohua..."
# run ohua
{ohua_foo}

echo " done ."


""".format(
            inp=inp, runs=runs, stm_foo=stm_runs, ohua_foo=freq_runs)

    print("Writing Benchmark Script")
    with open("bench-freq-stm.sh", 'w') as outfile:
        outfile.write(bench_script)

if __name__ == '__main__':
    main(sys.argv)
