use std::env::args;

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        std::process::exit(0);
    }

    let output = &args[1..].join(" ");

    println!("{output}");
}
