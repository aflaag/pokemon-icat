use std::{env, fs::File, path::PathBuf};

use clap::{Parser, ValueEnum};
use csv::Reader;
use rand::{prelude::IteratorRandom, Rng};
use viuer::{print_from_file, Config};

const TYPES: [&str; 18] = [
    "🏳️", "🔥", "🌊", "⚡", "🍃", "🌨️", "🥊", "💀", "🌎", "🐦", "🔮", "🐞", "🗿", "👻", "🐲", "🌑",
    "🔩", "🧚",
];
const INVALID_TYPE_STR: &str = "🚫";

// generation id, then its label in each `Lang` (en, fr, ja)
const GENERATIONS: [(&str, [&str; 3]); 10] = [
    ("1", ["I Generation", "Génération I", "第1世代"]),
    ("2", ["II Generation", "Génération II", "第2世代"]),
    ("3", ["III Generation", "Génération III", "第3世代"]),
    ("4", ["IV Generation", "Génération IV", "第4世代"]),
    ("5", ["V Generation", "Génération V", "第5世代"]),
    ("6", ["VI Generation", "Génération VI", "第6世代"]),
    ("7", ["VII Generation", "Génération VII", "第7世代"]),
    ("8", ["VIII Generation", "Génération VIII", "第8世代"]),
    ("Hisui", ["Hisui Region", "Région de Hisui", "ヒスイ地方"]),
    ("9", ["IX Generation", "Génération IX", "第9世代"]),
];

/// language used for the Pokémon names and the generation labels
#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Lang {
    En,
    Fr,
    Ja,
}

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

    /// language of the Pokémon info
    #[clap(
        short,
        long,
        value_enum,
        env = "POKEMON_ICAT_LANG",
        default_value = "en"
    )]
    pub lang: Lang,
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
    // PokeAPI identifier, also the icon file name
    name: String,
    generation: String,
    height: u32,
    typing: String,
    // localized names, added by update_names.py (may be missing in older data)
    #[serde(default)]
    name_en: String,
    #[serde(default)]
    name_fr: String,
    #[serde(default)]
    name_ja: String,
}

impl Pokemon {
    fn localized_names(&self) -> [&str; 3] {
        [&self.name_en, &self.name_fr, &self.name_ja]
    }

    fn display_name(&self, lang: Lang) -> &str {
        let name = self.localized_names()[lang as usize];
        if name.is_empty() {
            &self.name
        } else {
            name
        }
    }
}

fn get_pokemon(pokemon_name: &str, pokemons: &[Pokemon]) -> Pokemon {
    pokemons
        .iter()
        .find(|p| {
            p.name == pokemon_name
                || p.localized_names()
                    .iter()
                    .any(|n| !n.is_empty() && n.to_lowercase() == pokemon_name.to_lowercase())
        })
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

fn gen_label(gen: &str, lang: Lang) -> &str {
    GENERATIONS.iter().find(|(g, _)| *g == gen).unwrap().1[lang as usize]
}

fn main() {
    let mut root_path =
        PathBuf::from(env::var("POKEMON_ICAT_DATA").expect("$POKEMON_ICAT_DATA is not set"));

    let args = ProgramArgs::parse();

    if let Some(gens) = &args.generations {
        if gens
            .iter()
            .any(|gen_arg| !GENERATIONS.iter().any(|(gen, _)| gen_arg == gen))
        {
            panic!("invalid region.");
        }
    }

    root_path.push("pokemon_data.csv");

    let pokemon_data = File::open(&root_path).expect("missing `pokemon_data.csv` file");

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

    let luck_num = rand::thread_rng().gen_range(0..args.shiny_probability);

    let is_shiny = luck_num == 0;

    if !args.quiet {
        println!(
            "{} {} {}",
            if pokemon.typing.is_empty() {
                INVALID_TYPE_STR.to_string()
            } else {
                pokemon
                    .typing
                    .split(' ')
                    // out-of-range types (e.g. MissingNo's "18") show as invalid
                    .map(|t| {
                        *TYPES
                            .get(t.parse::<usize>().expect("`pokemon_data.csv` is corrupted"))
                            .unwrap_or(&INVALID_TYPE_STR)
                    })
                    .collect::<String>()
            },
            pokemon.display_name(args.lang),
            if is_shiny { "✨" } else { "" }
        );
    }

    root_path.pop();
    root_path.push("pokemon-icons");

    if is_shiny {
        root_path.push("shiny");
    } else {
        root_path.push("normal");
    }

    root_path.push(format!("{}.png", pokemon.name));

    let conf = Config {
        absolute_offset: false,
        height: Some(if let Some(h) = args.height {
            h
        } else {
            (pokemon.height as f32 * args.scale).round() as u32
        }),
        ..Default::default()
    };

    print_from_file(&root_path, &conf).expect("failed to show the image");

    if !args.quiet {
        println!("{}", gen_label(&pokemon.generation, args.lang));
    }
}
