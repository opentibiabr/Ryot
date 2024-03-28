//! This module contains the structs and enums that are used to load the graphical content
//! of a OT based game, with assets from the client's content folder.
//! The content folder is the folder that contains the content-catalog.json file and the
//! files that are listed in it.
//! The content-catalog.json file is a list of resource files that the client needs to load.
//! This module also wraps the protobuf representation of the appearances.dat file.
//! See appearances.proto and the auto generated appearances.rs file for more information.
include!(concat!(env!("OUT_DIR"), "/appearances.rs"));

use std::ops::Deref;

use crate::SpriteLayout;
use glam::UVec2;
use serde::{Deserialize, Serialize};

/// Those are the available contents within the content-catalog.json file
/// There are 5 known types of content: appearances, staticdata, staticmapdata, map and sprite.
/// Even though we deserialize all the 5 types, we only use the sprites and appearances in this
/// library.
///
/// Example:
/// ```json
/// [
///    {
///      "type": "appearances",
///      "file": "appearances.dat"
///    },
///    {
///      "type": "staticdata",
///      "file": "staticdata.dat"
///    },
///    {
///      "type": "staticmapdata",
///      "file": "staticmapdata.dat"
///    },
///    {
///      "type": "map",
///      "file": "map.otbm"
///    },
///    {
///       "type": "sprite",
///       "file": "spritesheet.png",
///       "spritetype": 0,
///       "firstspriteid": 100,
///       "lastspriteid": 200,
///       "area": 64
///     }
/// ]
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "appearances")]
    Appearances { file: String },
    #[serde(rename = "staticdata")]
    StaticData { file: String },
    #[serde(rename = "staticmapdata")]
    StaticMapData { file: String },
    #[serde(rename = "map")]
    Map { file: String },
    #[serde(rename = "sprite")]
    Sprite(SpriteSheetData),
}

/// This is the content of the Sprite ContentType. It contains the information needed
/// to load the sprite sheet and individual sprites from it.
/// A sprite sheet is defined by:
/// - a lzma compressed bmp file
/// - a sprite layout (1:1, 1:2, 2:1 or 2:2)
/// - the ids of first and last sprites in the sheet, to determine which sprites are in the sheet
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpriteSheetData {
    pub file: String,
    #[serde(rename = "spritetype")]
    pub layout: SpriteLayout,
    #[serde(rename = "firstspriteid")]
    pub first_sprite_id: u32,
    #[serde(rename = "lastspriteid")]
    pub last_sprite_id: u32,
    pub area: u32,
}

impl SpriteSheetData {
    /// Checks if the sprite sheet contains the given sprite id
    pub fn has_sprite(&self, sprite_id: u32) -> bool {
        self.first_sprite_id <= sprite_id && self.last_sprite_id >= sprite_id
    }

    /// Returns the index of a given sprite id in the sprite sheet.
    /// The index is the position of the sprite in the sprite sheet, starting from 0.
    /// The index is used to calculate the position of the sprite in the sprite sheet.
    /// If the sprite id is not in the sprite sheet, None is returned.
    pub fn get_sprite_index(&self, sprite_id: u32) -> Option<usize> {
        if self.has_sprite(sprite_id) {
            Some((sprite_id - self.first_sprite_id) as usize)
        } else {
            None
        }
    }

    /// Returns the size of a sprite in the sprite sheet.
    /// The size is calculated based on the sprite layout and the sprite sheet config.
    pub fn get_tile_size(&self, tile_size: &UVec2) -> UVec2 {
        let width = self.layout.get_width(tile_size);
        let height = self.layout.get_height(tile_size);
        UVec2::new(width, height)
    }
}

/// This is a collection of sprite sheets.
/// It contains the sprite sheets and the sprite sheet config.
/// The sprite sheet config is used to calculate the position and size of a sprite in the sprite
/// sheet.
#[derive(Debug, Default, Clone)]
pub struct SpriteSheetDataSet(Vec<SpriteSheetData>);

impl Deref for SpriteSheetDataSet {
    type Target = Vec<SpriteSheetData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SpriteSheetDataSet {
    /// Creates a new SpriteSheetSet from a list of ContentType.
    /// The ContentType list is filtered to only contain the Sprite ContentType.
    pub fn from_content(content: &[ContentType]) -> Self {
        let sprite_sheets = content
            .iter()
            .filter_map(|content_type| match content_type {
                ContentType::Sprite(sprite_sheet) => Some(sprite_sheet),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        Self(sprite_sheets)
    }

    /// Returns the sprite sheet that contains the given sprite id.
    /// Returns None if the sprite id is not in any of the sprite sheets.
    pub fn get_by_sprite_id(&self, sprite_id: u32) -> Option<&SpriteSheetData> {
        self.iter().find(|sheet| sheet.has_sprite(sprite_id))
    }
}

pub fn is_true(value: Option<bool>) -> bool {
    value == Some(true)
}

#[cfg(test)]
mod tests;
