mod loot; //phf hashmaps
use anyhow::{Context, Result, Error};
use clap::Parser;
use csv::Writer;
use rand::{SeedableRng, RngCore};
use rand_chacha::{ChaCha8Rng}; // Useful for deterministic RNG

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

    
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let game_count = args.run_count;
    let player_count = args.player_count;
    let mut seed = match args.seed {
        Some(val) => ChaCha8Rng::seed_from_u64(val),
        None => ChaCha8Rng::from_seed(Default::default()),
    };

    let mut wtr = Writer::from_writer(vec![]);
    for _ in 0..game_count {}
    Ok(())
}

fn field_wtr(mut wtr: Writer<Vec<u8>>, player_count: &u8) -> Result<(), Error> {
    wtr.write_field(todo!())?;
    todo!();
}

/// Generates a set of random treasurespheres per game
// I highly suspect this will work inconsistently seeded with multithreaded
fn generate_ts(seed: ChaCha8Rng) -> Vector<Treasuresphere> {
    let count = *TS_COUNT;
    let ts = Vec::with_capacity(count);

    let mut nums: Vec<u8> = (0..8).collect(); 
    seed.shuffle(&mut nums);
    for i in 0..count {
        ts.push(Treasuresphere::from_index(nums[i]));
    } //Omitting the last two numbers in array

    return ts;
}

/// Generates a set of random items per game
fn generate_it(treasurespheres: Vec<Treasuresphere>, seed: ChaCha8Rng, player_count: u8) -> Result<Vec<&'static str>, Error> {
    // Almost sure this will break if you try to put a non 6-value
    let loot_counts: [u8; *TS_COUNT] = match player_count { 
        1 => *loot_player_count::ONE,
        2 => *loot_player_count::TWO,
        3 => *loot_player_count::THREE,
        4 => *loot_player_count::FOUR,
        _ => bail!("Invalid player count: {}\nPlease enter a number from 1 to 4.", player_count),
    };

    // Reminder to self:
    // Moved externally to avoid rerolling this every ts or it
    // As long as you reset the counter 'p', otherwise it will be biased towards previous flavoured ts-incompatible items
    //
    // Not sure if it's faster to check if items are unique or to remove items from item pool vector
    // (i.e. moving items from the back one item forward every single time item is drawn)
    let mut itempool: Vec<u8> = (0..200).collect();
    seed.shuffle(&mut itempool);

    let items_found = Vec::with_capacity(loot_counts.sum());
    for t in 0..*TS_COUNT {
        for loot_count in loot_counts[t] {
            let mut p: u8 = 0; // Count through item indices in Pool of total itempool
            let treasuresphere = treasurespheres.get(t).expect("Invalid treasuresphere indexed.");
            
            'find_valid_item: loop {
                if treasuresphere.in_ts_pool(&p) { // Checks if item can be found in current ts, otherwise reset and advance counter
                    'is_unique: for f in items_found {
                        if nums(p) != f { // Checks if unique (not duplicate)
                            items_found.push(itempool.get[p].unwrap()); // Add item to our items found
                            p += 1;
                        }
                    }
                } else {
                    p += 1;
                    continue 'find_valid_item;
                }
            }
        }
    }

    let items_found_str = items_found.iter().map(|x| loot::ITEM_NAMES.get(x)?).collect::Vec<&str>();
    return items_found_str;
}

/// Treasuresphere Count
///
/// May break if changed from 6 as of right now.
static TS_COUNT: &'static usize = &6usize;

/// Item Count
static IT_COUNT: &'static usize = &200usize;

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
            _ => panic!("Unexpected treasuresphere index: {}", index)
        }
    }

    fn in_ts_pool(&self, loot: &u8) -> bool{
        let option = match &self {
            Self::Normal if *loot < IT_COUNT.clone() as u8 => return true,
            Self::Normal => panic!("Loot index is out of bounds: {}", loot),
            Self::Opal => loot::IS_OPAL.get(loot),
            Self::Sapphire => loot::IS_SAPPHIRE.get(loot),
            Self::Ruby => loot::IS_RUBY.get(loot),
            Self::Garnet => loot::IS_GARNET.get(loot),
            Self::Emerald => loot::IS_EMERALD.get(loot),
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
        }.to_string()
    }
}

/// Module to call constants based on player count
mod loot_player_count {
    pub static ONE: &'static [u8; 6] = &[5, 5, 3, 3, 3, 3];
    pub static TWO: &'static [u8; 6] = &[5, 5, 4, 4, 4, 4];
    pub static THREE: &'static [u8; 6] = &[5, 5, 4, 4, 4, 4];
    pub static FOUR: &'static [u8; 6] = &[5, 5, 5, 5, 5, 5];
}
