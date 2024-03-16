use clap::Parser;
use csv::Reader;
use rand::{prelude::IteratorRandom, Rng};
use std::fs::File;

const MAX_GEN: usize = 10;
const GENERATIONS: [(&str, &str); MAX_GEN] = [
    ("1", "I"),
    ("2", "II"),
    ("3", "III"),
    ("4", "IV"),
    ("5", "V"),
    ("6", "VI"),
    ("7", "VII"),
    ("8", "VIII"),
    ("Hisui", "Hisui"),
    ("9", "IX"),
];

/// TODO: DO ME
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ProgramArgs {
    #[clap(short, long, value_parser)]
    pub pokemon: Option<String>,

    #[clap(long, value_parser)]
    pub show_info: bool,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    pub generations: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Pokemon {
    name: String,
    generation: String,
}

fn get_pokemon_gen<'a>(pokemon_name: &'a str, pokemons: &'a [Pokemon]) -> &'a str {
    pokemons
        .iter()
        .find(|p| p.name == pokemon_name)
        .expect("the given pokemon does not exist")
        .generation
        .as_str()
}

fn get_random_pokemon<R: Rng + ?Sized + Clone>(
    rng: &mut R,
    pokemons: &[Pokemon],
    gens: &Option<Vec<String>>,
) -> Option<Pokemon> {
    pokemons
        .iter()
        .filter(|p| {
            if let Some(gs) = &gens {
                gs.contains(&p.generation)
            } else {
                true
            }
        })
        .choose(rng)
        .cloned()
}

fn gen_to_roman(gen: &str) -> &str {
    GENERATIONS.iter().find(|(g, _)| *g == gen).unwrap().1
}

fn main() {
    let args = ProgramArgs::parse();

    if let Some(gens) = &args.generations {
        if gens
            .iter()
            .any(|gen_arg| !GENERATIONS.iter().any(|(gen, _)| gen_arg == gen))
        {
            panic!("invalid region.");
        }
    }

    let pokemon_data = File::open("pokemon_data.csv").expect("missing `pokemon_data.csv` file.");

    let pokemons: Vec<Pokemon> = Reader::from_reader(pokemon_data)
        .deserialize()
        .map(|p| p.expect("`pokemon_data.csv` is corrupted"))
        .collect();

    // TODO: use conflicts_with in clap to avoid checking for (Some, Some)
    let pokemon = if let (Some(n), None) = (&args.pokemon, &args.generations) {
        Pokemon {
            name: n.clone(),
            generation: get_pokemon_gen(n.as_str(), &pokemons).to_string(),
        }
    } else {
        let mut rng = rand::thread_rng();

        get_random_pokemon(&mut rng, &pokemons, &args.generations).unwrap()
    };

    if args.show_info {
        println!(
            "{} - {}",
            pokemon.name,
            gen_to_roman(pokemon.generation.as_str())
        );
    }
}
