//! Player-based functions and constants
use crate::error::RnsError;

/// Retrieve the loot counts for for a game based on player count
pub fn loot_counts(player_count: usize) -> Result<[usize; 6], RnsError> {
    let loot_counts = match player_count {
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        _ => return Err(RnsError::InvalidPlayerCount(player_count)),
    };
    Ok(loot_counts)
}

/// Retrieve the loot counts for for a game based on player count
pub fn loot_sum(player_count: &usize) -> Result<&usize, RnsError> {
    let loot_sum = match player_count {
        1 => &SUM_ONE,
        2 => &SUM_TWO,
        3 => &SUM_THREE,
        4 => &SUM_FOUR,
        _ => return Err(RnsError::InvalidPlayerCount(*player_count)),
    };
    Ok(loot_sum)
}

const ONE: [usize; 6] = [5, 5, 3, 3, 3, 3];
const TWO: [usize; 6] = [5, 5, 4, 4, 4, 4];
const THREE: [usize; 6] = [5, 5, 4, 4, 4, 4];
const FOUR: [usize; 6] = [5, 5, 5, 5, 5, 5];

const SUM_ONE: usize = 22;
const SUM_TWO: usize = 26;
const SUM_THREE: usize = 26;
const SUM_FOUR: usize = 30;
