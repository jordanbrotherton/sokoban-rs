use std::string;

use crate::cell::Cell;

struct Grid {
    width: usize,
    height: usize,
    contents: Vec<Cell>,
}

impl Grid {
    pub fn new(xsb: String) -> Grid {
        let mut contents = vec![];

        let lines = xsb.lines();
        let x: usize = 0;
        let y: usize = 0;
        for line in lines {
            if line.len() > x {
                x = line.len();
            }
            for char in line {}
            // TODO - make a grid based on each type
            y += 1;
        }
        Grid {
            width: x,
            height: y,
            //contents,
        }
    }
}
