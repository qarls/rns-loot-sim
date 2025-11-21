mod loot; //phf hashmaps and Vanilla game constants
mod writer; //writing to wtr functions
use anyhow::{Error, Result};
use clap::Parser;
use csv::Writer;
use loot::treasuresphere::Colors as Treasuresphere; // The treasuresphere types, i.e normal{1,2,3}, ruby, garnet
use loot::{IT_COUNT, TS_COUNT}; // vanilla constants for item count and ts count in 1.4.5
use rand::{self, seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng; // Useful for deterministic RNG
use std::fs::File;
use std::io::Write;

/// Program that simulates a number of games in Rabbit & Steel and writes items found
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of game runs (samples)
    #[arg(short = 'n', long, default_value_t = 1, value_parser(clap::value_parser!(u64).range(1..=200000)))]
    //200k
    run_count: u64,

    /// Player count
    #[arg(short, long, default_value_t = 1, value_parser(clap::value_parser!(u64).range(1..=4)))]
    player_count: u64,

    /// Output file (csv), if not used, print to stdout
    #[arg(short, long)]
    output_file: Option<String>,

    /// Use a positive interger (u64) seed for RNG (non-compliant)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Use indices instead of item names (it_[NAME])
    #[arg(short, long, action)]
    indices_for_items: Option<u64>,
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
    let game_count = args.run_count as u32;
    let player_count = args.player_count as u32;
    let mut seed = match args.seed {
        Some(val) => ChaCha8Rng::seed_from_u64(val),
        None => ChaCha8Rng::from_os_rng(),
    };

    let mut wtr = Writer::from_writer(vec![]);
    writer::field_wtr_headers(&mut wtr, &false, &player_count)?;

    for _ in 0..game_count {
        let ts: Vec<Treasuresphere> = generate_ts(&mut seed);
        let it: Vec<u32> = generate_it(&ts, &mut seed, &player_count)?;
        writer::field_wtr(&mut wtr, &ts, &it, &false, &player_count)?;
    }

    if let Some(file) = args.output_file {
        let mut file = File::create(file)?;
        file.write_all(&wtr.into_inner()?)?;
    } else {
        println!("{}", String::from_utf8(wtr.into_inner()?)?);
    }

    Ok(())
}

/// Generates a set of 6 random treasurespheres per game
///
/// # Examples
///
/// ```
/// use loot::treasuresphere::Colors;
///
/// let mut rng = ChaCha8Rng::seed_from_u64(20251121);
/// let ts = generate_ts(&mut rng);
/// assert_eq!(ts.len(), 6)
/// assert_eq!(ts.get(0) == Some<Colors>);
/// ```
pub fn generate_ts(mut seed: &mut ChaCha8Rng) -> Vec<Treasuresphere> {
    let count = *TS_COUNT;
    let mut ts = Vec::with_capacity(count);

    let mut nums: Vec<u8> = (0..8).collect();
    nums.shuffle(&mut seed);
    for i in 0..count {
        ts.push(Treasuresphere::from_index(nums[i]));
    }

    return ts;
}

/// Generates a set of random items per game
///
/// The Result-Vec returned are string values of item names
/// and are deemed "relative", i.e.:
/// - in 1P, items 4_2 [18] and 5_0 [19] will sit next to each other, where items 4_{3,4} are not evaulated
/// - in 4p, items 4_4 [24] and 5_0 [25] next to each other
#[allow(unused_variables)]
fn generate_it(
    ts: &Vec<Treasuresphere>,
    mut seed: &mut ChaCha8Rng,
    player_count: &u32,
) -> Result<Vec<u32>, Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count as usize)?; // n loot to roll every ts
    let loot_sum = loot::player_loot::loot_sum(*player_count as usize)?; // sum of loot rolled in game

    let mut items_found: Vec<u32> = Vec::with_capacity(loot_sum); //collection of loot in game
    let mut itempool: Vec<u32> = (0..200).collect();
    itempool.shuffle(&mut seed);

    for t in 0..*TS_COUNT {
        let ts_t = ts.get(t).expect("Invalid treasuresphere indexed.");
        let loot_count = loot_counts
            .get(t)
            .expect("ts indexed loot_counts out of bounds in generate_it()");
        let mut p: usize = 0; // Count through item indices in "Pool" of total itempool

        // [QoL] orders items per ts by their index by buffering it
        let mut items_found_t: Vec<u32> = Vec::with_capacity(*loot::IT_FOUND_MAX_PER_TS);

        // Must reshuffle item order every different ts color
        if t > 0 {
            match ts.get(t - 1) {
                Some(val) if val == ts_t => (), // No need to shuffle on consecutive normal ts
                None | Some(_) => itempool.shuffle(&mut seed),
            }
        }

        'roll_next_item: for _ in 0..*loot_count {
            'find_valid_item: while p < *IT_COUNT {
                let item: &u32 = itempool.get(p).expect("Failed index on item in pool.");

                if ts_t.is_item_in_ts(item)
                    && !items_found.contains(item)
                    && loot::treasuresphere::is_item_in_ts_pos(item, &t, TS_COUNT)
                {
                    items_found_t.push(
                        *itempool
                            .get(p)
                            .expect("items_found_t.push() failed in generate_it()"),
                    );
                    p += 1;
                    // println!("{items_found:?}");
                    continue 'roll_next_item; // advances to next item
                } else {
                    p += 1;
                    continue 'find_valid_item;
                }
            }
        }
        items_found_t.sort_unstable();
        items_found.append(&mut items_found_t);
    }

    return Ok(items_found);

    // let items_found_str: Vec<&'static str> = items_found
    //     .iter()
    //     .map(|x| {
    //         *loot::treasuresphere::ITEM_NAMES
    //             .get(x)
    //             .expect("Index of item index to names in generate_it() misindexed")
    //     })
    //     .collect::<Vec<&str>>();

    // assert_eq!(loot_sum, items_found_str.len()); // Simply checks if they're same length, should not fail

    // return Ok(items_found_str);
}
