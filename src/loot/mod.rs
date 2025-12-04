//! Game constants, hashmaps and helper functions associated with them.
pub mod players;
pub mod treasuresphere;

/// Treasuresphere Count per game
pub static TS_GAME_COUNT: &usize = &6usize;

/// Treasuresphere Count
///
/// Should be equal to 5 (Coloured TS) and number of normal TS
pub static TS_COUNT: &usize = &8usize;

/// Item Count in game
pub static IT_COUNT: &usize = &200usize;

/// Max items found per Treasuresphere
pub static IT_TS_GAME_COUNT_MAX: &usize = &5usize;
