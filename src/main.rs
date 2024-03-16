use pokemon_icat::args::ProgramArgs;

use clap::Parser;
use rand::{prelude::IteratorRandom, Rng};

const MAX_GEN: usize = 10;
const GENERATIONS: [&str; MAX_GEN] = ["1", "2", "3", "4", "5", "6", "7", "8", "Hisui", "9"];
const ROMAN_NUMERALS: [&str; MAX_GEN] = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "Hisui", "IX"];
const GEN_RANGES: [(usize, usize); MAX_GEN] = [
    (1, 151),
    (152, 251),
    (252, 386),
    (387, 493),
    (494, 649),
    (650, 721),
    (722, 809),
    (810, 898),
    (899, 905),
    (906, 1025),
];

fn get_random_pokemon<R: Rng + ?Sized + Clone>(rng: &mut R, gen_idx: usize) -> Option<String> {
    todo!()
}

fn main() {
    let args = ProgramArgs::parse();

    // let mut rdr = csv::Reader::from_reader(io::stdin());
    //     for result in rdr.records() {
    //     // The iterator yields Result<StringRecord, Error>, so we check the
    //     // error here.
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    let mut pokemon = args.pokemon;
    let mut roman_gen = None;

    // TODO: make the two options conflict within clap
    match (pokemon, args.generations) {
        (Some(p), None) => {
            todo!()
        },
        (None, Some(gens)) => {
            todo!()
        },
        (_, _) => {
            let mut rng = rand::thread_rng();

            let gen_idx = (0..=MAX_GEN).choose(&mut rng).unwrap();

            roman_gen = Some(ROMAN_NUMERALS[gen_idx]);

            pokemon = get_random_pokemon(&mut rng, gen_idx);
        },
    }

    match (pokemon, roman_gen) {
        (Some(p), Some(r)) => println!("{} - {}", p, r),
        (_, _) => todo!(),
    }
}
