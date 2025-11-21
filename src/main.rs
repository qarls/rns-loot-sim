mod loot; //phf hashmaps and Vanilla game constants
use loot::{IT_COUNT, TS_COUNT}; // vanilla constants for item count and ts count in 1.4.5
use anyhow::{bail, Context, Error, Result};
use clap::Parser;
use csv::Writer;
use rand::{seq::SliceRandom, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng; // Useful for deterministic RNG
use std::fs::File;

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

    /// Use a positive interger (u64) seed for RNG (non-compliant)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Use no headers
    #[arg(short, long, action)]
    no_headers: bool,

    /// Use relative headers (i.e. in 1p, exclude it_4_{3,4} and it_5_{3,4})
    /// 
    /// Otherwise, default to absolute headers that make mixing mixed player
    /// count data much easier
    #[arg(short, long, action)]
    relative_headers: bool,
    }

// Probs going to impliment tokio once I confirm this working
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let game_count = args.run_count;
    let player_count = args.player_count;
    let mut seed = match args.seed {
        Some(val) => ChaCha8Rng::seed_from_u64(val),
        None => ChaCha8Rng::from_seed(Default::default()),
    };

    let mut wtr = Writer::from_writer(vec![]);
    if !args.no_headers { field_wtr_headers(wtr, args.relative_headers, player_count); };
    for _ in 0..game_count {
    }
    Ok(())
}

// pub struct ShuffleCha(ChaCha8Rng);

// impl Rng + ?Sized for ShuffleCha {
//     type Seed =

// }

// fn shuffle(&mut )}

/// Writes the headers for our CSV file
///
/// I've included an absolute bool where for 1-3p
/// you can include either 1p
fn field_wtr_headers(mut wtr: Writer<Vec<u8>>, relative: bool, player_count: u8) -> Result<(), Error> {
    // player_count +
    let loot_count = 
    let headers = if relative {
        Vec::with_capacity(1 + *TS_COUNT + loot_counts.into_iter().sum() as usize)
    } else {
        Vec::with_capacity(1 + *TS_COUNT + )
    }
    
    Ok(())
}

fn field_wtr(mut wtr: Writer<Vec<u8>>, player_count: &u8) -> Result<(), Error> {
    wtr.write_field(todo!())?;
    todo!();
}

/// Generates a set of random treasurespheres per game
// I highly suspect this will work inconsistently seeded with multithreaded
fn generate_ts(mut seed: &mut ChaCha8Rng) -> Vec<Treasuresphere> {
    let count = *TS_COUNT;
    let mut ts = Vec::with_capacity(count);

    let mut nums: Vec<u8> = (0..8).collect();
    nums.shuffle(&mut seed);
    for i in 0..count {
        ts.push(Treasuresphere::from_index(nums[i]));
    } //Omitting the last two numbers in array

    return ts;
}

/// Generates a set of random items per game
///
/// The Result-Vec returned are string values of item names
/// and are deemed "relative", i.e.
/// - in 1P, items 4_2 [18] and 5_0 [19] will sit next to each other,
/// where items 4_{3,4} are not evaulated
/// - in 4p, items 4_4 [24] and 5_0 [25] next to each other
#[allow(unused_variables)]
fn generate_it(
    treasurespheres: Vec<Treasuresphere>,
    mut seed: &mut ChaCha8Rng,
    player_count: u8,
) -> Result<Vec<&'static str>, Error> {
    // Almost sure this will break if you try to put a non 6-value
    let loot_counts: [usize; *TS_COUNT] = loot::player_loot::loot_counts(player_count)?;
    let loot_count_sum: usize = loot_counts.into_iter::<usize>().sum() as usize;

    // Moved externally to avoid rerolling this every ts or it
    // Should always reset the counter 'p', otherwise it will be biased towards previous flavoured ts-incompatible items
    //
    // Not sure if it's faster to check if items are unique or to remove items from item pool vector
    // (i.e. moving items from the back one item forward every single time item is drawn)
    let mut itempool: Vec<usize> = (0..200).collect();
    itempool.shuffle(&mut seed);

    let mut items_found = Vec::with_capacity(6);
    for t in 0..*TS_COUNT {
        for loot_count in loot_counts {
            //i.e. for 1p, this goes 5,5,3,3,3,3
            let mut p: usize = 0; // Count through item indices in Pool of total itempool
            let treasuresphere = treasurespheres
                .get(t)
                .expect("Invalid treasuresphere indexed.");

            'find_valid_item: loop {
                if treasuresphere.in_ts_pool(&p) {
                    // Checks if item can be found in current ts, otherwise reset and advance counter
                    for f in items_found {
                        // Checks if already prsent or not duplicate
                        if itempool
                            .get(p)
                            .expect("Vector index exceeded in generate_it")
                            != f
                        {
                            items_found.push(
                                itempool
                                    .get(p)
                                    .expect("Items found push in generate_it failed"),
                            ); // Add item to our items found on unique
                            p += 1; // advance item counter in current treasuresphere
                        }
                    }
                } else {
                    p += 1; // advance item counter in current treasuresphere until item in treasuresphere is found
                    continue 'find_valid_item;
                }
            }
        }
    }

    let items_found_str: Vec<&'static str> = items_found
        .iter()
        .map(|x| {
            *loot::treasuresphere::ITEM_NAMES
                .get(x)
                .expect("Index of item index to names in generate_it() misindexed")
        })
        .collect::<Vec<&str>>();
    return Ok(items_found_str);
}

enum Treasuresphere {
    Normal, // Reminder that you can find Normal 3 times
    Opal,
    Sapphire,
    Ruby,
    Garnet,
    Emerald,
}

impl Treasuresphere {
    /// Involves weighted indices, for use when generating treasurespheres
    fn from_index(index: u8) -> Self {
        match index {
            0 | 1 | 2 => Treasuresphere::Normal,
            3 => Treasuresphere::Opal,
            4 => Treasuresphere::Sapphire,
            5 => Treasuresphere::Ruby,
            6 => Treasuresphere::Garnet,
            7 => Treasuresphere::Emerald,
            _ => panic!("Unexpected treasuresphere index: {}", index),
        }
    }

    fn in_ts_pool(&self, loot: &usize) -> bool {
        let option = match &self {
            Self::Normal if *loot < IT_COUNT.clone() as usize => return true,
            Self::Normal => panic!("Loot index is out of bounds: {}", loot),
            Self::Opal => loot::treasuresphere::IS_OPAL.get(loot),
            Self::Sapphire => loot::treasuresphere::IS_SAPPHIRE.get(loot),
            Self::Ruby => loot::treasuresphere::IS_RUBY.get(loot),
            Self::Garnet => loot::treasuresphere::IS_GARNET.get(loot),
            Self::Emerald => loot::treasuresphere::IS_EMERALD.get(loot),
        };

        match option {
            None => panic!("Loot index out of bounds: {}", loot),
            Some(val) => return *val,
        }
    }
}

// ToString feels so Rusty :D
impl ToString for Treasuresphere {
    fn to_string(&self) -> String {
        match self {
            Treasuresphere::Normal => "normal",
            Treasuresphere::Opal => "opal",
            Treasuresphere::Sapphire => "sapphire",
            Treasuresphere::Ruby => "ruby",
            Treasuresphere::Garnet => "garnet",
            Treasuresphere::Emerald => "emerald",
        }
        .to_string()
    }
}
