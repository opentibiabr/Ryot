use bevy_asset::{Asset, Assets, Handle};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::{Res, Resource};
use bevy_reflect::TypePath;
use bevy_utils::tracing::debug;
use ryot_content::prelude::SpriteSheetDataSet;

pub trait CatalogAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn catalog_content(&self) -> &Handle<Catalog>;
}

/// An asset that holds a collection of raw content configs.
#[derive(serde::Deserialize, TypePath, Asset)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<ryot_content::prelude::ContentType>,
}

pub fn prepare_sprite_sheets<C: CatalogAsset>(
    content_assets: Res<C>,
    mut contents: ResMut<Assets<Catalog>>,
    mut sprite_sheets: ResMut<SpriteSheetDataSet>,
) {
    debug!("Preparing sprite sheets");

    *sprite_sheets = contents
        .get(content_assets.catalog_content())
        .expect("No catalog loaded")
        .content
        .clone()
        .into();

    contents.remove(content_assets.catalog_content());

    debug!("Finished preparing sprite sheets");
}