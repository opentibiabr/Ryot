use crate::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorEvents>().add_systems(
            Update,
            listen_cursor_events.run_if(on_event::<CursorEvents>()),
        );
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub enum CursorEvents {
    BrushChanged(usize),
    ToolModeChanged(ToolMode),
    InputTypeChanged(InputType),
    SizeChanged(i32),
}

fn listen_cursor_events(
    mut cursor_query: Query<&mut Cursor>,
    mut cursor_events: EventReader<CursorEvents>,
) {
    let mut cursor = cursor_query.single_mut();

    for event in cursor_events.read() {
        match event {
            CursorEvents::BrushChanged(id) => {
                cursor.drawing_state.brush_index = *id;
            }
            CursorEvents::ToolModeChanged(mode) => {
                cursor.drawing_state.tool_mode = *mode;
            }
            CursorEvents::InputTypeChanged(input) => {
                cursor.drawing_state.input_type = *input;
            }
            CursorEvents::SizeChanged(size) => {
                if let InputType::SingleClick(old_size) = &mut cursor.drawing_state.input_type {
                    *old_size = *size;
                }
            }
        }
    }
}
