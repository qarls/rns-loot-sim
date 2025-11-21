mod loot; //phf hashmaps and Vanilla game constants
use anyhow::{Error, Result};
// use anyhow::{bail, Context, Error, Result};
use clap::Parser;
use csv::Writer;
use loot::{IT_COUNT, TS_COUNT}; // vanilla constants for item count and ts count in 1.4.5
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng; // Useful for deterministic RNG
use std::fs::File;
use std::io::Write;
// use std::io::prelude::Write;

/// Program that simulates a thousand of games and checks if items are found
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of game runs (samples)
    #[arg(short = 'n', long, default_value_t = 30)]
    run_count: u16,

    /// Player count
    #[arg(short, long, default_value_t = 1, value_parser(clap::value_parser!(u64).range(1..=4)))]
    player_count: u64,

    /// Output file (csv), if not used, print to stdout
    #[arg(short, long)]
    output_file: Option<String>,

    /// Use a positive interger (u64) seed for RNG (non-compliant)
    #[arg(short, long)]
    seed: Option<u64>,
    // /// Use no headers
    // #[arg(short, long, action)]
    // no_headers: bool,

    // /// Use relative headers (i.e. in 1p, exclude it_4_{3,4} and it_5_{3,4})
    // ///
    // /// Otherwise, default to absolute headers that make mixing mixed player
    // /// count data much easier
    // #[arg(short, long, action)]
    // relative_headers: bool,
}

// Probs going to impliment tokio once I confirm this working
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let game_count = args.run_count;
    let player_count = args.player_count as usize;
    let mut seed = match args.seed {
        Some(val) => ChaCha8Rng::seed_from_u64(val),
        None => ChaCha8Rng::from_seed(Default::default()),
    };

    let mut wtr = Writer::from_writer(vec![]);
    field_wtr_headers(&mut wtr, &false, &player_count)?;

    for _ in 0..game_count {
        let ts: Vec<Treasuresphere> = generate_ts(&mut seed);
        let it: Vec<&'static str> = generate_it(&ts, &mut seed, &player_count)?;
        field_wtr(&mut wtr, &ts, &it, &false, &player_count)?;
    }

    if let Some(file) = args.output_file {
        let mut file = File::create(file)?;
        file.write_all(&wtr.into_inner()?)?;
    } else {
        println!("{}", String::from_utf8(wtr.into_inner()?)?);
    }

    println!("{:?}", loot::treasuresphere::IS_OPAL.get(&6).clone());

    println!("{:?}", loot::treasuresphere::ITEM_NAMES.get(&6).clone());

    Ok(())
}

// pub struct ShuffleCha(ChaCha8Rng);

// impl Rng + ?Sized for ShuffleCha {
//     type Seed =

// }

// fn shuffle(&mut )}

/// Writes the headers for our CSV file
///
/// I've included an "relative" bool where for 1-3p
/// Where in 1p, this excludes the headers  {2..=5}_{3,4}
fn field_wtr_headers(
    wtr: &mut Writer<Vec<u8>>,
    _relative: &bool,
    player_count: &usize,
) -> Result<(), Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count)?;

    if *_relative {
        todo!("Relative flag not priority.");
    } else {
        wtr.write_field("player_count")?;
        for t in 0..*TS_COUNT {
            let ts_t: String = format!("ts_{}", t);
            wtr.write_field(ts_t)?;
        }
        for t in 0..*TS_COUNT {
            for i in 0..*loot_counts
                .get(t)
                .expect("ts indexing exceeded bounds of loot_counts in field_wtr_headers().")
            {
                let it_t_i: String = format!("ts_{}_{}", t, i);
                wtr.write_field(it_t_i)?;
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    };

    Ok(())
}

fn field_wtr(
    wtr: &mut Writer<Vec<u8>>,
    treasurespheres: &Vec<Treasuresphere>,
    loot: &Vec<&str>,
    _relative: &bool,
    player_count: &usize,
) -> Result<(), Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count)?;

    assert_eq!(loot::player_loot::loot_sum(*player_count)?, loot.len()); // Simply checks if they're same length, should not fail

    if *_relative {
        todo!("Relative flag not priority.");
    } else {
        wtr.write_field(player_count.to_string())?;
        for t in 0..*TS_COUNT {
            let ts = treasurespheres
                .get(t)
                .expect("ts index exceeded the bounds of rolled treasurespheres in field_wtr().");
            wtr.write_field(ts.to_string())?;
        }

        let mut loot_index = 0;
        for t in 0..*TS_COUNT {
            for i in 0..*loot::IT_FOUND_MAX_PER_TS {
                //5
                let loot_count = loot_counts
                    .get(t)
                    .expect("ts indexing exceeded bounds of loot_counts in field_wtr().");

                let item = if i < *loot_count {
                    loot_index += 1;
                    loot.get(loot_index)
                        .expect("Item index exceeded bounds of loot in field_wtr().")
                } else {
                    ""
                }; // Write nothing i.e. for it_{2..=5}_{3,4}

                wtr.write_field(item)?;
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    };

    Ok(())
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
    treasurespheres: &Vec<Treasuresphere>,
    mut seed: &mut ChaCha8Rng,
    player_count: &usize,
) -> Result<Vec<&'static str>, Error> {
    // Almost sure this will break if you try to put a non 6-value
    let loot_counts: Vec<usize> = loot::player_loot::loot_counts(*player_count)?;
    let loot_sum = loot::player_loot::loot_sum(*player_count)?;

    // Moved externally to avoid rerolling this every ts or it
    // Should always reset the counter 'p', otherwise it will be biased towards previous flavoured ts-incompatible items
    //
    // Not sure if it's faster to check if items are unique or to remove items from item pool vector
    // (i.e. moving items from the back one item forward every single time item is drawn)
    let mut itempool: Vec<usize> = (0..200).collect();
    itempool.shuffle(&mut seed);

    let mut items_found: Vec<usize> = Vec::with_capacity(loot_sum);
    for t in 0..*TS_COUNT {
        'roll_next_item: for loot_count in &loot_counts {
            //i.e. for 1p, this goes 5,5,3,3,3,3
            let mut p: usize = 0; // Count through item indices in Pool of total itempool
            let treasuresphere = treasurespheres
                .get(t)
                .expect("Invalid treasuresphere indexed.");

            'find_valid_item: loop {
                let item: &usize = match itempool.get(p) {
                    Some(val) => val,
                    None => panic!(
                        "p indexed itempool invalid value in generate_it().\nIndex value:{}",
                        p
                    ),
                };

                if treasuresphere.in_ts_pool(item) {
                    //checks if in current color ts
                    for f in &items_found {
                        //checks if already found
                        // Checks if already prsent or not duplicate
                        if item != f {
                            continue;
                        } else {
                            p += 1;
                            continue 'find_valid_item;
                        }
                        // Add item to our items found on unique after finishing loop
                    }

                    p += 1;
                    items_found.push(
                        *itempool
                            .get(p)
                            .expect("Items found push in generate_it failed"),
                    );
                }
                continue 'roll_next_item;
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
            Treasuresphere::Normal if *loot < IT_COUNT.clone() as usize => return true,
            Treasuresphere::Normal => panic!("Loot index is out of bounds: {}", loot),
            Treasuresphere::Opal => loot::treasuresphere::IS_OPAL.get(loot).clone(),
            Treasuresphere::Sapphire => loot::treasuresphere::IS_SAPPHIRE.get(loot).clone(),
            Treasuresphere::Ruby => loot::treasuresphere::IS_RUBY.get(loot).clone(),
            Treasuresphere::Garnet => loot::treasuresphere::IS_GARNET.get(loot).clone(),
            Treasuresphere::Emerald => loot::treasuresphere::IS_EMERALD.get(loot).clone(),
        };

        match option {
            Some(val) => return *val,
            None => panic!(
                "Loot not found at index: {}\nTreasuresphere:{}\nfunction: in_ts_pool()",
                loot,
                self.to_string()
            ),
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
