use std::convert::From;
use std::cmp::Ordering;
use vindinium::{Dir, State, Tile};
use pathing;
use pathing::{UVector2, IVector2, Map};

const MAX_HEALTH: u8 = 100;
const LOW_HEALTH_PER: f32 = 0.25; // represented as percent of max health
const LOW_HEALTH: u8 = ((MAX_HEALTH as f32) * LOW_HEALTH_PER) as u8;
const CLOSE_RADIUS: u8 = 3; // In tiles

struct Location {
    pos: UVector2,
    tile: Tile
}

fn find_destination(state: &State) -> Option<UVector2> {

    let hero_pos = UVector2::from(&state.hero.pos);
    let health = state.hero.life;
    let other_heroes = state.game.heroes.iter().filter(|hero| hero.id != state.hero.id);
    let mut taverns: Vec<Location> = vec!();
    let mut mines: Vec<Location> = vec!();
    for (x, row) in state.game.board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            match *tile {
                Tile::Tavern => {
                    taverns.push(Location {pos: UVector2 {x: x, y: y}, tile: tile.clone()});
                },
                Tile::Mine(_) => {
                    mines.push(Location {pos: UVector2 {x: x, y: y}, tile: tile.clone()});
                },
                _ => {}
            }
        }
    }
    let dist_sort = &|a: &Location, b: &Location| -> Ordering {
        hero_pos.distance_from(&a.pos).cmp(&hero_pos.distance_from(&b.pos))
    };
    taverns.sort_by(dist_sort);
    mines.sort_by(dist_sort);

    if health as u8 <= LOW_HEALTH {
        return Some(taverns[0].pos.clone()) // returns closest tavern
    }
    //else if player close by
    //else if mine close by
    //else if other player lower health than i
    //else if unclaimed mine
    //else claim owned mine

    None
}

fn get_direction(pos1: &UVector2, pos2: &UVector2) -> Dir {
    let cur_pos = IVector2::from(pos1);
    let new_pos = IVector2::from(pos2);

    return match (cur_pos.x - new_pos.x, cur_pos.y - new_pos.y) {
        (0,1) => Dir::North,
        (1,0) => Dir::East,
        (0,-1) => Dir::South,
        (-1,0) => Dir::West,
        (_,_) => {
            println!("Could not determine direction returned from path!");
            Dir::Stay
        }
    };
}

pub fn step(state: &State) -> Dir {
    let cur_pos = UVector2{x: state.hero.pos.x as usize, y: state.hero.pos.y as usize};
    let w_destination = find_destination(state);
    let dir = match w_destination {
        Some(dest) => {
            let w_path = pathing::gen_path(
                &cur_pos, &dest, &Map::from(&state.game.board)
            );
            match w_path {
                Some(path) => get_direction(&cur_pos, &path.front().unwrap().pos),
                None => Dir::Stay //Path from bot pos to target is not valid TODO: Find a new target?
            }
        }
        None => {
            println!("ERROR: Could not determine a destination.");
            Dir::Stay
        }
    };

    dir
}
