use anyhow::Error;
use clap::Parser;
use std::io;

/// Program that simulates a thousand of games and checks if items are found
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of game runs (samples)
    #[arg(short=n, long, default_value_t = 30)]
    run_count: u16,

    /// Player count
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=4))]
    player_count: u8,

    /// Output file (csv), if not used, print to stdout
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let game_count = &args.runcount?;
    let player_count = &args.playercount?;

    let mut wtr = match &args.output_file {
        None => csv::Writer::from_writer(io::stdout()),
        Some(file) => csv::Writer::from_path(file),
    };

    for _ in game_count {}
}

// fn run() -> Result<(), Error> {}

/// Format of each Game (Row of a CSV)
///
/// Doing it this way in-case we'd like to read to .json or
/// other file formats.
///
/// 1. player_count represents number of players and
/// dictates item choices.
/// 2. ts_{0..6} represents treasuresphere, i.e. Opal
/// 3. it_{0..5}_{0..4} represents items found, i.e. it_golden_katana
struct Game {
    player_count: u8,
    stages: Vec<Stage>,
}

enum Treasuresphere {
    Normal0,
    Normal1,
    Normal2,
    Opal,
    Sapphire,
    Ruby,
    Garnet,
    Emerald,
}

/// Doing it this way should be speedy before writing (slightly)
impl ToString for Treasuresphere {
    fn to_string(&self) -> String {
        match self {
            Treasuresphere::Normal0 | Treasuresphere::Normal1 | Treasuresphere::Normal2 => "normal",
            Treasuresphere::Opal => "opal",
            Treasuresphere::Sapphire => "sapphire",
            Treasuresphere::Ruby => "ruby",
            Treasuresphere::Garnet => "garnet",
            Treasuresphere::Emerald => "emerald",
        }
    }
}

/// In each stage is a treasuresphere and the corresponding loot found
struct Stage {
    treasuresphere: Treasuresphere,
}

impl Stage {
    fn new() -> Stage {
        todo!("Not implemented yet");
    }
}

/// Module to call constants based on player count
mod PlayerCount {
    pub static ONE: &'static [u8] = &[5, 5, 3, 3, 3, 3];
    pub static TWO: &'static [u8] = &[5, 5, 4, 4, 4, 4];
    pub static THREE: &'static [u8] = &[5, 5, 4, 4, 4, 4];
    pub static FOUR: &'static [u8] = &[5, 5, 5, 5, 5, 5];
}

/// Generate btreemaps/hashmaps for each lootsphere
mod Key {}
