use std::convert::From;
use std::cmp::Ordering;
use vindinium::{Dir, State, Tile, Hero};
use pathing;
use pathing::{UVector2, IVector2, Map};

const MAX_HEALTH: u8 = 100;
const LOW_HEALTH_PER: f32 = 0.25; // represented as percent of max health
const LOW_HEALTH: u8 = ((MAX_HEALTH as f32) * LOW_HEALTH_PER) as u8;
const CLOSE_RADIUS: usize = 3; // In tiles

struct Location {
    pos: UVector2,
    tile: Tile
}

fn find_destination(state: &State) -> Option<UVector2> {

    let hero_pos = UVector2::from(&state.hero.pos);
    let bot_life = state.hero.life;
    let mut other_heroes: Vec<&Hero> = state.game.heroes.iter()
                                                        .filter(|hero| hero.id != state.hero.id)
                                                        .collect::<Vec<&Hero>>();
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

    other_heroes.sort_by(|a, b|
                             hero_pos.distance_from(&UVector2::from(&a.pos))
                                 .cmp(&hero_pos.distance_from(&UVector2::from(&b.pos)))
    );

    let loc_sort = &|a: &Location, b: &Location| -> Ordering {
        hero_pos.distance_from(&a.pos).cmp(&hero_pos.distance_from(&b.pos))
    };
    taverns.sort_by(loc_sort);
    mines.sort_by(loc_sort);

    if bot_life as u8 <= LOW_HEALTH {
        return Some(taverns[0].pos.clone()) // returns closest tavern
    }

    let enemy = other_heroes[0];
    let enemy_pos = UVector2::from(&enemy.pos);
    if hero_pos.distance_from(&enemy_pos) <= CLOSE_RADIUS {

        let enemy_mines = mines.iter().filter(|loc| match loc.tile {
            Tile::Mine(w_hero_id) => match w_hero_id {
                Some(hero_id) if hero_id == enemy.id => true,
                _ => false
            },
            _ => false
        }).collect::<Vec<&Location>>();
        if enemy.life < bot_life && !enemy_mines.is_empty() {
            return Some(enemy_pos)
        }
        else if hero_pos.distance_from(&taverns[0].pos) <= CLOSE_RADIUS {
            return Some(taverns[0].pos.clone())
        }
    }

    if hero_pos.distance_from(&mines[0].pos) <= CLOSE_RADIUS {
        return Some(mines[0].pos.clone())
    }

    let closest_enemy_pos = UVector2::from(&other_heroes[0].pos);
    if other_heroes[0].life < bot_life
        && hero_pos.distance_from(&closest_enemy_pos) <= CLOSE_RADIUS * 2 {

        return Some(closest_enemy_pos)
    }

    let unclaimed = mines.iter().filter(|loc| match loc.tile {
        Tile::Mine(w_hero_id) => w_hero_id.is_none(),
        _ => false
    }).collect::<Vec<&Location>>();

    if !unclaimed.is_empty() {
        return Some(unclaimed[0].pos.clone())
    }
    else {
        return Some(mines[0].pos.clone())
    }

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
