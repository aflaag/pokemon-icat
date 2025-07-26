extern crate ansi_colours;

use ansi_colours::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let index = args[1].parse::<u8>().unwrap();
        println!("{:-3}: {:?}", index, rgb_from_ansi256(index));
    } else if args.len() == 4 {
        let rgb = (
            args[1].parse::<u8>().unwrap(),
            args[2].parse::<u8>().unwrap(),
            args[3].parse::<u8>().unwrap(),
        );
        let index = ansi256_from_rgb(rgb);
        println!("{:?} ~ {:-3} {:?}", rgb, index, rgb_from_ansi256(index));
    } else {
        eprintln!("usage: convert ( <index> | <r> <g> <b> )");
        std::process::exit(1);
    }
}
