pub mod front_end;
pub mod back_end;
pub mod utils;

pub use back_end::Format;

use front_end::{
    ByteStream,
};
use back_end::{
    X86Mode,
    Arch,
};

pub fn main<T: Iterator<Item = String>>(mut args: T) -> i32 {
    let mut mfile = None;
    let mut mout = None;
    let mut march = None;
    let mut mformat = None;
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "-h" | "--help" => {
                print_usage();
                return 0;
            },
            "-a" => match args.next() {
                Some(arg) => match &march {
                    &None => march = Some(match arg.as_ref() {
                        "x86" => X86Mode::X86,
                        "x86-64" | "x86_64" | "amd64" | "x64" => X86Mode::Amd64,
                        a => {
                            println!("Unsupported architecture {}.", a);
                            print_usage();
                            return 1;
                        },
                    }),
                    _ => {
                        println!("Architecture already passed.");
                        print_usage();
                        return 1;
                    },
                },
                _ => {
                    println!("Expecting one more argument after -a");
                    print_usage();
                    return 1;
                },
            },
            "-o" => match args.next() {
                Some(o) => match &mout {
                    &None => mout = Some(o),
                    _ => {
                        println!("Output file already passed.");
                        print_usage();
                        return 1;
                    },
                },
                _ => {
                    println!("Expecting one more argument after -o");
                    print_usage();
                    return 1;
                },
            },
            "-f" => match args.next() {
                Some(arg) => match &mformat {
                    &None => mformat = Some(match arg.as_ref() {
                        "elf" => Format::Elf,
                        "asm" => Format::Asm,
                        f => {
                            println!("Unsupported format {}.", f);
                            print_usage();
                            return 1;
                        },
                    }),
                    _ => {
                        println!("Format already passed.");
                        print_usage();
                        return 1;
                    },
                },
                _ => {
                    println!("Expecting one more argument after -f");
                    print_usage();
                    return 1;
                },
            },
            _ => match &mfile {
                &None => mfile = Some(arg),
                _ => {
                    println!("Input file already passed.");
                    print_usage();
                    return 1;
                },
            },
        }
    }
    let file = match mfile {
        Some(f) => f,
        _ => {
            print_usage();
            return 1;
        }
    };
    let out = match mout {
        Some(f) => f,
        _ => String::from("a.out"),
    };
    let arch = match march {
        Some(a) => a,
        _ => X86Mode::Amd64,
    };
    let format = match mformat {
        Some(f) => f,
        _ => Format::Elf,
    };
    let bs = match ByteStream::from_file(file.clone()) {
        Ok(bs) => bs,
        Err(e) => {
            println!("Error opening {}: {}", file, e);
            return -1;
        },
    };
    let tree = match front_end::parse(bs) {
        Ok(tree) => tree,
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
            return -1;
        }
    };
    match arch.generate(tree, format, out) {
        Err(e) => {
            println!("{}", e);
            -1
        },
        _ => 0,
    }
}

fn print_usage() {
    println!("bfc [options] file");
    println!("options:");
    print!  ("    -a X                      Sets the architecture to X, where X can be `x86` or `amd64`.");
    println!("Instead of `amd64`, `x86_64`, `x86-64` or `x64` could also be written. Must be defined only once.");
    print!  ("    -f X                      Sets the format to X, where X can be `asm` or `elf`.");
    println!(" Must be defined only once");
    print!  ("    -h, --help                Shows this help message and exits.");
    println!(" File argument is not necessary in this case.");
    println!("    -o X                      Sets output file to X. Must be defined only once.");
}
