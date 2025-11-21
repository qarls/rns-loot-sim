// Module for writer functions
use crate::loot;
use anyhow::{Error, Result};
use csv::Writer;
use loot::treasuresphere::Colors as Treasuresphere;
use loot::TS_COUNT; // vanilla constants for item count and ts count in 1.4.5

/// Writes the headers for our CSV file
//
// I've included an unused "relative" bool where for 1-3p
// Where in 1p, this excludes the headers  {2..=5}_{3,4}
pub fn field_wtr_headers(
    wtr: &mut Writer<Vec<u8>>,
    _relative: &bool,
    _player_count: &u32,
) -> Result<(), Error> {
    // Writes the ts_headers
    wtr.write_field("player_count")?;
    for t in 0..*TS_COUNT {
        let ts_t: String = format!("ts_{}", t);
        wtr.write_field(ts_t)?;
    }

    // Writes the it_headers
    if *_relative {
        todo!("Relative flag not priority.");
        // for t in 0..*TS_COUNT {
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
        for t in 0..*TS_COUNT {
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
    treasurespheres: &Vec<Treasuresphere>,
    loot: &Vec<&str>,
    _relative: &bool,
    player_count: &u32,
) -> Result<(), Error> {
    let loot_counts = loot::player_loot::loot_counts(*player_count as usize)?;

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
                let loot_count = loot_counts
                    .get(t)
                    .expect("ts indexing exceeded bounds of loot_counts in field_wtr().");

                let item = if i < *loot_count {
                    let it = loot
                        .get(loot_index)
                        .expect("Item index exceeded bounds of loot in field_wtr().");
                    loot_index += 1;
                    it
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
