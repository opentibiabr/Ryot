use crate::{Cursor, DrawingAction};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};

pub fn erase_on_hold() -> SystemConfigs {
    on_hold(
        delete_tile_content::<Changed<TilePosition>>,
        DrawingAction::Erase,
    )
}

pub fn erase_on_click() -> SystemConfigs {
    on_press(delete_tile_content::<()>, DrawingAction::Erase)
}

/// A function that listens to the right mouse button and deletes the content of the tile under the cursor.
/// It always delete the topmost content of the tile, following the Z-ordering.
fn delete_tile_content<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut command_history: ResMut<CommandHistory>,
    tiles: ResMut<MapTiles>,
    brushes: Res<Brushes<DrawingBundle>>,
    cursor_query: Query<(&Cursor, &TilePosition), F>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
) {
    for (cursor, tile_pos) in &cursor_query {
        info!("Deleting content at {:?}", tile_pos);
        let positions: Vec<TilePosition> = brushes(
            cursor.drawing_state.brush_index,
            cursor.drawing_state.brush_size,
            DrawingBundle::from_tile_position(*tile_pos),
        )
        .into_iter()
        .map(|bundle| bundle.tile_pos)
        .collect();

        let mut queued = 0;

        for tile_pos in positions {
            let Some(tile_content) = tiles.get(&tile_pos) else {
                continue;
            };

            let mut content: Option<(Entity, Layer, AppearanceDescriptor)> = None;

            for layer in [Layer::Top, Layer::Items, Layer::Bottom, Layer::Ground] {
                if let Some(entity) = tile_content.get(&layer) {
                    if let Ok((current, visibility)) = current_appearance_query.get(*entity) {
                        if visibility == Visibility::Hidden {
                            continue;
                        }

                        content = Some((*entity, layer, *current));
                        break;
                    }
                }
            }

            let Some((entity, layer, appearance)) = content else {
                continue;
            };

            let command =
                UpdateTileContent(None, Some(DrawingBundle::new(layer, tile_pos, appearance)));

            commands.add(command.with_entity(entity));
            command_history
                .performed_commands
                .push(TileCommandRecord::new(layer, tile_pos, Box::new(command)).into());

            queued += 1;
        }

        command_history
            .performed_commands
            .push(CommandBatchSize(queued).into());

        command_history.reversed_commands.clear();
    }
}
