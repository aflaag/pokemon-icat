use pokemon_icat::args::ProgramArgs;

use clap::Parser;
use csv::Reader;
use rand::{prelude::IteratorRandom, Rng};
use std::fs::File;

const MAX_GEN: usize = 10;
const GENERATIONS: [&str; MAX_GEN] = ["1", "2", "3", "4", "5", "6", "7", "8", "Hisui", "9"];
const ROMAN_NUMERALS: [&str; MAX_GEN] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "Hisui", "IX",
];
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

#[derive(Debug, Clone, serde::Deserialize)]
struct Pokemon {
    name: String,
    generation: String,
}

fn get_random_pokemon<R: Rng + ?Sized + Clone>(
    rng: &mut R,
    gen: &str,
    pokemons: &[Pokemon],
) -> Option<String> {
    pokemons
        .iter()
        .filter(|p| p.generation == gen)
        .choose(rng)
        .map(|p| p.name.clone())
}

fn get_pokemon_gen<'a>(pokemon_name: &'a str, pokemons: &'a [Pokemon]) -> &'a str {
    pokemons
        .iter()
        .find(|p| p.name == pokemon_name)
        .expect("the given pokemon does not exist")
        .generation
        .as_str()
}

fn main() {
    let args = ProgramArgs::parse();

    let pokemon_data = File::open("pokemon_data.csv").expect("missing `pokemon_data.csv` file.");

    let pokemons: Vec<Pokemon> = Reader::from_reader(pokemon_data)
        .deserialize()
        .map(|p| p.expect("`pokemon_data.csv` is corrupted"))
        .collect();

    let mut pokemon_name = args.pokemon;
    let roman_gen;

    // TODO: make the two options conflict within clap
    match (&pokemon_name, args.generations) {
        (Some(n), None) => {
            let gen = get_pokemon_gen(n.as_str(), &pokemons);

            roman_gen = ROMAN_NUMERALS[GENERATIONS
                .iter()
                .enumerate()
                .find(|(_, g)| **g == gen)
                .unwrap()
                .0];
        }
        (None, Some(gens)) => {
            todo!()
        }
        (_, _) => {
            let mut rng = rand::thread_rng();

            let gen_idx = (0..=MAX_GEN).choose(&mut rng).unwrap();

            roman_gen = ROMAN_NUMERALS[gen_idx];

            pokemon_name = get_random_pokemon(&mut rng, GENERATIONS[gen_idx], &pokemons);
        }
    }

    if args.show_info {
        println!("{} - {}", pokemon_name.unwrap(), roman_gen);
    }
}
