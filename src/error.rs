//! # Custom Error module
//!
//! See [RnsError] for more information.
use thiserror::Error;

/// # Library-Targeted Errors
///
/// These should typically not be seen when used as a binary and rather
/// used as part of a binary.
///
/// I.e. Clap crate, via value parser, should already target invalid values.
#[derive(Debug, Error)]
pub enum RnsError {
    /// # Invalid Player Count
    ///
    /// This should be a value within inclusive bounds from 1 to 4.
    #[error(
        "Invalid player count {0} ({min} <= expected <= {max})",
        min = 1,
        max = 4
    )]
    InvalidPlayerCount(usize),

    /// # Misindexed Treasuresphere
    ///
    /// In [crate::loot::treasuresphere::Colors::try_from] and [crate::generate_it].
    #[error(
        "Invalid treasuresphere index {0} ({min} <= expected <= {max})",
        min = 0,
        max = 5
    )]
    InvalidTreasuresphereIndex(usize),

    /// # Writer-based errors
    ///
    /// Includes unmatched number of fields
    /// between the use of field_wtr_headers() and field_wtr().
    #[error(transparent)]
    WriterError(#[from] crate::writer::Error),
}
