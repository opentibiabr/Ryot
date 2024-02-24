use bevy::prelude::{Deref, DerefMut, Event};
use std::path::PathBuf;

mod assets;
pub use assets::*;

mod camera;
pub use camera::*;

mod gui;
pub use gui::*;

mod palette;
pub use palette::*;

mod drawing;
pub use drawing::*;

mod hud;
pub use hud::*;

#[derive(Event, Debug, Clone, Default, Deref, DerefMut)]
pub struct MapExport(pub PathBuf);
