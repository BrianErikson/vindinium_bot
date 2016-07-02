use std::convert::From;
use vindinium::{Dir, State};
use pathing;
use pathing::{UVector2, IVector2, Map};


pub fn step(state: &State) -> Dir {
    let cur_pos = UVector2{x: state.hero.pos.x as usize, y: state.hero.pos.y as usize};
    let w_path = pathing::gen_path(
        &cur_pos,
        &UVector2{x:5,y:5},
        &Map::from(&state.game.board)
    );

    match w_path {
        Some(path) => {
            let cur_pos = IVector2::from(&cur_pos);
            let new_pos = IVector2::from(&path.front().unwrap().pos);

            let dir = match (cur_pos.x - new_pos.x, cur_pos.y - new_pos.y) {
                (0,1) => Dir::North,
                (1,0) => Dir::East,
                (0,-1) => Dir::South,
                (-1,0) => Dir::West,
                (_,_) => panic!("Could not determine direction returned from path!")
            };
            dir
        },
        None => { // Path from bot pos to target is not valid
            // TODO: Find a new target?
            Dir::Stay
        }
    }
}
