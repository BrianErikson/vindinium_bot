extern crate rand;
use self::rand::distributions::{IndependentSample, Range};
use vindinium::{Bot, Dir, State};
use pathing::step_towards;


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

    fn step(&self, _: &State) -> EmergentBot {
        let mut bot: EmergentBot = self.clone();
        bot

//        let mut rng = rand::thread_rng();
//        let range = Range::new(0, 5);
//        let new_dir = range.ind_sample(&mut rng);
//        let new_dir = match new_dir {
//            0 => Dir::North,
//            1 => Dir::East,
//            2 => Dir::South,
//            3 => Dir::West,
//            4 => Dir::Stay,
//            _ => panic!("Random value out of range! Value: {}", new_dir)
//        };
//        bot.dir = new_dir;
//        bot
    }

    fn dir(&self) -> Dir {
        self.dir.clone()
    }
}
