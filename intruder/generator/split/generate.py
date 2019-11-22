import sys

def generate_ohuac(num):
    with open('template/split_detector.ohuac') as f:
        template = f.read()
    
    dec_list = ""
    res_list = ""
    loop_list = ""

    for i in range(num):
        dec_list += "dec{}, ".format(i)
        res_list += "res{}, ".format(i)
        loop_list += """    let res{n} = for decoded in dec{n} {{
        analyze_stream(decoded)
    }};
""".format(n=i)

    fil = "../../src/algos/split{}_detector.ohuac".format(num)
    print("Generating", fil)
    with open(fil, 'w') as outfile:
        outfile.write(
            template.replace("{decs}", dec_list.rstrip(', ')).replace(
                "{loops}", loop_list).replace("{results}",
                                                   res_list.rstrip(', ')))

def generate_rust_file(num):
    with open('template/ohua_split.rs') as f:
        rust_template = f.read()

    splitup_args = "\n    VecDeque<DecodedPacket>," * num
    splitup_pops = "\n        paths_to_map.pop().unwrap()," * num

    join_args = "mut v1: Vec<(DecodedPacket, DetectorResult)>,"
    join_appends = ""
    for i in range(2, num + 1):
        join_args += "\n    mut v{}: Vec<(DecodedPacket, DetectorResult)>,".format(i)
        join_appends += "\n    v1.append(&mut v{});".format(i)

    fil = "../../src/bin/ohua-split{}.rs".format(num)
    print("Generating", fil)
    with open(fil, 'w') as outfile:
        outfile.write(
            rust_template.replace("{_py_size_}", str(num)).replace(
                "{_py_splitup_args_}", splitup_args).replace(
                    "{_py_splitup_pops_}", splitup_pops).replace(
                        "{_py_join_args_}",
                        join_args).replace("{_py_join_appends_}",
                                           join_appends))

def main(args):
    if len(args) < 2:
        print('[Error] Insufficient number of arguments! Invocation: ./generate.py [number(s)]')
        return
    
    for i in args[1:]:
        num = int(i)
        generate_ohuac(num)
        generate_rust_file(num)

    
if __name__ == '__main__':
    main(sys.argv)