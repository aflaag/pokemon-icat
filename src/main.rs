use clap::Parser;
use csv::Reader;
use rand::{prelude::IteratorRandom, Rng};
use std::fs::File;

// const CSV_PATH: &str = "/home/aless/.pokemon-icat/pokemon_data.csv";

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

/// Show Pokémons inside your terminal!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ProgramArgs {
    /// pick a pokemon to show
    #[clap(short, long, value_parser)]
    pub pokemon: Option<String>,

    /// randomly choose a pokemon from the given generations;
    /// currently supported generations are: [1, 2, 3, 4, 5, 6, 7, 8, Hisui, 9]
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    #[arg(conflicts_with = "pokemon")]
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
    let mut home_path = home::home_dir().expect("unable to get home dir");

    assert!(!home_path.as_os_str().is_empty(), "unable to get home dir");

    let args = ProgramArgs::parse();

    if let Some(gens) = &args.generations {
        if gens
            .iter()
            .any(|gen_arg| !GENERATIONS.iter().any(|(gen, _)| gen_arg == gen))
        {
            panic!("invalid region.");
        }
    }

    home_path.push(".pokemon-icat/pokemon_data.csv");

    let pokemon_data = File::open(home_path).expect("missing `pokemon_data.csv` file");

    let pokemons: Vec<Pokemon> = Reader::from_reader(pokemon_data)
        .deserialize()
        .map(|p| p.expect("`pokemon_data.csv` is corrupted"))
        .collect();

    let pokemon = if let (Some(n), None) = (&args.pokemon, &args.generations) {
        Pokemon {
            name: n.clone(),
            generation: get_pokemon_gen(n.as_str(), &pokemons).to_string(),
        }
    } else {
        let mut rng = rand::thread_rng();

        get_random_pokemon(&mut rng, &pokemons, &args.generations).unwrap()
    };

    println!(
        "{} - {}",
        pokemon.name,
        gen_to_roman(pokemon.generation.as_str())
    );
}
