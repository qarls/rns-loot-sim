//! This module implements flexible lengths of headers.
//!
//! Using both functions does expect each
//! to compute the same number of fields. Otherwise
//! csv::Writer will throw a fit.
use crate::error::RnsError;
use crate::loot::{self, treasuresphere};
pub use csv::Error; // For writer errors
pub use csv::Writer;

/// Writes the headers for our CSV file
pub fn field_wtr_headers(
    wtr: &mut Writer<Vec<u8>>,
    _relative: &bool,
    _player_count: &usize,
) -> Result<(), RnsError> {
    // Writes the ts_headers
    wtr.write_field("player_count")?;
    for t in 0..*loot::TS_GAME_COUNT {
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
        for t in 0..*loot::TS_GAME_COUNT {
            for i in 0..*loot::IT_TS_GAME_COUNT_MAX {
                let it_t_i: String = format!("it_{}_{}", t, i);
                wtr.write_field(it_t_i)?;
            }
        }
    };
    wtr.write_record(None::<&[u8]>)?;

    Ok(())
}

/// Writes the fields for our CSV file
pub fn field_wtr(
    wtr: &mut Writer<Vec<u8>>,
    treasurespheres: &[treasuresphere::Colors],
    loot: &[usize],
    _relative: &bool,
    player_count: &usize,
) -> Result<(), RnsError> {
    let loot_counts = loot::players::loot_counts(*player_count)?;

    if *_relative {
        todo!("Relative flag not priority.");
    } else {
        wtr.write_field(player_count.to_string())?;
        for t in 0..*loot::TS_GAME_COUNT {
            let ts = treasurespheres
                .get(t)
                .expect("ts index exceeded the bounds of rolled treasurespheres in field_wtr().");
            wtr.write_field(ts.to_string())?;
        }

        let mut loot_index = 0;
        for t in 0..*loot::TS_GAME_COUNT {
            for i in 0..*loot::IT_TS_GAME_COUNT_MAX {
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
