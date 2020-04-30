import sys


def generate_ohuac(template, size):
    tm_list = ""
    path_parts = ""
    part_list = ""
    for i in range(size):
        tm_list += "tm{}, ".format(i)
        part_list += "part{}, ".format(i)
        path_parts += """    let part{num} = for pair in tm{num} {{
        find_path(maze, pair)
    }};
""".format(num=i)

    fil = "../split{}_frequency.ohuac".format(size)
    print("Generating", fil)
    with open(fil, 'w') as outfile:
        outfile.write(
            template.replace("{tmlist}", tm_list.rstrip(', ')).replace(
                "{pathparts}", path_parts).replace("{partlist}",
                                                   part_list.rstrip(', ')))


def generate_rustfile(rust_template, size):
    splitup_args = "\n    Vec<(Point, Point)>," * size
    splitup_pops = "\n        paths_to_map.pop().unwrap()," * size

    join_args = "mut v1: Vec<Option<Path>>,"
    join_appends = ""
    for i in range(2, size + 1):
        join_args += "\n    mut v{}: Vec<Option<Path>>,".format(i)
        join_appends += "\n    v1.append(&mut v{});".format(i)

    fil = "../../bin/ohua-split{}-freq.rs".format(size)
    print("Generating", fil)
    with open(fil, 'w') as outfile:
        outfile.write(
            rust_template.replace("{_py_size_}", str(size)).replace(
                "{_py_splitup_args_}", splitup_args).replace(
                    "{_py_splitup_pops_}", splitup_pops).replace(
                        "{_py_join_args_}",
                        join_args).replace("{_py_join_appends_}",
                                           join_appends))


def generate_shellscript(sizes):
    # test parameters
    inputs = [
        # "random-x32-y32-z3-n96.txt", "random-x48-y48-z3-n64.txt",
        "random-x128-y128-z5-n128.txt", "random-x256-y256-z5-n256.txt"
    ]
    runs = "30"

    # fixe frequencies
    frequencies = [1, 2, 3, 4, 6]

    with open("./template_comb.sh") as f:
        template = f.read()

    build_calls = ""
    for s in sizes:
        build_calls += "cargo build --release --bin ohua-split{}-freq --features \"cli ohua\"\n".format(s)
        # build_calls += "cargo build --release --bin ohua-split{} --features \"cli ohua\"\n".format(
            # s)

    executions = ""
    for inp in inputs:
        executions += """# ------ {inp} ------
echo "Running benchmarks for {inp}"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/{inp} --json -o split_freq --runs {runs}

echo " done."

# echo -n "Ohua (split)"
# run ohua
""".format(
            inp=inp, runs=runs)
#         split_calls = ""
#         for s in sizes:
#             split_calls += """
# target/release/ohua-split{n} inputs/{inp} --json -o split_freq --runs {runs}
# echo -n "." """.format(
#                 n=s, inp=inp, runs=runs)
#         executions += split_calls
#         executions += """
# echo " done."


# """
        executions += """echo -n "Ohua (split-freq)"
# run ohua
"""
        for s in sizes:
            split_calls = ""
            for f in frequencies:
                split_calls += """
target/release/ohua-split{n}-freq inputs/{inp} --json -o split_freq --runs {runs} -f {fr}
echo -n "." """.format(
                n=s, inp=inp, runs=runs, fr=int(f)*int(s))
            executions += split_calls
        executions += """
echo " done."


"""
#         executions += """echo -n "Ohua (freq)"
# # run ohua
# """
#         split_calls = ""
#         for f in frequencies:
#             split_calls += """
# target/release/ohua-frequency inputs/{inp} --json -o split_freq --runs {runs} -f {fr}
# echo -n "." """.format(
#                 inp=inp, runs=runs, fr=f)
#             executions += split_calls
#             executions += """
# echo " done."
#
#
# """

# TODO: das oben fertig

    print("Writing Benchmark Script")
    with open("../../../bench-split-freq.sh", 'w') as outfile:
        outfile.write(
            template.format(build_calls=build_calls, executions=executions))


def main(args):
    if len(args) < 2:
        print("[Error] Too few arguments!\nUsage: ./generate_combined.py [split sizes]")
        return

    with open("./template_comb.ohuac") as f:
        ohuac_template = f.read()
    with open("./template_comb.rs") as rf:
        rust_template = rf.read()

    for s in args[1:]:
        size = int(s)
        assert (size > 0)

        generate_ohuac(ohuac_template, size)
        generate_rustfile(rust_template, size)

    generate_shellscript(args[1:])


if __name__ == '__main__':
    main(sys.argv)
