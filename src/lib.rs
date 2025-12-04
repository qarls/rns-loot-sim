pub mod error;
pub mod loot;
pub mod writer;
use anyhow::Result;
use error::RnsError;
use loot::treasuresphere::Colors; // The treasuresphere types, i.e normal{1,2,3}, ruby, garnet
use rand::{self, seq::SliceRandom};
pub use rand_chacha::ChaCha8Rng as Rng;
use std::ops::ControlFlow;

/// # Treasuresphere Generate per Game
///
/// This should always roll 6 Treasurespheres of [Colors] variants.
///
/// ## Item weights
///
/// - 3: Normal
/// - 1: Opal, Sapphire, Ruby, Garnet, Emerald
///
/// ## Examples
///
/// This is test that checks no bounds were exceeded,
/// either by more [Colors] variants than in the pool
/// or the Vector returned from the function.
///
/// ```
/// use rns_loot_sim::loot::treasuresphere::Colors;
/// use rns_loot_sim::Rng;
/// use rand::SeedableRng;
///
/// # fn main() -> anyhow::Result<(), anyhow::Error> {
///     let mut rng = Rng::seed_from_u64(20251121);
///     let ts: Vec<Colors> = rns_loot_sim::generate_ts(&mut rng)?;
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
///     Ok(())
/// # }
/// ```
pub fn generate_ts(mut seed: &mut Rng) -> Result<Vec<Colors>, RnsError> {
    let ts_weights = [3, 1, 1, 1, 1]; //NOSRGE; for modding
    let mut ts_all: Vec<Colors> = Vec::new(); // contains colors and duplicates of normal

    // Push pool of colors into ts_all vector based on weights
    let iter_cf = ts_weights.iter().enumerate().try_for_each(|(i, w)| {
        for _ in 0..*w {
            let color = Colors::try_from(i);
            match color {
                Ok(val) => ts_all.push(val),
                Err(error) => return ControlFlow::Break(error),
            }
        }
        return ControlFlow::Continue(());
    });

    // Way to return error from for loop
    if let ControlFlow::Break(error) = iter_cf {
        return Err(error);
    }
    {
        let ts: Vec<Colors> = ts_all
            .partial_shuffle(&mut seed, *loot::TS_GAME_COUNT)
            .0
            .to_vec(); // Then we roll our colours
        Ok(ts)
    }
}

/// # Item Generate per Game
///
/// The Result-Vec returned are indices values ([usize]) of item names
/// and are deemed "relative", i.e.:
/// - in 1P, items 4_2 \[18\] and 5_0 \[19\] will sit next to each other, in vec it_{3,4} are not evaulated
/// - in 4p, items 4_4 \[24\] and 5_0 \[25\] next to each other
///
/// ## Notes
///
/// This function assumes you pass a &\[[Colors]\] of length 6.
///
/// There is a "QoL" feature that orders items rolled within each found TS.
pub fn generate_it(
    ts: &[Colors],
    mut seed: &mut Rng,
    player_count: &usize,
) -> Result<Vec<usize>, RnsError> {
    let loot_counts = loot::players::loot_counts(*player_count)?; // n loot to roll every ts
    let loot_sum = loot::players::loot_sum(&player_count)?; // sum of loot rolled in game

    let mut items_found: Vec<usize> = Vec::with_capacity(*loot_sum); //collection of loot in game
    for t in 0..*loot::TS_GAME_COUNT {
        if let Some(ts_t) = ts.get(t) {
            let loot_count = loot_counts
                .get(t)
                .expect("Invalid loot count index in generate_it()");
            let mut p: usize = 0; // Count through item indices in "Pool" of total itempool

            // [QoL] orders items per ts by their index by buffering it
            let mut items_found_t: Vec<usize> = Vec::with_capacity(*loot::IT_TS_GAME_COUNT_MAX);

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
                        && loot::treasuresphere::is_item_in_ts_pos(item, &t, loot::TS_GAME_COUNT)
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
        } else {
            return Err(RnsError::InvalidTreasuresphereIndex(t));
        }
    }

    Ok(items_found)
}
