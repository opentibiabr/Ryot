//! Shows how to do the bare minimum to execute ray casting using ryot

use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use ryot_core::game::Point;
use ryot_core::prelude::Flags;
use ryot_ray_casting::prelude::{InterestPositions, RadialArea, TrajectoryApp, VisibleTrajectory};
use ryot_tiled::prelude::TilePosition;
use ryot_utils::cache::Cache;

fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, (basic_setup, spawn_obstacle()))
        // .add_systems(First, draw_grid::<TilePosition>)
        .add_systems(Update, process_interest)
        .add_trajectory::<VisibleTrajectory<TilePosition>, Flags>()
        .add_plugins(LogPlugin::default())
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ));

    app.run();
}

pub fn basic_setup(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());

    for i in 0..1_000_000 {
        commands.spawn(VisibleTrajectory::<TilePosition>::new(
            RadialArea::default()
                .with_range(15)
                .with_angle_range((0, 1)),
            // RadialArea::circle().with_range(5),
        ));
    }
}

pub fn spawn_obstacle() -> impl FnMut(Commands, ResMut<Cache<TilePosition, Flags>>) {
    move |mut commands: Commands, cache: ResMut<Cache<TilePosition, Flags>>| {
        let Ok(mut write_guard) = cache.write() else {
            return;
        };

        write_guard.insert(
            TilePosition::new(1, 1, 0),
            Flags::default().with_blocks_sight(true),
        );
    }
}

fn draw_grid<P: Point + Into<Vec2>>(mut gizmos: Gizmos) {
    for x in -10..=10 {
        for y in -10..=10 {
            gizmos.rect_2d(
                P::generate(x, y, 0).into(),
                0.,
                tile_size().as_vec2(),
                Color::WHITE,
            );
        }
    }
}

fn process_interest(
    // mut gizmos: Gizmos,
    player_query: Query<&InterestPositions<VisibleTrajectory<TilePosition>>>,
) {
    for interest_positions in &player_query {
        for pos in interest_positions.positions.iter() {
            // gizmos.circle_2d(pos.into(), (tile_size().x / 2) as f32, Color::BLUE);
        }
    }
}