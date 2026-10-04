mod cell;
mod grid;

use crate::grid::Grid;
use std::io::{self, Write};

fn main() {
    let mut grid = Grid::new(String::from(
        "##########\n# @       ##\n#  # $      #\n#  .       #\n###########",
    ));
    grid.print_grid();
    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input == "w" {
            grid.move_object(grid.player, 0, -1);
        }
        if input == "a" {
            grid.move_object(grid.player, -1, 0);
        }
        if input == "s" {
            grid.move_object(grid.player, 0, 1);
        }
        if input == "d" {
            grid.move_object(grid.player, 1, 0);
        }
        grid.print_grid();
    }
}
