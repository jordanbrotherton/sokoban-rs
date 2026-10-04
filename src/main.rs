mod cell;
mod grid;

use crate::grid::Grid;
use std::io::{self, Write};

fn main() {
    let mut grid = Grid::new(String::from(
        "##########\n# @       ##\n#  # $   $  #\n#  . .     #\n###########",
    ));
    let mut undo_grid = grid.clone();
    grid.print_grid();
    loop {
        if (grid.goal_count == 0) {
            println!("Level complete!");
            break;
        }

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input == "w" {
            undo_grid = grid.clone();
            grid.move_object(grid.player, 0, -1);
        }
        if input == "a" {
            undo_grid = grid.clone();
            grid.move_object(grid.player, -1, 0);
        }
        if input == "s" {
            undo_grid = grid.clone();
            grid.move_object(grid.player, 0, 1);
        }
        if input == "d" {
            undo_grid = grid.clone();
            grid.move_object(grid.player, 1, 0);
        }
        if input == "undo" {
            grid = undo_grid.clone();
        }
        grid.print_grid();
    }
}
