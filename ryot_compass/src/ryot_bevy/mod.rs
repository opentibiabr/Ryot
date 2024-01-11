/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use bevy::prelude::*;
use ryot::cip_content::{
    decompress_sprite_sheet, get_decompressed_file_name, get_sprite_grid_by_id,
    get_sprite_index_by_id, ContentType, SheetGrid,
};
use std::path::PathBuf;

mod palette;
pub use palette::*;

#[derive(Resource, Debug)]
pub struct CipContent {
    pub raw_content: Vec<ContentType>,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandlers(pub bevy::utils::HashMap<String, Handle<TextureAtlas>>);

impl Default for CipContent {
    fn default() -> Self {
        Self {
            raw_content: vec![],
        }
    }
}

pub struct LoadedSprite {
    pub sprite_id: u32,
    pub sprite_index: usize,
    pub atlas_grid: SheetGrid,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

pub fn load_sprites(
    sprite_ids: &[u32],
    content: &[ContentType],
    asset_server: &Res<AssetServer>,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Vec<LoadedSprite> {
    let unsaved_sprites: Vec<(SheetGrid, TextureAtlas)> = sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            if let Ok(grid) = get_sprite_grid_by_id(content, *sprite_id) {
                Some((
                    grid.clone(),
                    build_texture_atlas_from_sheet(&grid, asset_server),
                ))
            } else {
                None
            }
        })
        .collect();

    unsaved_sprites.iter().for_each(|(grid, atlas)| {
        build_atlas_handler(&grid, atlas.clone(), atlas_handlers, texture_atlases);
    });

    sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            if let Ok(grid) = get_sprite_grid_by_id(content, *sprite_id) {
                Some(LoadedSprite {
                    sprite_id: *sprite_id,
                    sprite_index: get_sprite_index_by_id(content, *sprite_id).unwrap(),
                    atlas_grid: grid.clone(),
                    atlas_texture_handle: atlas_handlers.0.get(&grid.file).unwrap().clone(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn load_sprite(
    sprite_id: u32,
    content: &[ContentType],
    asset_server: &Res<AssetServer>,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(grid) = get_sprite_grid_by_id(content, sprite_id) {
        let atlas = build_texture_atlas_from_sheet(&grid, asset_server);
        build_atlas_handler(&grid, atlas, atlas_handlers, texture_atlases);
    }
}

pub fn get_sprite_by_id(
    sprite_id: u32,
    content: &[ContentType],
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
) -> Option<LoadedSprite> {
    if let Ok(grid) = get_sprite_grid_by_id(content, sprite_id) {
        Some(LoadedSprite {
            sprite_id,
            sprite_index: get_sprite_index_by_id(content, sprite_id).unwrap(),
            atlas_grid: grid.clone(),
            atlas_texture_handle: atlas_handlers.0.get(&grid.file).unwrap().clone(),
        })
    } else {
        None
    }
}

pub fn build_atlas_handler(
    grid: &SheetGrid,
    texture_atlas: TextureAtlas,
    mut atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    if !atlas_handlers.0.contains_key(&grid.file) {
        let atlas_handle = texture_atlases.add(texture_atlas);
        atlas_handlers.0.insert(grid.file.clone(), atlas_handle);
    }

    atlas_handlers.0.get(&grid.file).unwrap().clone()
}

pub fn build_texture_atlas_from_sheet(
    grid: &SheetGrid,
    asset_server: &Res<AssetServer>,
) -> TextureAtlas {
    let path = format!("sprite-sheets/{}", get_decompressed_file_name(&grid.file));

    std::fs::create_dir_all("assets/sprite-sheets").unwrap();

    if !PathBuf::from(format!("assets/{}", path)).exists() {
        decompress_sprite_sheet(&grid.file, "assets/cip_catalog", "assets/sprite-sheets");
    }

    let image_handle: Handle<Image> = asset_server.load(&path);
    TextureAtlas::from_grid(
        image_handle,
        Vec2::new(grid.tile_size.width as f32, grid.tile_size.height as f32),
        grid.columns,
        grid.rows,
        None,
        None,
    )
}

pub fn draw_sprite(pos: Vec3, sprite: &LoadedSprite, commands: &mut Commands) {
    let x = pos.x * 32.;
    let y = pos.y * 32.;
    let z = pos.z + (pos.x + pos.y) / u16::MAX as f32;

    commands.spawn(build_sprite_bundle(
        sprite.atlas_texture_handle.clone(),
        Vec3::new(x, -y, z),
        sprite.sprite_index,
    ));
}

pub fn build_sprite_bundle(
    handle: Handle<TextureAtlas>,
    translation: Vec3,
    index: usize,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        transform: Transform {
            translation,
            ..default()
        },
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: handle,
        ..default()
    }
}
