use super::tables;
use crate::error::RnsError;
use std::fmt;

/// # The Treasuresphere variants
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Colors {
    /// - White
    /// - Up to 3 in each game
    Normal, // Reminder that you can find Normal 3 times
    /// - Purple/violet
    Opal,
    /// - Blue
    Sapphire,
    /// - Red
    Ruby,
    /// - Yellow
    Garnet,
    /// - Green
    Emerald,
}

/// Checks if the item is valid in the current Treasuresphere position
pub fn is_item_in_ts_pos(item: &usize, ts_i: &usize, ts_count: &usize) -> bool {
    let delta = ts_count - ts_i; // 1..=6
    match tables::NOT_IN_LAST_TS.get(&(*item as u32)) {
        //if 2 (topaz charm), then as long as delta exceeds, it returns true (it is present)
        Some(val) if &delta <= val => false,
        Some(_) => true,
        None => true,
    }
}

impl Colors {
    /// Returns the contents of the Treasuresphere variant
    pub fn items_in_ts(&self) -> Vec<usize> {
        //This needs to be modifiable
        match &self {
            Colors::Normal => (0..*super::IT_COUNT).collect(),
            Colors::Opal => tables::ITEM_OPAL.iter().copied().collect(),
            Colors::Sapphire => tables::ITEM_SAPPHIRE.iter().copied().collect(),
            Colors::Ruby => tables::ITEM_RUBY.iter().copied().collect(),
            Colors::Garnet => tables::ITEM_GARNET.iter().copied().collect(),
            Colors::Emerald => tables::ITEM_EMERALD.iter().copied().collect(),
        }
    }
}

/// For indexing a Color enum variant
impl TryFrom<usize> for Colors {
    type Error = RnsError;

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Colors::Normal),
            1 => Ok(Colors::Opal),
            2 => Ok(Colors::Sapphire),
            3 => Ok(Colors::Ruby),
            4 => Ok(Colors::Garnet),
            5 => Ok(Colors::Emerald),
            _ => Err(RnsError::InvalidTreasuresphereIndex(val)),
        }
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Colors::Normal => "normal",
            Colors::Opal => "opal",
            Colors::Sapphire => "sapphire",
            Colors::Ruby => "ruby",
            Colors::Garnet => "garnet",
            Colors::Emerald => "emerald",
        };
        write!(f, "{}", color)
    }
}
