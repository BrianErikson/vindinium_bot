extern crate vindinium_bot;
extern crate rustc_serialize;
extern crate term;
use std::string::String;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::convert::From;
use vindinium_bot::vindinium::{State};
use vindinium_bot::vindinium;
use vindinium_bot::pathing;
use vindinium_bot::pathing::{Map};
use rustc_serialize::json;
use term::{Terminal};
use term::color;

fn print_over(path: &pathing::Path, map: &Map) {
    let mut term = term::stdout().unwrap();

    // print tiles and path on board
    for row in &map.grid {
        for cell in row {
            let p_cell = path.iter()
                .filter(|p_cell| p_cell.pos == cell.pos)
                .next();
            let s: String = match p_cell {
                Some(_) => {
                    term.bg(color::BRIGHT_BLACK).unwrap();
                    term.fg(color::WHITE).unwrap();
                    "..".to_string()
                },
                None => vindinium::get_tile_rep(&cell.tile, &mut term)
            };
            (write!(term, "{}", s)).unwrap();
        }
        term.bg(color::BLACK).unwrap();
        term.fg(color::WHITE).unwrap();
        (writeln!(term,"")).unwrap();
    }
}

#[test]
fn pathing_ok() {
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
    let player_pos: pathing::UVector2 = pathing::UVector2 {
        x: state.hero.pos.x as usize, y: state.hero.pos.y as usize
    };
    let w_path = pathing::gen_path(
        &player_pos,
        &pathing::UVector2 { x: player_pos.x + 8, y: player_pos.y },
        &map
    );
    assert!(w_path.is_some());
    let w_path = pathing::gen_path(
        &player_pos,
        &pathing::UVector2 { x: 0, y: 0 },
        &map
    );
    assert!(w_path.is_none());
}

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
    let w_path = pathing::gen_path(
        &player_pos,
        &pathing::UVector2{x: player_pos.x + 8, y: player_pos.y},
        &map
    );
    let path: pathing::Path = match w_path {
        Some(path) => path,
        None => {panic!("Error occurred while computing path.");}
    };
    print_over(&path, &map);
}