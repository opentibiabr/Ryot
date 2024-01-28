use serde_repr::{Deserialize_repr, Serialize_repr};

mod config;
pub use config::*;

mod sheet_loading;
pub use sheet_loading::*;

pub mod tile_grid;

pub mod error;

#[derive(Serialize_repr, Deserialize_repr, Default, PartialEq, Debug, Clone)]
#[repr(u32)]
pub enum SpriteLayout {
    #[default]
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

impl SpriteLayout {
    pub fn get_width(&self, sheet_config: &SpriteSheetConfig) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => sheet_config.tile_size.x,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => sheet_config.tile_size.x * 2,
        }
    }

    pub fn get_height(&self, sheet_config: &SpriteSheetConfig) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => sheet_config.tile_size.y,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => sheet_config.tile_size.y * 2,
        }
    }
}

#[cfg(test)]
mod tests;
