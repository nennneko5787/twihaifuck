use std::env;
use std::fs;

mod saens;
mod twfck;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let mode = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename).expect("something went wrong reading the file");

    if mode.starts_with("twfck") {
        twfck::interpreter(contents.to_lowercase());
    } else if mode.starts_with("convert-twfck") {
        twfck::converter(filename.to_string(), contents);
    } else if mode.starts_with("saens") {
        saens::interpreter(contents.to_uppercase());
    } else if mode.starts_with("convert-saens") {
        saens::converter(filename.to_string(), contents);
    }
}
