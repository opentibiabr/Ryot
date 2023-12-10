/*
"kv storage" MapName
header -> {
  width: u16,
  height: u16,
  floors: u8,
  version: u8,
  description: string,
}

x:y:z:tile -> {
    uid: int,
    tile_id: u16,
    flags: u32,
}

parent_id:item -> {
    uid: int
    item_id: u16,
    count: u8,
    charges: u8, // maybe could merge with count? same idea of stacking
    depot_id: u16,
    text: String,
}

parent_id:house -> {
    id: u32,
    door_id: u8,
}

item_id:action -> {
    action_id: u16,
    unique_id: u16,
    teleport: Position,
}

wp:name -> {
    name: String,
    position: Position,
}

town:name -> {
    id: u8,
    name: String,
    position: Position,
}

x:y:z:spawn -> {
    radius: u8,
    spawn_time: u16,
}

spawn_id:ncp:id -> {
    name: String,
    x: u8,
    y: u8,
    z: u8,
}

spawn_id:monster:id -> {
    name: String,
    x: u8,
    y: u8,
    z: u8,
}

zone:id -> {
    name: String,
}

house:id -> {
    id: u32,
    name: String,
    position: Position,
    rent: u32,
    guild_hall: bool,
    town_id: u8,
    size: u16,
    beds: u8,
}
 */
use rand::Rng;
use crate::compass::{Item, Plan, Component, Tile, Position, ItemAttribute};

pub fn build_map() -> Plan {
    let mut map = Plan::default();

    for x in 60000..61100 {
        for y in 60000..61100 {
            for z in 0..15 {
                let mut tile = Tile::new(Position::new(x, y, z as u8));
                let item1 = Item {
                    id: rand::thread_rng().gen_range(300..=305),
                    items: Vec::new(),
                    attributes: get_attribute_array(),
                };

                let item2 = Item {
                    id: rand::thread_rng().gen_range(300..=305),
                    items: vec![item1],
                    attributes: get_attribute_array(),
                };

                tile.set_item(Item{
                    id: rand::thread_rng().gen_range(300..=400),
                    // items: Vec::new(),
                    // attributes: Vec::new(),
                    items: vec![item2],
                    attributes: get_attribute_array(),
                });

                map.add(Component::Tile(tile));
            }
        }
    }

    map
}

pub fn get_attribute_array() -> Vec<ItemAttribute> {
    let chance = rand::thread_rng().gen_range(0..=100);
    if chance < 1 {
        return vec![ItemAttribute::Count(rand::thread_rng().gen_range(0..=5))];
    }

    if chance < 2 {
        return vec![ItemAttribute::HouseId(rand::thread_rng().gen_range(0..=u16::MAX))];
    }

    if chance < 3 {
        return vec![ItemAttribute::Charges(rand::thread_rng().gen_range(0..=5))];
    }

    if chance < 4 {
        return vec![ItemAttribute::ActionId(rand::thread_rng().gen_range(300..=305))];
    }

    if chance < 5 {
        return vec![ItemAttribute::UniqueId(rand::thread_rng().gen_range(300..=305))];
    }

    if chance < 6 {
        return vec![ItemAttribute::DepotId(rand::thread_rng().gen_range(300..=305))];
    }

    if chance < 7 {
        return vec![ItemAttribute::Text(rand::thread_rng().gen_range(10000..=10005).to_string())];
    }

    if chance < 8 {
        return vec![ItemAttribute::Flags(rand::thread_rng().gen_range(300..=305))];
    }

    Vec::new()
}
