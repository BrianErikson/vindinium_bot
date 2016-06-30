use vindinium::{Tile, Board};
use std::convert::From;
use std::collections::LinkedList;
use std::collections::HashMap;

pub type Grid = Vec<Vec<Cell>>;
pub type Path = LinkedList<Cell>;

#[derive(Debug, PartialEq, Clone)]
pub struct IVector2 {
    pub x: isize,
    pub y: isize
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct UVector2 {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub tile: Tile,
    pub pos: UVector2,
    pub parent_pos: UVector2,
    pub f: u8,
    pub g: u8,
    pub h: u8
}

#[derive(Debug, Clone)]
pub struct Map {
    pub size: u8,
    pub grid: Grid
}

impl<'a> From<&'a Board> for Map {
    fn from(board: &'a Board) -> Self {
        let tiles = &board.tiles;
        let mut cells: Grid = Grid::new();

        for x in 0..board.size {
            let mut new_row: Vec<Cell> = Vec::new();
            for y in 0..board.size {
                let pos = UVector2 {x: x, y: y};
                new_row.push(
                    Cell {
                        tile: tiles[x][y].clone(),
                        pos: pos.clone(),
                        parent_pos: pos.clone(),
                        f: 0, g: 0, h: 0
                    });
            }
            cells.push(new_row);
        }

        Map {size: board.size as u8, grid: cells}
    }
}

impl<'a> From<&'a UVector2> for IVector2 {
    fn from(vec: &'a UVector2) -> Self {
        IVector2 {x: vec.x as isize, y: vec.y as isize}
    }
}

impl<'a> From<&'a IVector2> for UVector2 {
    fn from(vec: &'a IVector2) -> Self {
        let x = vec.x.abs();
        let y = vec.y.abs();
        UVector2 {x: x as usize, y: y as usize}
    }
}

fn calc_neighbor(cp: &UVector2, ref_cell: &Cell, target_pos: &UVector2) -> Cell {
    let mut cell = ref_cell.clone();
    let tp: IVector2 = IVector2::from(target_pos);

    // calculate rough manhattan distance from target
    let i_cp: IVector2 = IVector2::from(&cell.pos);
    let h = 10_isize*((i_cp.x-tp.x).abs() + (i_cp.y-tp.y).abs());

    assert!(h <= 255);
    cell.h = h as u8;
    cell.g = 10_u8;

    // sum score
    cell.f = cell.g + cell.h;

    if cell.tile != Tile::Free {
       cell.f = 255_u8; // cannot move into space!
    }

    cell.parent_pos = cp.clone();
    cell
}

fn cell_index_valid(row: isize, column: isize, grid_size: usize) -> bool {
    let size: isize = grid_size as isize;

    if (row-1 >= 0 && column-1 >= 0) && (row+1 < size && column+1 < size) {true}
    else {false}
}

/// Calculates f, g, and h values for each cell surrounding the `cp` parameter.
/// - Returns surrounding neighbors with calculated values
fn calc_neighbors(cp: &UVector2, target_pos: &UVector2, cells: &Grid, grid_size: usize) -> HashMap<UVector2, Cell> {
    // ul-uv-ur
    // hl-cp-hr
    // dl-dv-dr
    let i_grid_size = grid_size as isize;
    let i_cp = IVector2::from(cp);
    let mut open_cells: Vec<Cell> = vec!();

    // quick constrain bounds
    //println!("x: {} y: {}", cp.x, cp.y);
    if (i_cp.x-1 >= 0 && i_cp.y-1 >= 0) && (i_cp.x+1 < i_grid_size && i_cp.y+1 < i_grid_size) {
        // now safe to not do bounds checking
        open_cells.push(calc_neighbor(cp, &cells[cp.x][cp.y+1], target_pos)); // uv
        open_cells.push(calc_neighbor(cp, &cells[cp.x+1][cp.y], target_pos)); // hr
        open_cells.push(calc_neighbor(cp, &cells[cp.x][cp.y-1], target_pos)); // dv
        open_cells.push(calc_neighbor(cp, &cells[cp.x-1][cp.y], target_pos)); // dl
    }
    // slow constrain bounds :(
    else {
        for x in 0..3 {
            for y in 0..3 {
                let cell_ind = ((x as isize) - 1, (y as isize) - 1);
                if cell_ind.0 == 0 && cell_ind.1 == 0 {continue}
                // excluding diagonal neighbors
                match cell_ind {
                    (-1,-1) => {},
                    (-1, 1) => {},
                    (1, 1)  => {},
                    (1, -1) => {},
                    (_, _)  => {
                        if cell_index_valid(cell_ind.0, cell_ind.1, grid_size) {
                            open_cells.push(
                                calc_neighbor(
                                    cp,
                                    &cells[cell_ind.0 as usize][cell_ind.1 as usize],
                                    target_pos
                                )
                            );
                        }
                    }
                };
            }
        }
    }
    // end constrain bounds
    let mut map = HashMap::new();
    for cell in open_cells {
        map.insert(cell.pos.clone(), cell.clone());
    }
    map
}

pub fn gen_path(bot_pos: &UVector2, target_pos: &UVector2, map: &Map) -> Path {
    let path_grid = &map.grid;
    let mut open_nodes: HashMap<UVector2, Cell> = HashMap::new();
    let mut closed_nodes: HashMap<UVector2, Cell> = HashMap::new();
    let start_cell = path_grid[bot_pos.x][bot_pos.y].clone();

    open_nodes.insert(start_cell.pos.clone(), start_cell);

    // gather optimal f-val cells
    while !open_nodes.is_empty() || !closed_nodes.contains_key(&target_pos) {
        // Find best node in open list
        let mut best_node = open_nodes.values().next().unwrap().clone();
        for (_, node) in &open_nodes {
            if node.f <= best_node.f {
                best_node = node.clone();
            }
        }

        // calculate and get neighbors to current cell
        let mut neighbors = calc_neighbors(&best_node.pos, &target_pos, &path_grid, map.size as usize);

        // pop most optimal node of open cells and add to closed cells
        open_nodes.remove(&best_node.pos);
        closed_nodes.insert(best_node.pos.clone(), best_node.clone());

        // Remove new neighbors if it is already in the closed list
        // check to see if open cell is better than closed
        for (key, _) in &closed_nodes {
            neighbors.remove(key);
        }

        // Remove new neighbors if already in the open list--update open cell if g val is better
        for (key, mut node) in &mut open_nodes {
            let res = match neighbors.get(key) {
                Some(other) => {
                    if node.g > other.g {
                        node.f = other.f;
                        node.g = other.g;
                        node.h = other.h;
                        node.parent_pos = other.parent_pos.clone();
                    }
                    Some(other.pos.clone())
                },
                None => None
            };

            match res {
                Some(key) => {neighbors.remove(&key);},
                None => {}
            }
        }

        // append new neighbors
        open_nodes.extend(neighbors);
    }

    // determine path by walking backwards from the destination
    closed_nodes.remove(&bot_pos);
    let mut path: Path = Path::new();

    if !closed_nodes.contains_key(&target_pos) {
        panic!("Could not find a valid path to position: {:#?} from {:#?}", target_pos, bot_pos);
    }

    let w_end_node = &closed_nodes.get(&target_pos);
    if w_end_node.is_some() {
        let mut cur_node = w_end_node.unwrap();
        path.push_front(cur_node.clone());
        while cur_node.parent_pos != cur_node.pos && cur_node.pos != *bot_pos { // parent pos == node pos if no parent
            let w_node = closed_nodes.get(&cur_node.parent_pos);
            if w_node.is_some() {
                cur_node = w_node.unwrap();
                path.push_front(cur_node.clone());
            }
            else {
                println!("Error in path gen. Breaking on {:#?}", cur_node);
                break;
            }
        }
    }

    path
}