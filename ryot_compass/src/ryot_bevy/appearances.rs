use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use prost::Message;
use ryot::prelude::Appearances;
use thiserror::Error;

pub struct AppearanceAssetPlugin;

impl Plugin for AppearanceAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Appearance>()
            .register_asset_loader(AppearanceAssetLoader {});
    }
}

#[derive(Debug, TypeUuid, Asset, TypePath)]
#[uuid = "b34dd6e4-23de-4bd2-8375-ce64cc8ca9fd"]
pub struct Appearance(pub Appearances);

#[derive(Default)]
pub struct AppearanceAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AppearanceLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [protobuf decode Error](prost::DecodeError)
    #[error("Could not decode from protobuf: {0}")]
    DecodeError(#[from] prost::DecodeError),
}

impl AssetLoader for AppearanceAssetLoader {
    type Asset = Appearance;
    type Settings = ();
    type Error = AppearanceLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let appearances = Appearances::decode(&*bytes)?;

            Ok(Appearance(appearances))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}
