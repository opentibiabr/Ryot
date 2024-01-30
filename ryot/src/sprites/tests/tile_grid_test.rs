use crate::tile_grid::TileGrid;
use glam::{UVec2, Vec2, Vec3};
use rstest::rstest;

#[rstest]
#[case(TileGrid::from_grid_size(0, 0), 0)]
#[case(TileGrid::from_grid_size(10, 0), 0)]
#[case(TileGrid::from_grid_size(0, 10), 0)]
#[case(TileGrid::from_grid_size(10, 10), 100)]
#[case(TileGrid::from_grid_size(100, 100), 10000)]
#[case(TileGrid::default(), u16::MAX as u32 * u16::MAX as u32)]
fn test_get_tile_count(#[case] tile_grid: TileGrid, #[case] expected: u32) {
    assert_eq!(tile_grid.get_tile_count(), expected);
}

#[rstest]
#[case(
    TileGrid::from_grid_size(100, 100),
    Vec2::new(0., 0.),
    Vec2::new(0., 0.)
)]
#[case(
    TileGrid::from_grid_size(100, 100),
    Vec2::new(32., 32.),
    Vec2::new(1., -1.),
)]
#[case(
    TileGrid::from_grid_size(100, 100),
    Vec2::new(64., -64.),
    Vec2::new(2., 2.),
)]
#[case(
    TileGrid::from_grid_size(2048, 1024),
    Vec2::new(32., -64.),
    Vec2::new(1., 2.),
)]
#[case(
    TileGrid::from_tile_size(UVec2::new(128, 256)),
    Vec2::new(950., -9123.),
    Vec2::new(7., 35.),
)]
fn test_get_tile_pos_from_display_pos_vec2(
    #[case] tile_grid: TileGrid,
    #[case] cursor_pos: Vec2,
    #[case] expected: Vec2,
) {
    assert_eq!(
        tile_grid.get_tile_pos_from_display_pos(cursor_pos),
        expected
    );
}

#[rstest]
#[case(TileGrid::from_grid_size(100, 100), Vec3::new(0., 0., 0.), None)]
#[case(TileGrid::from_grid_size(100, 100), Vec3::new(1., 1., 0.), Some(Vec3::new(32., -32., 3.0518044e-5)))]
#[case(
    TileGrid::from_grid_size(100, 100),
    Vec3::new(32., 33., 1.),
    Some(Vec3::new(1024., -1056., 1.0009918))
)]
#[case(
    TileGrid::from_grid_size(100, 100),
    Vec3::new(1., 100., 1.),
    Some(Vec3::new(32., -3200., 1.0015411))
)]
#[case(
    TileGrid::from_tile_size(UVec2::new(128, 256)),
    Vec3::new(100., 1., 1.),
    Some(Vec3::new(12800., -256., 1.0015411))
)]
fn test_get_display_position_from_tile_pos(
    #[case] tile_grid: TileGrid,
    #[case] tile_pos: Vec3,
    #[case] expected: Option<Vec3>,
) {
    assert_eq!(
        tile_grid.get_display_position_from_tile_pos(tile_pos),
        expected
    );
}
