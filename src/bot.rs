use std::convert::From;
use vindinium::{Bot, Dir, State};
use pathing;
use pathing::{UVector2, IVector2, Map};


#[derive(Debug, Clone)]
pub struct EmergentBot {
    pub dir: Dir,
}

impl EmergentBot {
    pub fn new() -> EmergentBot {
        EmergentBot {
            dir: Dir::Stay,
        }
    }
}

impl Bot for EmergentBot {

    fn step(&self, state: &State) -> EmergentBot {
        let mut bot: EmergentBot = self.clone();
        let cur_pos = UVector2{x: state.hero.pos.x as usize, y: state.hero.pos.y as usize};
        let path = pathing::gen_path(
            &cur_pos,
            &UVector2{x:5,y:5},
            &Map::from(&state.game.board)
        );

        let cur_pos = IVector2::from(&cur_pos);
        let new_pos = IVector2::from(&path.front().unwrap().pos);

        let new_dir = match (cur_pos.x - new_pos.x, cur_pos.y - new_pos.y) {
            (0,1) => Dir::North,
            (1,0) => Dir::East,
            (0,-1) => Dir::South,
            (-1,0) => Dir::West,
            (_,_) => panic!("Could not determine direction returned from path!")
        };

        bot.dir = new_dir;
        bot
    }

    fn dir(&self) -> Dir {
        self.dir.clone()
    }
}
