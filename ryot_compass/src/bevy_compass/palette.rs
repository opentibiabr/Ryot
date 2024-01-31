use crate::{draw_palette_window, get_egui_parameters_for_texture, PaletteState, TilesetCategory};
use bevy::asset::Assets;
use bevy::log::warn;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::EguiPlugin;
use itertools::Itertools;
use ryot::bevy_ryot::sprites::{load_sprites, LoadSpriteSheetTextureCommand};
use ryot::prelude::*;
use std::marker::PhantomData;

pub struct PalettePlugin<C: AppearancesAssets + SpriteAssets>(PhantomData<C>);

impl<C: AppearancesAssets + SpriteAssets> PalettePlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: AppearancesAssets + SpriteAssets> Default for PalettePlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppearancesAssets + SpriteAssets> Plugin for PalettePlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<Palette>()
            .init_resource::<PaletteState>()
            .add_systems(OnEnter(InternalContentState::Ready), setup_categories::<C>)
            .add_systems(
                Update,
                (
                    update_palette_category::<C>,
                    update_palette_items::<C>,
                    draw_palette_window,
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct Palette {
    tile_set: HashMap<TilesetCategory, Vec<u32>>,
}

impl Palette {
    pub fn is_empty(&self) -> bool {
        self.tile_set.is_empty()
    }

    pub fn add_to_category(&mut self, category: TilesetCategory, id: u32) {
        self.tile_set.entry(category).or_default().push(id);
    }

    pub fn get_categories(&self) -> Vec<&TilesetCategory> {
        let mut categories: Vec<_> = self.tile_set.keys().collect();
        categories.push(&TilesetCategory::Raw);
        categories.sort();
        categories
    }

    pub fn get_for_category(&self, category: &TilesetCategory) -> Vec<u32> {
        match category {
            TilesetCategory::Raw => {
                // get the merge of all arrays
                let mut merged = vec![];
                for (_, v) in self.tile_set.iter() {
                    merged.extend(v);
                }
                merged
            }
            _ => self.tile_set.get(category).unwrap().to_vec(),
        }
    }
}

fn setup_categories<C: AppearancesAssets + SpriteAssets>(
    content_assets: Res<C>,
    mut palettes: ResMut<Palette>,
) {
    let Some(objects) = content_assets
        .prepared_appearances()
        .get_group(AppearanceGroup::Object)
    else {
        warn!("Appearances were not properly prepared");
        return;
    };

    objects.iter().for_each(|(asset_id, object)| {
        palettes.add_to_category(object.into(), *asset_id);
    });
}

pub fn update_palette_category<C: AppearancesAssets + SpriteAssets>(
    palettes: Res<Palette>,
    content_assets: Res<C>,
    mut palette_state: ResMut<PaletteState>,
) {
    if palettes.is_empty() {
        warn!("Cannot set category: palette is still empty");
        return;
    }

    if !palette_state.category_sprites.is_empty() {
        return;
    }

    let sprite_ids: Vec<u32> = palettes
        .get_for_category(&palette_state.selected_category)
        .iter()
        .filter_map(|value| {
            Some(
                *content_assets
                    .prepared_appearances()
                    .get_for_group(AppearanceGroup::Object, *value)?
                    .frame_group
                    .first()?
                    .sprite_info
                    .clone()?
                    .sprite_id
                    .first()?,
            )
        })
        .unique()
        .collect();

    sprite_ids.iter().for_each(|id| {
        palette_state.category_sprites.push(*id);
    });

    palette_state.category_sprites.sort();
}

pub fn update_palette_items<C: AppearancesAssets + SpriteAssets>(
    palettes: Res<Palette>,
    content_assets: Res<C>,
    mut palette_state: ResMut<PaletteState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut build_spr_sheet_texture_cmd: EventWriter<LoadSpriteSheetTextureCommand>,
) {
    if palettes.is_empty() {
        return;
    }

    let begin = palette_state
        .begin()
        .min(palette_state.category_sprites.len() - 5);

    let end = palette_state
        .end()
        .min(palette_state.category_sprites.len());

    let sprite_ids = &palette_state.category_sprites[begin..end].to_vec();

    if palette_state
        .loaded_images
        .iter()
        .map(|(sprite, ..)| sprite.sprite_id)
        .collect::<Vec<_>>()
        .eq(sprite_ids)
    {
        debug!("Palette content didn't change, no need to reload images");
        return;
    }

    palette_state.loaded_images.clear();

    for sprite in load_sprites(
        sprite_ids,
        &content_assets,
        &mut build_spr_sheet_texture_cmd,
    ) {
        let Some(atlas) = texture_atlases.get(sprite.atlas_texture_handle.clone()) else {
            continue;
        };

        let Some((rect_vec2, uv)) = get_egui_parameters_for_texture(&sprite, atlas) else {
            continue;
        };

        palette_state
            .loaded_images
            .push((sprite, atlas.texture.clone_weak(), rect_vec2, uv));
    }
}
