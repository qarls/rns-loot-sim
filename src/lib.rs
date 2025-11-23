pub mod loot;
pub use anyhow::{Error, Result};
pub use csv::Writer; // for main and writer
pub use loot::treasuresphere::{self, Colors}; // The treasuresphere types, i.e normal{1,2,3}, ruby, garnet
use rand::{self, seq::SliceRandom};
pub use rand_chacha::ChaCha8Rng as Rng; // vanilla constants for item count and ts count in 1.4.5 // Useful for deterministic RNG

/// Generates a set of 6 random treasurespheres per game
///
/// # Examples
///
/// ```
/// use rns_loot_sim::Colors;
/// use rns_loot_sim::Rng;
/// use rand::SeedableRng;
///
/// # fn main() {
///     let mut rng = Rng::seed_from_u64(20251121);
///     let ts: Vec<Colors> = rns_loot_sim::generate_ts(&mut rng);
///     assert_eq!(ts.len(), 6);
///
///     let ts_counts_max = [3,1,1,1,1,1];
///     let mut ts_counts = [0; 6];
///     ts.into_iter().for_each(|x| match x {
///         Colors::Normal => ts_counts[0]+=1,
///         Colors::Opal => ts_counts[1]+=1,
///         Colors::Sapphire => ts_counts[2]+=1,
///         Colors::Ruby => ts_counts[3]+=1,
///         Colors::Garnet => ts_counts[4]+=1,
///         Colors::Emerald => ts_counts[5]+=1,
///     });
///
///     for (t, count) in ts_counts.iter().enumerate() {
///             let max_count = ts_counts_max.get(t).unwrap();
///             assert!(count <= max_count);
///     }
/// # }
/// ```
pub fn generate_ts(mut seed: &mut Rng) -> Vec<Colors> {
    let mut nums: Vec<usize> = (0..8).collect();
    nums = nums.partial_shuffle(&mut seed, *loot::TS_COUNT).0.to_vec();
    let ts: Vec<Colors> = nums.into_iter().map(|t| Colors::from_index(&t)).collect();
    ts
}

/// Generates a set of random items per game
///
/// The Result-Vec returned are string values of item names
/// and are deemed "relative", i.e.:
/// - in 1P, items 4_2 [18] and 5_0 [19] will sit next to each other, where items 4_{3,4} are not evaulated
/// - in 4p, items 4_4 [24] and 5_0 [25] next to each other
pub fn generate_it(
    ts: &[Colors],
    mut seed: &mut Rng,
    player_count: &usize,
) -> Result<Vec<usize>, Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count)?; // n loot to roll every ts
    let loot_sum = loot::player_loot::loot_sum(*player_count)?; // sum of loot rolled in game

    let mut items_found: Vec<usize> = Vec::with_capacity(loot_sum); //collection of loot in game

    for t in 0..*loot::TS_COUNT {
        let ts_t = ts.get(t).expect("Invalid treasuresphere indexed.");
        let loot_count = loot_counts
            .get(t)
            .expect("ts indexed loot_counts out of bounds in generate_it()");
        let mut p: usize = 0; // Count through item indices in "Pool" of total itempool

        // [QoL] orders items per ts by their index by buffering it
        let mut items_found_t: Vec<usize> = Vec::with_capacity(*loot::IT_FOUND_MAX_PER_TS);

        // Pull items in itempool and partially shuffle them
        let mut itempool = ts_t.items_in_ts();
        let itempool_slice = itempool
            .partial_shuffle(&mut seed, loot_count + items_found.len())
            .0;

        'roll_next_item: for _ in 0..*loot_count {
            'find_valid_item: while p < *loot::IT_COUNT {
                let item = itempool_slice
                    .get(p)
                    .expect("Failed index on item in pool.");
                if !items_found.contains(item)
                    && loot::treasuresphere::is_item_in_ts_pos(item, &t, loot::TS_COUNT)
                {
                    items_found_t.push(
                        *itempool_slice
                            .get(p)
                            .expect("items_found_t.push() failed in generate_it()"),
                    );
                    p += 1;
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

    Ok(items_found)
}

/// Writes the headers for our CSV file
//
// I've included an unused "relative" bool where for 1-3p
// Where in 1p, this excludes the headers  {2..=5}_{3,4}
pub fn field_wtr_headers(
    wtr: &mut Writer<Vec<u8>>,
    _relative: &bool,
    _player_count: &usize,
) -> Result<(), Error> {
    // Writes the ts_headers
    wtr.write_field("player_count")?;
    for t in 0..*loot::TS_COUNT {
        let ts_t: String = format!("ts_{}", t);
        wtr.write_field(ts_t)?;
    }

    // Writes the it_headers
    if *_relative {
        todo!("Relative flag not priority.");
        // for t in 0..*loot::TS_COUNT {
        //     let loot_counts = loot::player_loot::loot_counts(*_player_count as usize)?;
        //     let loot_count = loot_counts
        //         .get(t)
        //         .expect("ts indexing exceeded bounds of loot_counts in field_wtr_headers().");
        //     for i in 0..*loot_count {
        //         let it_t_i: String = format!("it_{}_{}", t, i);
        //         wtr.write_field(it_t_i)?;
        //     }
        // }
    } else {
        for t in 0..*loot::TS_COUNT {
            for i in 0..*loot::IT_FOUND_MAX_PER_TS {
                let it_t_i: String = format!("it_{}_{}", t, i);
                wtr.write_field(it_t_i)?;
            }
        }
    };
    wtr.write_record(None::<&[u8]>)?;

    Ok(())
}

pub fn field_wtr(
    wtr: &mut Writer<Vec<u8>>,
    treasurespheres: &[Colors],
    loot: &[usize],
    _relative: &bool,
    player_count: &usize,
) -> Result<(), Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count)?;

    if *_relative {
        todo!("Relative flag not priority.");
    } else {
        wtr.write_field(player_count.to_string())?;
        for t in 0..*loot::TS_COUNT {
            let ts = treasurespheres
                .get(t)
                .expect("ts index exceeded the bounds of rolled treasurespheres in field_wtr().");
            wtr.write_field(ts.to_string())?;
        }

        let mut loot_index = 0;
        for t in 0..*loot::TS_COUNT {
            for i in 0..*loot::IT_FOUND_MAX_PER_TS {
                let loot_count = loot_counts
                    .get(t)
                    .expect("ts indexing exceeded bounds of loot_counts in field_wtr().");

                let item = if i < *loot_count {
                    let it = *loot
                        .get(loot_index)
                        .expect("Item index exceeded bounds of loot in field_wtr().");
                    loot_index += 1;
                    treasuresphere::ITEM_NAMES
                        .index(it)
                        .expect("Item not found in index for ITEM_NAMES in field_wtr().")
                } else {
                    ""
                }; // Write nothing i.e. for it_{2..=5}_{3,4}

                wtr.write_field(item)?;
            }
        }
    };

    wtr.write_record(None::<&[u8]>)?;
    Ok(())
}
