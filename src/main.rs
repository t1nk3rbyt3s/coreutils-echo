use std::env::{self, args};

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        std::process::exit(0);
    }
}
