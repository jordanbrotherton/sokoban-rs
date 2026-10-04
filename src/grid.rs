use std::string;

use crate::cell::{
    Cell,
    CellObjects::{self, Goal, Player},
};

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    contents: Vec<Cell>,
    pub player: Cell,
    pub goal_count: usize,
}

impl Grid {
    pub fn new(xsb: String) -> Grid {
        let mut contents = vec![];

        let mut player = Cell::new(0, 0, '@');
        let mut lines = xsb.lines();
        let mut goal_count: usize = 0;
        let width: usize = lines.max_by_key(|s| s.len()).map(|s| s.len()).unwrap_or(0);
        let mut height: usize = 0;
        lines = xsb.lines();
        for line in lines {
            let mut current_x = 0;
            for char in line.chars() {
                let cell = Cell::new(current_x, height, char);
                if cell.object_type == CellObjects::Player {
                    player = cell;
                } else if cell.object_type == CellObjects::Goal {
                    goal_count += 1;
                }
                contents.push(cell);
                current_x += 1;
            }
            while current_x < width {
                let cell = Cell::new(current_x, height, ' ');
                contents.push(cell);
                current_x += 1;
            }

            height += 1;
        }
        Grid {
            width,
            height,
            contents,
            player,
            goal_count,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let index = x + self.width * y;
        if index < self.contents.len() {
            Some(self.contents[index])
        } else {
            None
        }
    }

    /// Gets the neighbor, given an X from -1 to 1 or a Y from -1 to 1.
    pub fn get_neighbor(&self, cell: &Cell, x: i32, y: i32) -> Option<Cell> {
        let target_x = (cell.x as i32 + x);
        let target_y = (cell.y as i32 + y);
        if target_x >= 0 && target_y >= 0 {
            self.get_cell(target_x as usize, target_y as usize)
        } else {
            None
        }
    }

    pub fn move_object(&mut self, mut cell: Cell, x: i32, y: i32) -> bool {
        if !cell.is_moveable() {
            return false;
        }

        let Some(mut neighbor): Option<Cell> = self.get_neighbor(&cell, x, y).clone() else {
            return false;
        };

        if cell.object_type == CellObjects::Player || cell.object_type == CellObjects::PlayerOnGoal
        {
            if neighbor.is_moveable() && !self.move_object(neighbor, x, y) {
                return false;
            } else if (neighbor.is_moveable()) {
                return self.move_object(cell, x, y);
            }
        }

        if neighbor.is_collidable() {
            return false;
        }

        let target_x = (cell.x as i32 + x) as usize;
        let target_y = (cell.y as i32 + y) as usize;
        neighbor.x = cell.x;
        neighbor.y = cell.y;
        cell.x = target_x;
        cell.y = target_y;

        if cell.is_on_goal() {
            neighbor.object_type = CellObjects::Goal;
            cell.object_type = match cell.object_type {
                CellObjects::PlayerOnGoal => CellObjects::Player,
                CellObjects::BoxOnGoal => CellObjects::Box,
                _ => CellObjects::Floor,
            };
            if (cell.object_type == CellObjects::Box) {
                self.goal_count += 1;
            }
        } else {
            if neighbor.object_type == CellObjects::Goal {
                cell.object_type = match cell.object_type {
                    CellObjects::Player => CellObjects::PlayerOnGoal,
                    CellObjects::Box => CellObjects::BoxOnGoal,
                    _ => CellObjects::Floor,
                };
                neighbor.object_type = CellObjects::Floor;
                if (cell.object_type == CellObjects::BoxOnGoal) {
                    self.goal_count -= 1;
                }
            } else {
                neighbor.object_type = CellObjects::Floor;
            }
        }

        let cached_neighbor = neighbor.clone();
        self.contents[cell.x + self.width * cell.y] = cell.clone();
        self.contents[neighbor.x + self.width * neighbor.y] = cached_neighbor;
        if (self.contents[cell.x + self.width * cell.y].object_type == CellObjects::Player
            || self.contents[cell.x + self.width * cell.y].object_type == CellObjects::PlayerOnGoal)
        {
            self.player = self.contents[cell.x + self.width * cell.y];
        }
        true
    }

    pub fn print_grid(&self) {
        // Clear Screen.
        // Iterate through contents.
        // Create string and append the sprites to it.
        // Check Y, and finish the line if new.
        // Repeat.
        clearscreen::clear();
        println!(
            "Width: {}\nHeight: {}\nPlayer: ({}, {})\nGoals Left: {}",
            self.width, self.height, self.player.x, self.player.y, self.goal_count
        );
        let mut current_y = 0;
        let mut output_line = String::new();
        for cell in &self.contents {
            if cell.y != current_y {
                current_y = cell.y;
                println!("{}", output_line);
                output_line = String::new();
            }
            output_line.push_str(&cell.get_sprite().to_string());
        }
        println!("{}", output_line);
    }
}
