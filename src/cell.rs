use std::collections::HashMap;

pub enum CellObjects {
    Floor,
    Player,
    PlayerOnGoal,
    Wall,
    Box,
    Goal,
    BoxOnGoal,
}

pub struct Cell {
    sprite: char,
    x: u32,
    y: u32,
    movable: bool,
    collidable: bool,
    object_type: CellObjects,
}

impl Cell {
    pub fn new(x: u32, y: u32, object_type: CellObjects) -> Cell {
        Cell {
            sprite: Self::lookup_sprite(&object_type),
            x,
            y,
            movable: Self::lookup_movable(&object_type),
            collidable: Self::lookup_collidable(&object_type),
            object_type,
        }
    }

    fn lookup_sprite(object_type: &CellObjects) -> char {
        return match object_type {
            CellObjects::Floor => ' ',
            CellObjects::Player => '@',
            CellObjects::PlayerOnGoal => '+',
            CellObjects::Wall => '#',
            CellObjects::Box => '$',
            CellObjects::Goal => '.',
            CellObjects::BoxOnGoal => '*',
        };
    }

    fn lookup_movable(object_type: &CellObjects) -> bool {
        match object_type {
            CellObjects::Player => true,
            CellObjects::Box => true,
            CellObjects::BoxOnGoal => true,
            _ => false,
        }
    }

    fn lookup_collidable(object_type: &CellObjects) -> bool {
        match object_type {
            CellObjects::Floor => false,
            CellObjects::Goal => false,
            _ => true,
        }
    }
}
