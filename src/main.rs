use std::env;
use raytracer::write::{write_image};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Incorrect arguments, usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    write_image(256, 256, filename)
}
