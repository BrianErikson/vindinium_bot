extern crate vindinium_bot;
extern crate rustc_serialize;
use std::string::String;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::convert::From;
use vindinium_bot::vindinium::{State, Game, Board};
use vindinium_bot::pathing;
use rustc_serialize::json;

#[test]
//#[ignore]
fn display_path() {
    let mut json_str = String::new();
    let res = File::open(&Path::new("tests/test_state.json")).unwrap().read_to_string(&mut json_str);
    match res {
        Ok(_) => {}
        Err(err) => panic!("{}", err)
    }

    let state: State = match json::decode(&json_str) {
        Ok(state) => state,
        Err(err) => panic!("{}", err)
    };

    let map: pathing::Map = pathing::Map::from(&state.game.board);
    let player_pos: pathing::UVector2 = pathing::UVector2{
        x: state.hero.pos.x as usize, y: state.hero.pos.y as usize};
    let path: pathing::Path = pathing::gen_path(
        &player_pos,
        &pathing::UVector2{x: player_pos.x + 5, y: player_pos.y},
        &map
    );
    pathing::print_over(&path, &map);
}