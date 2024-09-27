use std::fs::File;

use clap::Parser;
use csv::Reader;
use rand::{prelude::IteratorRandom, Rng};
use viuer::{print_from_file, Config};

const TYPES: [&str; 18] = [
    "🏳️", "🔥", "🌊", "⚡", "🍃", "🌨️", "🥊", "💀", "🌎", "🐦", "🔮", "🐞", "🗿", "👻", "🐲", "🌑",
    "🔩", "🧚",
];
const INVALID_TYPE_STR: &str = "🚫";

const GENERATIONS: [(&str, &str); 10] = [
    ("1", "I Generation"),
    ("2", "II Generation"),
    ("3", "III Generation"),
    ("4", "IV Generation"),
    ("5", "V Generation"),
    ("6", "VI Generation"),
    ("7", "VII Generation"),
    ("8", "VIII Generation"),
    ("Hisui", "Hisui Region"),
    ("9", "IX Generation"),
];

/// Show Pokémons inside your terminal!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ProgramArgs {
    /// pick a pokemon to show
    #[clap(short, long, value_parser)]
    pub pokemon: Option<String>,

    /// randomly choose a pokemon from the given generations
    /// currently supported generations are: [1, 2, 3, 4, 5, 6, 7, 8, Hisui, 9]
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',', verbatim_doc_comment)]
    #[arg(conflicts_with = "pokemon")]
    pub generations: Option<Vec<String>>,

    /// suppress the Pokémon info
    #[clap(short, long, value_parser)]
    pub quiet: bool,

    /// change the Pokémon size
    #[clap(long, default_value = "1.0", value_parser = check_scale)]
    pub scale: f32,

    /// use a fixed height for every Pokémon
    #[clap(long, value_parser = check_height)]
    #[arg(conflicts_with = "scale")]
    pub height: Option<u32>,

    /// makes the pokemon shiny
    #[clap(long, default_value = "8192", value_parser)]
    pub shiny_probability: u32,
}

fn check_scale(scale: &str) -> Result<f32, String> {
    if let Ok(s) = scale.parse::<f32>() {
        if s >= 0.5 {
            return Ok(s);
        }
    }

    Err(String::from("scale factor must be at least 0.5."))
}

fn check_height(height: &str) -> Result<u32, String> {
    if let Ok(h) = height.parse::<u32>() {
        if h >= 2 {
            return Ok(h);
        }
    }

    Err(String::from("height must be at least 2."))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Pokemon {
    name: String,
    generation: String,
    height: u32,
    typing: String,
}

fn get_pokemon(pokemon_name: &str, pokemons: &[Pokemon]) -> Pokemon {
    pokemons
        .iter()
        .find(|p| p.name == pokemon_name)
        .expect("the given pokemon does not exist")
        .clone()
}

fn get_random_pokemon<R: Rng + Clone>(
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

fn gen_label(gen: &str) -> &str {
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

    home_path.push(".cache/pokemon-icat");
    home_path.push("pokemon_data.csv");

    let pokemon_data = File::open(&home_path).expect("missing `pokemon_data.csv` file");

    let pokemons: Vec<Pokemon> = Reader::from_reader(pokemon_data)
        .deserialize()
        .map(|p| p.expect("`pokemon_data.csv` is corrupted"))
        .collect();

    let pokemon = if let (Some(n), None) = (&args.pokemon, &args.generations) {
        get_pokemon(n.as_str(), &pokemons)
    } else {
        let mut rng = rand::thread_rng();

        get_random_pokemon(&mut rng, &pokemons, &args.generations).unwrap()
    };

    if !args.quiet {
        println!(
            "{} {}",
            if pokemon.typing.is_empty() {
                INVALID_TYPE_STR.to_string()
            } else {
                pokemon
                    .typing
                    .split(' ')
                    .map(|t| TYPES[t.parse::<usize>().expect("`pokemon_data.csv` is corrupted")])
                    .collect::<String>()
            },
            pokemon.name,
        );
    }

    home_path.pop();

    home_path.push("pokemon-icons");
    let luck_num = rand::thread_rng().gen_range(0..args.shiny_probability);
    if luck_num == 0 {
        home_path.push("shiny");
    } else {
        home_path.push("normal");
    }

    home_path.push(format!("{}.png", pokemon.name));

    let conf = Config {
        absolute_offset: false,
        height: Some(if let Some(h) = args.height {
            h
        } else {
            (pokemon.height as f32 * args.scale).round() as u32
        }),
        ..Default::default()
    };

    // println!("{:?}", home_path);
    print_from_file(&home_path, &conf).expect("failed to show the image");

    println!("{}", gen_label(&pokemon.generation));
}
