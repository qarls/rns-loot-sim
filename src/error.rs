use anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RnsError {
    #[error(
        "Invalid player count {0} ({min} <= expected <= {max})",
        min = 1,
        max = 4
    )]
    InvalidPlayerCount(usize),

    #[error(
        "Invalid treasuresphere index {0} ({min} <= expected <= {max})",
        min = 0,
        max = 5
    )]
    InvalidTreasuresphereIndex(usize),

    // Classifying this as an error for when we add
    // modifiable Treasuresphere count in game
    #[error(
        "Invalid index to a Treasuresphere-based variable {0} ({min} <= expected <= {max} (Treasuresphere count))",
        min = 0,
        max = 5
    )]
    InvalidTreasuresphereCountIndex(usize),

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
