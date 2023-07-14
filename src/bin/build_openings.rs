// Program that compiles OPENINGS into match arms that can
// be pasted into openings.rs

use puccinia_s_checkmate::misc::str2move;
use puccinia_s_checkmate::val::*;
use puccinia_s_checkmate::Game;
use std::collections::hash_map::HashMap;

#[rustfmt::skip]
const OPENINGS: [&str;107]=[
        "#Spansk_v1",
        "e2e4", "e7e5", 
        "g1f3", "b8c6", 
        "f1b5", "a7a6", 
        "b5a4", "g8f6", 
        "e1g1", "f8e7", 
        "f1e1", "b7b5",
        "a4b3", "d7d6", 
        "c2c3", "e8g8",
        "#Spansk_v2",
        "e2e4", "e7e5", 
        "g1f3", "b8c6", 
        "f1b5", "a7a6", 
        "b5a4", "g8f6", 
        "e1g1", "f6e4",
        "#Spansk_v3",
        "e2e4", "e7e5",
        "g1f3", "b8c6",
        "f1b5", "a7a6",
        "b5c6", "d7c6",
        "#Philidors_Forsvar_v1",
        "e2e4", "e7e5", 
        "g1f3", "d7d6", 
        "d2d4", "e5d4",
        "#Philidors_Forsvar_v2",
        "e2e4", "e7e5", 
        "g1f3", "d7d6", 
        "d2d4", "b8d7",
        "#Fransk",
        "e2e4", "e7e6", 
        "d2d4", "d7d5", 
        "b1c3", "g8f6", 
        "g1f3", "f8e7",
        "#Caro-Kann",
        "e2e4","c7c6",
        "d2d4","d7d5",
        "b1c3","d5e4",
        "c3e4","c8f5",
        "#Siciliansk.",
        "e2e4","c7c5",
        "g1f3","d7d6",
        "d2d4","c5d4",
        "f3d4","g8f6",
        "#Dronninggambit",
        "d2d4","d7d5",
        "c2c4","e7e6",
        "b1c3","g8f6",
        "c1g5","f8e7",
        "#Nimzo-Indisk",
        "d2d4","g8f6",
        "c2c4","e7e6",
        "b1c3","f8b4",
        "d1c2","b8c6",
        "#Dronningeindisk",
        "d2d4","g8f6",
        "c2c4","e7e6",
        "g1f3","b7b6",
        "g2g3","c8b7",
        "f1g2","f8e7",
];

fn main() {
    let mut game = Game::new(ROOT_BOARD);
    let mut h: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();
    for s in &OPENINGS {
        if &s[0..1] == "#" {
            //println!("Opening: {s}");
            game = Game::new(ROOT_BOARD);
        } else {
            let (frm, to) = str2move(s);
            let moves = game.legal_moves();
            if let Some(m) = moves.iter().find(|m| (m.frm, m.to) == (frm, to)) {
                h.entry(game.hash)
                    .and_modify(|x| {
                        if !x.iter().any(|(frm, to)| (*frm, *to) == (m.frm, m.to)) {
                            x.push((m.frm, m.to))
                        }
                    })
                    .or_insert(vec![(m.frm, m.to)]);
                game.update(m);
            } else {
                panic!("Not a legal move");
            }
        }
    }
    for (k, v) in h {
        println!("{k} => &{:?}[0..{}],", v, v.len())
    }
}
