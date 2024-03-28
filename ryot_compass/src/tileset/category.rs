use egui::WidgetText;
use ryot::appearances::{is_true, Flags, StoreCategory, VisualElement};
use ryot::prelude::*;
use std::cmp::Ordering;
use strum::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum TilesetCategory {
    Terrains,
    Edges,
    BaseLayer,
    UpperLayer,
    Decor,
    Corpses,
    Containers,
    Clothes,
    Consumables,
    Tools,
    Miscellaneous,
    Valuables,
    CreatureProducts,
    Weapons,
    Raw,
}

impl From<&Flags> for TilesetCategory {
    fn from(flags: &Flags) -> Self {
        // Market has categories, so we can use it to determine the category of the item.
        // If the item has a market flag, it's category is prioritized over the other flags.
        if let Some(market) = &flags.market_info {
            if let Some(category) = market.category {
                return (&StoreCategory::try_from(category).unwrap()).into();
            }
        }

        if flags.ground.is_some() || is_true(flags.is_ground) {
            return TilesetCategory::Terrains;
        }

        if is_true(flags.is_edge) {
            return TilesetCategory::Edges;
        }

        if is_true(flags.is_bottom) {
            return TilesetCategory::BaseLayer;
        }

        if is_true(flags.is_top) {
            return TilesetCategory::UpperLayer;
        }

        // Corpses are also containers, so they need to be checked first
        if is_true(flags.is_corpse) || is_true(flags.is_player_corpse) {
            return TilesetCategory::Corpses;
        }

        if is_true(flags.is_container) {
            return TilesetCategory::Containers;
        }

        if is_true(flags.can_be_hanged) || is_true(flags.can_rotate) || flags.hook_info.is_some() {
            return TilesetCategory::Decor;
        }

        if flags.slot.is_some() {
            return TilesetCategory::Clothes;
        }

        TilesetCategory::Miscellaneous
    }
}

impl From<&StoreCategory> for TilesetCategory {
    fn from(category: &StoreCategory) -> Self {
        match category {
            StoreCategory::Ammunition
            | StoreCategory::Axes
            | StoreCategory::Clubs
            | StoreCategory::DistanceWeapons
            | StoreCategory::Shields
            | StoreCategory::Quiver
            | StoreCategory::Swords
            | StoreCategory::WandsRods => TilesetCategory::Weapons,
            StoreCategory::Armors
            | StoreCategory::Amulets
            | StoreCategory::Boots
            | StoreCategory::HelmetsHats
            | StoreCategory::Legs
            | StoreCategory::Rings => TilesetCategory::Clothes,
            StoreCategory::CreatureProducts => TilesetCategory::CreatureProducts,
            StoreCategory::Containers => TilesetCategory::Containers,
            StoreCategory::Decoration => TilesetCategory::Decor,
            StoreCategory::Food | StoreCategory::Potions | StoreCategory::Runes => {
                TilesetCategory::Consumables
            }
            StoreCategory::PremiumScrolls
            | StoreCategory::TibiaCoins
            | StoreCategory::Valuables => TilesetCategory::Valuables,
            StoreCategory::Others => TilesetCategory::Miscellaneous,
            StoreCategory::Tools => TilesetCategory::Tools,
        }
    }
}

impl From<&VisualElement> for TilesetCategory {
    fn from(appearance: &VisualElement) -> Self {
        if let Some(flags) = &appearance.flags {
            return flags.into();
        }

        TilesetCategory::Miscellaneous
    }
}

impl From<&PreparedAppearance> for TilesetCategory {
    fn from(appearance: &PreparedAppearance) -> Self {
        if let Some(flags) = &appearance.flags {
            return flags.into();
        }

        TilesetCategory::Miscellaneous
    }
}

impl PartialOrd<Self> for TilesetCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TilesetCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_label().cmp(&other.get_label())
    }
}

impl From<&TilesetCategory> for WidgetText {
    fn from(val: &TilesetCategory) -> Self {
        WidgetText::from(val.get_label())
    }
}

impl TilesetCategory {
    pub fn get_label(&self) -> String {
        match self {
            TilesetCategory::Terrains => String::from("Terrains"),
            TilesetCategory::Edges => String::from("Edges"),
            TilesetCategory::BaseLayer => String::from("Base Layer"),
            TilesetCategory::UpperLayer => String::from("Upper Layer"),
            TilesetCategory::Decor => String::from("Decor"),
            TilesetCategory::Corpses => String::from("Corpses"),
            TilesetCategory::Containers => String::from("Containers"),
            TilesetCategory::Clothes => String::from("Clothes"),
            TilesetCategory::Consumables => String::from("Consumables"),
            TilesetCategory::Tools => String::from("Tools"),
            TilesetCategory::Miscellaneous => String::from("Miscellaneous"),
            TilesetCategory::Valuables => String::from("Valuables"),
            TilesetCategory::CreatureProducts => String::from("Creature Products"),
            TilesetCategory::Weapons => String::from("Weapons"),
            TilesetCategory::Raw => String::from("Raw"),
        }
    }
}
