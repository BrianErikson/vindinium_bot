use vindinium::{Tile};

pub trait ToGrid {
    fn to_grid(&self) -> Grid;
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub tile: Tile,
    pub pos: Vector2,
    pub f: i8,
    pub g: i8,
    pub h: i8
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub size: i8,
    pub cells: Vec<Vec<Cell>>
}

fn gen_path(grid: &Grid) {
    for row in &grid.cells {
        for cell in row {

        }
    }
}

fn surr(cp: &Vector2, cells: &mut Vec<Vec<Cell>>, grid_size: usize) {
    // ul-uv-ur
    // hl-cp-hr
    // dl-dv-dr
    let D_WEIGHT_DIAG = 14i8;
    let D_WEIGHT = 10i8;

    // quick constrain bounds
    if (cp.x-1 >= 0 && cp.y-1 >= 0) && (cp.x+1 <= grid_size && cp.y+1 <= grid_size) {
        // now safe to not do bounds checking

        // diagonal d-weights
        *(&mut cells[cp.x-1][cp.y+1].g) = D_WEIGHT_DIAG; // ul
        *(&mut cells[cp.x+1][cp.y+1].g) = D_WEIGHT_DIAG; // ur
        *(&mut cells[cp.x-1][cp.y-1].g) = D_WEIGHT_DIAG; // dl
        *(&mut cells[cp.x+1][cp.y-1].g) = D_WEIGHT_DIAG; // dr
        // end diagonal d-weights
        *(&mut cells[cp.x][cp.y+1].g) = D_WEIGHT; // uv
        *(&mut cells[cp.x+1][cp.y].g) = D_WEIGHT; // hr
        *(&mut cells[cp.x][cp.y-1].g) = D_WEIGHT; // dv
        *(&mut cells[cp.x-1][cp.y].g) = D_WEIGHT; // dl
    }
    // slow constrain bounds :(
    else {
        if cp.x > 0 {
            *(&mut cells[cp.x-1][cp.y].g) = D_WEIGHT;            // hl

            if cp.y < grid_size-1 {
                *(&mut cells[cp.x-1][cp.y+1].g) = D_WEIGHT_DIAG; // ul
                *(&mut cells[cp.x][cp.y+1].g) = D_WEIGHT;        // uv
            }

            if cp.y > 0 {
                *(&mut cells[cp.x-1][cp.y-1].g) = D_WEIGHT_DIAG; // dl
                *(&mut cells[cp.x][cp.y-1].g) = D_WEIGHT;        // dv
            }
        }

        if cp.x < grid_size - 1 {
            *(&mut cells[cp.x+1][cp.y].g) = D_WEIGHT;            // hr

            if cp.y < grid_size-1 {
                *(&mut cells[cp.x+1][cp.y+1].g) = D_WEIGHT_DIAG; // ur
            }

            if cp.y > 0 {
                *(&mut cells[cp.x+1][cp.y-1].g) = D_WEIGHT_DIAG; // dr
            }
        }
    }
    // end constrain bounds
}

pub fn step_towards(bot_pos: &Vector2, target_pos: &Vector2, grid: &Grid) /*-> Vector2*/ {
    let mut path_cells = grid.cells.clone();
    //let ref target_cell = path_cells[target_pos.x][target_pos.y];

    surr(&bot_pos, &mut path_cells, grid.size as usize);
}