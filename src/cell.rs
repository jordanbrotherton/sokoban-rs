use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CellObjects {
    Floor,
    Player,
    PlayerOnGoal,
    Wall,
    Box,
    Goal,
    BoxOnGoal,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub object_type: CellObjects,
}

impl Cell {
    pub fn new(x: usize, y: usize, sprite: char) -> Cell {
        Cell {
            x,
            y,
            object_type: Self::get_object(&sprite),
        }
    }

    pub fn from_object(x: usize, y: usize, object_type: CellObjects) -> Cell {
        Cell { x, y, object_type }
    }

    pub fn get_sprite(&self) -> char {
        return match self.object_type {
            CellObjects::Floor => ' ',
            CellObjects::Player => '@',
            CellObjects::PlayerOnGoal => '+',
            CellObjects::Wall => '#',
            CellObjects::Box => '$',
            CellObjects::Goal => '.',
            CellObjects::BoxOnGoal => '*',
        };
    }

    pub fn get_object(sprite: &char) -> CellObjects {
        return match sprite {
            ' ' => CellObjects::Floor,
            '@' => CellObjects::Player,
            '+' => CellObjects::PlayerOnGoal,
            '#' => CellObjects::Wall,
            '$' => CellObjects::Box,
            '.' => CellObjects::Goal,
            '*' => CellObjects::BoxOnGoal,
            _ => CellObjects::Floor,
        };
    }

    pub fn is_moveable(&self) -> bool {
        match self.object_type {
            CellObjects::Player => true,
            CellObjects::PlayerOnGoal => true,
            CellObjects::Box => true,
            CellObjects::BoxOnGoal => true,
            _ => false,
        }
    }

    pub fn is_on_goal(&self) -> bool {
        match self.object_type {
            CellObjects::PlayerOnGoal => true,
            CellObjects::BoxOnGoal => true,
            _ => false,
        }
    }

    pub fn is_collidable(&self) -> bool {
        match self.object_type {
            CellObjects::Floor => false,
            CellObjects::Goal => false,
            _ => true,
        }
    }
}
