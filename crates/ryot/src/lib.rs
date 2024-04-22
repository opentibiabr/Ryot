//! MMORPG library based on the concepts of open tibia.
//!
//! Ryot is an event-driven library that provides simple utilities for building OT based games.
//! It is designed to be used with the [Bevy](https://bevyengine.org/) game engine.
//! It is currently in early development and is not yet ready for use.
//!
//! Ryot is design to integrate with OpenTibia concepts, facilitating the creation
//! of games that intend to use Tibia-like contents/assets formats, as well as some
//! game mechanics.
//!
//! It also provides some utilities:
//! * [Bevy Helpers](crate::bevy_ryot) - A collection of helpers that can be used to send async events,
//!   load configurations, sprites and contents as BevyAssets.
//! * [Compression](crate::compression) - A compression utility that can be used to compress
//!   and decompress sprite sheets.
//! * [ContentBuilder](crate::build::content) - A builder that can be used to build
//!   content assets from the Tibia client content folder, decompressing sprite sheets and
//!   copying the necessary files to the assets folder.
//! * [Sprite Utilities](crate::sprites) - Functions that can be used to decompress, manipulate
//!   and load sprite sheets as game assets, taking into considerations Tibia-like sprite sheets
//!   structures.
//! * [Content Utilities](crate::content) - A collection of structs that can be used to manipulate
//!   contents, including configuring and loading them.
#![feature(fn_traits)]
#![feature(lazy_cell)]
#![feature(unboxed_closures)]
#![feature(let_chains)]

#[cfg(feature = "bevy")]
pub mod bevy_ryot;

pub mod sprites;

pub use sprites::*;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::bevy_ryot::*;
    pub use crate::position::*;
    pub use crate::sprites::*;
    pub use ryot_internal::prelude::*;
}
