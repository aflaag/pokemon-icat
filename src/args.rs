use clap::Parser;

// TODO pokemon and generations should be mutually exclusive
/// TODO: DO ME
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ProgramArgs {
    #[clap(short, long, value_parser)]
    pub pokemon: Option<String>,

    #[clap(short, long, value_parser)]
    pub show_info: bool,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    pub generations: Option<Vec<String>>,
}
