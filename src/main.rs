mod loot; //phf hashmaps
use anyhow::Error;
use clap::Parser;
use csv::Writer;

/// Program that simulates a thousand of games and checks if items are found
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of game runs (samples)
    #[arg(short = 'n', long, default_value_t = 30)]
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

    let mut wtr = Writer::from_writer(vec![]);

    for _ in game_count {}

    Ok(())
}

fn field_wtr(wtr: Writer<Vec<u8>>, player_count: u8) -> Result<(), Error> {}

fn generate(wtr: Writer<Vec<u8>>) -> Result<(), Error> {}

/// Constant format of each Game (Row of a CSV)
///
/// Doing it this way in-case we'd like to read to .json or
/// other file formats.
///
/// 1. player_count represents number of players and
/// dictates item choices.
/// 2. ts_{0..6} represents treasuresphere, i.e. "opal"
/// 3. it_{0..5}_{0..4} represents items found, i.e. "it_golden_katana"
#[derive(serde::Serialize)]
struct Game<'a> {
    player_count: u8,
    ts_0: &'a str,
    ts_1: &'a str,
    ts_2: &'a str,
    ts_3: &'a str,
    ts_4: &'a str,
    ts_5: &'a str,
    it_0_0: &'a str,
    it_0_1: &'a str,
    it_0_2: &'a str,
    it_0_3: &'a str,
    it_0_4: &'a str,
    it_1_0: &'a str,
    it_1_1: &'a str,
    it_1_2: &'a str,
    it_1_3: &'a str,
    it_1_4: &'a str,
    it_2_0: &'a str,
    it_2_1: &'a str,
    it_2_2: &'a str,
    it_2_3: &'a str,
    it_2_4: &'a str,
    it_3_0: &'a str,
    it_3_1: &'a str,
    it_3_2: &'a str,
    it_3_3: &'a str,
    it_3_4: &'a str,
    it_4_0: &'a str,
    it_4_1: &'a str,
    it_4_2: &'a str,
    it_4_3: &'a str,
    it_4_4: &'a str,
    it_5_0: &'a str,
    it_5_1: &'a str,
    it_5_2: &'a str,
    it_5_3: &'a str,
    it_5_4: &'a str,
}

enum Treasuresphere {
    Normal, // Reminder that you can find Normal 3 times
    Opal,
    Sapphire,
    Ruby,
    Garnet,
    Emerald,
}

/// Doing it this way should be speedy before writing (slightly)
impl ToString for Treasuresphere {
    fn to_string(&self) -> &str {
        match self {
            Treasuresphere::Normal => "normal",
            Treasuresphere::Opal => "opal",
            Treasuresphere::Sapphire => "sapphire",
            Treasuresphere::Ruby => "ruby",
            Treasuresphere::Garnet => "garnet",
            Treasuresphere::Emerald => "emerald",
        }
    }
}

/// Module to call constants based on player count
mod PlayerCount {
    pub static ONE: &'static [u8] = &[5, 5, 3, 3, 3, 3];
    pub static TWO: &'static [u8] = &[5, 5, 4, 4, 4, 4];
    pub static THREE: &'static [u8] = &[5, 5, 4, 4, 4, 4];
    pub static FOUR: &'static [u8] = &[5, 5, 5, 5, 5, 5];
}
