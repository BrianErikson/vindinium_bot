use vindinium::{Tile};

pub trait ToGrid {
    fn to_grid(&self) -> Grid;
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: i8,
    pub y: i8
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub tile: Tile,
    pub pos: Vector2,
    pub f: i8,
    pub g: i8,
    pub h: i8
}

pub struct Grid {
    pub size: i8,
    pub cells: Vec<Vec<Cell>>
}

// TODO:
//fn do_things(grid: &Grid) {
//    for row in &grid.cells {
//        for cell in row {
//
//        }
//    }
//}