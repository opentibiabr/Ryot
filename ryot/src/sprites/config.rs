/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

use crate::Rect;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(unused)]
pub struct SpriteSheetConfig {
    pub tile_size: Rect,
    pub sheet_size: Rect,
    #[serde(default)]
    pub compression_config: Option<CompressionConfig>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct CompressionConfig {
    pub compressed_header_size: usize,
    pub content_header_size: usize,
}

pub fn cip_sheet() -> SpriteSheetConfig {
    SpriteSheetConfig {
        tile_size: Rect::new(32, 32),
        sheet_size: Rect::new(384, 384),
        compression_config: Some(CompressionConfig {
            compressed_header_size: 32,
            content_header_size: 122,
        }),
    }
}
