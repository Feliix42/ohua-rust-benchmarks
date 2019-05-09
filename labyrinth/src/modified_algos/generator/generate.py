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


if __name__ == '__main__':
    main(sys.argv)
