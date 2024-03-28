use self::elevation::{apply_elevation, Elevation};
use crate::prelude::*;
use bevy::prelude::*;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub mod elevation;

pub struct GamePlugin<C: AppearanceAssets>(PhantomData<C>);

impl<C: AppearanceAssets> GamePlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: AppearanceAssets> Default for GamePlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppearanceAssets> Plugin for GamePlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>()
            .add_systems(Update, apply_elevation::<C>);
    }
}

#[derive(
    Component,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Default,
    Reflect,
    PartialOrd,
    Ord,
)]
pub enum GameObjectId {
    #[default]
    None,
    Object(u32),
    Outfit(u32),
    Effect(u32),
    Missile(u32),
}

impl GameObjectId {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn as_group_and_id(&self) -> Option<(AppearanceGroup, u32)> {
        match self {
            GameObjectId::None => None,
            GameObjectId::Object(id) => Some((AppearanceGroup::Object, *id)),
            GameObjectId::Outfit(id) => Some((AppearanceGroup::Outfit, *id)),
            GameObjectId::Effect(id) => Some((AppearanceGroup::Effect, *id)),
            GameObjectId::Missile(id) => Some((AppearanceGroup::Missile, *id)),
        }
    }

    pub fn group(&self) -> Option<AppearanceGroup> {
        self.as_group_and_id().map(|(group, _)| group)
    }

    pub fn id(&self) -> Option<u32> {
        self.as_group_and_id().map(|(_, id)| id)
    }
}

impl fmt::Display for GameObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameObjectId::None => write!(f, "None"),
            GameObjectId::Object(id) => write!(f, "Object({})", id),
            GameObjectId::Outfit(id) => write!(f, "Outfit({})", id),
            GameObjectId::Effect(id) => write!(f, "Effect({})", id),
            GameObjectId::Missile(id) => write!(f, "Missile({})", id),
        }
    }
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct GameObjectBundle {
    pub object_id: GameObjectId,
    pub position: TilePosition,
    pub elevation: Elevation,
    pub layer: Layer,
}

impl GameObjectBundle {
    pub fn new(object_id: GameObjectId, position: TilePosition, layer: Layer) -> Self {
        Self {
            object_id,
            position,
            layer,
            elevation: default(),
        }
    }
}

#[derive(Event, Clone, Debug, Deref, DerefMut)]
pub struct LoadObjects(pub Vec<GameObjectBundle>);
