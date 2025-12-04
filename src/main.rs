use anyhow::{Error, Result};
use clap::Parser;
use rand::{self, SeedableRng};
use rand_chacha::ChaCha8Rng; // Useful for deterministic RNG
use rayon::prelude::*;
use rns_loot_sim::loot::treasuresphere::Colors;
use rns_loot_sim::writer;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf; // Anyhow

/// Program that simulates a number of games in Rabbit & Steel and writes items found
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of game runs (samples)
    #[arg(short = 'n', long, default_value_t = 1, value_parser(clap::value_parser!(u64).range(1..=500000)))]
    run_count: u64,

    /// Player count
    #[arg(short, long, default_value_t = 1, value_parser(clap::value_parser!(u64).range(1..=4)))]
    player_count: u64,

    /// Output file (csv), if not used, print to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// Use a positive integer seed for RNG (non-compliant)
    #[arg(short, long)]
    seed: Option<u64>,
    // /// Use indices instead of item names (it_[NAME])
    // #[arg(short, long, action)]
    // indices_for_items: bool,

    // /// Use no headers
    // #[arg(short, long, action)]
    // no_headers: bool,

    // /// Use relative headers (i.e. in 1p, exclude it_{2..=5}_{3..=4})
    // ///
    // /// Otherwise, default to absolute headers that make mixing mixed player
    // /// count data much easier
    // #[arg(short, long, action)]
    // relative_headers: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let player_count = args.player_count as usize; // Avoid recasting if on 32-bit

    let mut wtr = writer::Writer::from_writer(Vec::with_capacity(args.run_count as usize));
    writer::field_wtr_headers(&mut wtr, &false, &player_count)?;

    // Deterministic
    let data: Vec<(Vec<Colors>, Vec<usize>)> = (0..args.run_count as usize)
        .into_par_iter()
        .map(|i| {
            let mut seed = match args.seed {
                Some(val) => ChaCha8Rng::seed_from_u64(val),
                None => ChaCha8Rng::from_os_rng(),
            };
            seed.set_stream(i as u64); // Makes the seed deterministic despite threads
            let ts: Vec<Colors> = rns_loot_sim::generate_ts(&mut seed).unwrap();
            let it: Vec<usize> = rns_loot_sim::generate_it(&ts, &mut seed, &player_count).unwrap();
            (ts, it)
        })
        .collect();

    data.iter()
        .for_each(|(t, i)| writer::field_wtr(&mut wtr, t, i, &false, &player_count).unwrap());

    if let Some(path) = args.output_file {
        let mut file = File::create(path)?;
        file.write_all(&wtr.into_inner()?)?;
    } else {
        println!("{}", String::from_utf8(wtr.into_inner()?)?);
    }

    Ok(())
}
