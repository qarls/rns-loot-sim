//! Game constants, hashmaps and helper functions associated with them.
pub mod players;
pub mod tables;
pub mod treasuresphere;

/// # Treasuresphere Count per game
///
/// - 6 Treasurespheres are rolled per game.
pub static TS_GAME_COUNT: &usize = &6usize;

/// # Treasuresphere Count
///
/// - 8 Treasurespheres in the pool.
/// - 5 Coloured TS + 3 Normal TS.
pub static TS_COUNT: &usize = &8usize;

/// # Item Count
///
/// - 200 items to roll from.
///
/// Normal Treasurespheres will pull from this full pool.
/// This should match the length of [tables::ITEM_NAMES].
pub static IT_COUNT: &usize = &200usize;

/// # Max items found per Treasuresphere
///
/// - 5 items found per Treasuresphere.
/// This variable is player-agnostic, consider passing a
/// [usize] to [players::loot_counts] or [players::loot_sum].
pub static IT_TS_GAME_COUNT_MAX: &usize = &5usize;
