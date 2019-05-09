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

    fil = "../transact_split{}.ohuac".format(size)
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

    fil = "../../bin/ohua-split{}.rs".format(size)
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
        "random-x32-y32-z3-n96.txt", "random-x48-y48-z3-n64.txt",
        "random-x128-y128-z5-n128.txt", "random-x256-y256-z5-n256.txt"
    ]
    runs = "30"

    with open("./template.sh") as f:
        template = f.read()

    build_calls = ""
    for s in sizes:
        build_calls += "cargo build --release --bin ohua-split{} --features \"cli bench ohua\"\n".format(
            s)

    executions = ""
    for inp in inputs:
        executions += """# ------ {inp} ------
echo "Running benchmarks for {inp}"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/{inp} --json --runs {runs}

echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua inputs/{inp} --json --runs {runs}""".format(inp=inp,
                                                                runs=runs)
        split_calls = ""
        for s in sizes:
            split_calls += "\ntarget/release/ohua-split{n} inputs/{inp} --json --runs {runs}".format(
                n=s, inp=inp, runs=runs)
        executions += split_calls
        executions += """
echo " done."


"""

    print("Writing Benchmark Script")
    with open("../../../bench-split-versions.sh", 'w') as outfile:
        outfile.write(
            template.format(build_calls=build_calls, executions=executions))


def main(args):
    if len(args) < 2:
        print("[Error] Too few arguments!\nUsage: ./generate.py [split sizes]")
        return

    with open("./template.ohuac") as f:
        ohuac_template = f.read()
    with open("./template.rs") as rf:
        rust_template = rf.read()

    for s in args[1:]:
        size = int(s)
        assert (size > 1)

        generate_ohuac(ohuac_template, size)
        generate_rustfile(rust_template, size)

    generate_shellscript(args[1:])


if __name__ == '__main__':
    main(sys.argv)
