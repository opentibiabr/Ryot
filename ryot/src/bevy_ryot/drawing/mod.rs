use std::time::Duration;

use crate::appearances::{self, is_true};
use crate::bevy_ryot::{GameObjectId, InternalContentState};
use crate::directional::*;
use crate::layer::*;
use crate::position::{Sector, SpriteMovement, TilePosition};
use bevy::prelude::*;
use bevy::render::view::{check_visibility, VisibilitySystems, VisibleEntities};

mod brushes;
pub use brushes::*;

mod commands;
pub use commands::*;

mod systems;
pub use systems::*;

use super::sprites::FrameGroupComponent;

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Layer>()
            .add_systems(
                PostUpdate,
                apply_detail_level_to_visibility
                    .in_set(VisibilitySystems::CheckVisibility)
                    .after(check_visibility)
                    .run_if(in_state(InternalContentState::Ready)),
            )
            .add_systems(
                PostUpdate,
                (apply_update, apply_deletion)
                    .in_set(DrawingSystems::Apply)
                    .after(VisibilitySystems::VisibilityPropagate),
            )
            .add_systems(
                PostUpdate,
                (persist_update, persist_deletion)
                    .in_set(DrawingSystems::Persist)
                    .after(DrawingSystems::Apply),
            );
    }
}

#[derive(Component, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct TileComponent;

/// A bundle that represents an entity drawn to a location (Layer + TilePosition) in the map.
/// The DrawingBundle is used to create and update the entities that are drawn on the map.
/// It holds the layer, the tile position, the appearance descriptor and the visibility of the entity.
///
/// Visibility is mainly a sprite component, however, for performance reasons, we use the sprite
/// visibility to control the visibility of the tile content. This way, we can reduce the amount of
/// effort made to drawn the map, by not drawing the tiles that are not visible, while still keeping
/// them as entity.
#[derive(Bundle, Debug, Copy, Clone, Default, PartialEq)]
pub struct DrawingBundle {
    pub layer: Layer,
    pub tile_pos: TilePosition,
    pub object_id: GameObjectId,
    pub frame_group: FrameGroupComponent,
    pub visibility: Visibility,
    pub tile: TileComponent,
}

impl BrushItem for DrawingBundle {
    fn from_position(original: Self, tile_pos: TilePosition) -> Self {
        DrawingBundle {
            tile_pos,
            ..original
        }
    }

    fn get_position(&self) -> TilePosition {
        self.tile_pos
    }
}

impl DrawingBundle {
    pub fn new(
        layer: impl Into<Layer>,
        tile_pos: TilePosition,
        object_id: GameObjectId,
        frame_group: FrameGroupComponent,
    ) -> Self {
        Self {
            layer: layer.into(),
            tile_pos,
            object_id,
            frame_group,
            tile: TileComponent,
            visibility: Visibility::Visible,
        }
    }

    pub fn from_tile_position(tile_pos: TilePosition) -> Self {
        Self {
            tile_pos,
            ..Default::default()
        }
    }

    pub fn object(layer: impl Into<Layer>, tile_pos: TilePosition, id: u32) -> Self {
        Self::new(layer, tile_pos, GameObjectId::Object(id), default())
    }

    pub fn outfit(
        layer: impl Into<Layer>,
        tile_pos: TilePosition,
        id: u32,
        frame_group: impl Into<FrameGroupComponent>,
    ) -> Self {
        Self::new(
            layer,
            tile_pos,
            GameObjectId::Outfit(id),
            frame_group.into(),
        )
    }

    pub fn effect(layer: impl Into<Layer>, tile_pos: TilePosition, id: u32) -> Self {
        Self::new(layer, tile_pos, GameObjectId::Effect(id), default())
    }

    pub fn missile(layer: impl Into<Layer>, tile_pos: TilePosition, id: u32) -> Self {
        Self::new(layer, tile_pos, GameObjectId::Missile(id), default())
    }

    pub fn with_position(mut self, tile_pos: TilePosition) -> Self {
        self.tile_pos = tile_pos;
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn with_layer(mut self, layer: impl Into<Layer>) -> Self {
        self.layer = layer.into();
        self
    }
}

/// A bundle that represents a moving entity going from one position to another.
/// The MovementBundle is a special case of the DrawingBundle, where the entity is drawn
/// in the top layer and it has a start and end position along with a duration.
/// The moving is drawn from the start position to the end position over the duration.
/// The moving is removed from the map when the duration is over by default.
#[derive(Bundle, Debug, Clone)]
pub struct MovementBundle {
    pub layer: Layer,
    pub object_id: GameObjectId,
    pub frame_group: FrameGroupComponent,
    pub movement: SpriteMovement,
    pub direction: Directional,
}

impl MovementBundle {
    pub fn new(
        layer: Layer,
        object_id: GameObjectId,
        frame_group: FrameGroupComponent,
        start: Vec3,
        end: Vec3,
        duration: Duration,
    ) -> Self {
        Self {
            layer,
            object_id,
            frame_group,
            movement: SpriteMovement::new(start, end, duration).despawn_on_end(true),
            direction: Directional::Ordinal(OrdinalDirection::from(end - start)),
        }
    }

    pub fn object(
        start: Vec3,
        end: Vec3,
        layer: impl Into<Layer>,
        id: u32,
        duration: Duration,
    ) -> Self {
        Self::new(
            layer.into(),
            GameObjectId::Object(id),
            default(),
            start,
            end,
            duration,
        )
    }

    pub fn missile(
        layer: impl Into<Layer>,
        start: Vec3,
        end: Vec3,
        id: u32,
        duration: Duration,
    ) -> Self {
        Self::new(
            layer.into(),
            GameObjectId::Missile(id),
            default(),
            start,
            end,
            duration,
        )
    }

    pub fn effect(
        layer: impl Into<Layer>,
        start: Vec3,
        end: Vec3,
        id: u32,
        duration: Duration,
    ) -> Self {
        Self::new(
            layer.into(),
            GameObjectId::Effect(id),
            default(),
            start,
            end,
            duration,
        )
    }

    pub fn sticky(self) -> Self {
        Self {
            movement: self.movement.despawn_on_end(false),
            ..self
        }
    }
}

/// Drawing levels (we try to keep a maximum of 100k visible sprites per level):
///     - Max details: 1 floor, 1 top, 1 bottom, 1 ground and 10 contents - ~64x64
///     - Medium details: 1 floor: 1 top, 1 bottom, 1 ground and 5 content - ~112x112
///     - Minimal details: 1 floor: 1 top, 1 bottom, 1 ground and 1 content - ~160x160
///     - Ground+bottom: 1 floor, 1 bottom, 1 ground - 224x224
///     - Ground only - 320x320
///     - >320x320 - Not possible (maybe chunk view so that people can navigate through the map quicker in the future)
///     - Draw rules change per detail level
///
/// We load 2-3x the current view but we only set as visible the 1.1 * view.
/// As we move the camera, we set the new tiles as visible and the old ones as hidden and we deload/load the sector (as hidden)
/// As we zoom in and out, we change the detail level of the tiles and change visibility accordingly.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Default, Reflect, Component)]
pub enum DetailLevel {
    #[default]
    Max,
    Medium,
    Minimal,
    GroundBottom,
    GroundOnly,
    None,
}

impl DetailLevel {
    pub fn from_area(area: u32) -> Self {
        let size = (area as f32).sqrt() as i32;
        match size {
            0..=64 => Self::Max,
            65..=112 => Self::Medium,
            113..=160 => Self::Minimal,
            161..=224 => Self::GroundBottom,
            225..=320 => Self::GroundOnly,
            _ => Self::None,
        }
    }

    pub fn is_layer_visible(&self, layer: &Layer) -> bool {
        let visible_layers: Vec<Layer> = match self {
            Self::Max => vec![
                Layer::Ground,
                Layer::Edge,
                Layer::Bottom(BottomLayer::new(10, RelativeLayer::Object)),
                Layer::Top,
                Layer::Hud(Order::MAX),
            ],
            Self::Medium => vec![
                Layer::Ground,
                Layer::Edge,
                Layer::Bottom(BottomLayer::new(5, RelativeLayer::Object)),
                Layer::Top,
                Layer::Hud(Order::MAX),
            ],
            Self::Minimal => vec![
                Layer::Ground,
                Layer::Edge,
                Layer::Bottom(BottomLayer::new(3, RelativeLayer::Object)),
                Layer::Hud(Order::MAX),
            ],
            Self::GroundBottom => vec![
                Layer::Ground,
                Layer::Edge,
                Layer::Bottom(BottomLayer::new(1, RelativeLayer::Object)),
                Layer::Hud(Order::MAX),
            ],
            Self::GroundOnly => vec![Layer::Ground, Layer::Edge, Layer::Hud(Order::MAX)],
            Self::None => vec![],
        };

        for visible_layer in visible_layers {
            match visible_layer {
                Layer::Bottom(bottom) => match layer {
                    Layer::Bottom(layer) if layer.order <= bottom.order => return true,
                    _ => (),
                },
                _ if *layer == visible_layer => return true,
                _ => (),
            }
        }

        false
    }
}

impl From<Option<appearances::Flags>> for Layer {
    fn from(flags: Option<appearances::Flags>) -> Self {
        match flags {
            Some(flags) if is_true(flags.is_top) => Layer::Top,
            Some(flags) if flags.ground.is_some() => Layer::Ground,
            Some(flags) if is_true(flags.is_ground) => Layer::Ground,
            Some(flags) if is_true(flags.is_edge) => Layer::Edge,
            _ => Layer::Bottom(BottomLayer::new(0, RelativeLayer::Object)),
        }
    }
}

#[allow(clippy::type_complexity)]
fn apply_detail_level_to_visibility(
    mut q_visible_entities: Query<(&mut VisibleEntities, &Sector), With<Camera>>,
    mut q_all_entities: Query<
        (&mut ViewVisibility, Option<&Layer>),
        (Without<Deletion>, With<TileComponent>),
    >,
) {
    for (mut visible_entities, sector) in q_visible_entities.iter_mut() {
        let detail_level = DetailLevel::from_area(sector.area());

        let entities = visible_entities
            .entities
            .iter()
            .filter_map(|entity| {
                let Ok((mut view_visibility, layer)) = q_all_entities.get_mut(*entity) else {
                    // If no tile is found we cannot infer anything about the detail level, so we
                    // just keep the entity visible.
                    return Some(*entity);
                };

                if let Some(layer) = layer {
                    if !detail_level.is_layer_visible(layer) {
                        *view_visibility = ViewVisibility::HIDDEN;
                        return None;
                    }
                }

                view_visibility.get().then_some(*entity)
            })
            .collect::<Vec<_>>();

        visible_entities.entities = entities;
    }
}
