// Generate openings.rs

use mateus::Game;
use mateus::misc::str2move;
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
    let mut game = Game::default();
    let mut h: HashMap<u64, Vec<(u8, u8)>> = HashMap::new();
    let mut log = vec![];
    for s in &OPENINGS {
        if &s[0..1] == "#" {
            //println!("Opening: {s}");
            game = Game::default();
            log = vec![];
            continue;
        }

        let Some((frm, to)) = str2move(s) else {
            panic!("Not a legal move");
        };
        let moves = game.board.legal_moves();
        let Some(m) = moves.iter().find(|m| (m.frm(), m.to()) == (frm, to)) else {
            panic!("Not a legal move");
        };
        h.entry(game.board.hash)
            .and_modify(|x| {
                if !x.iter().any(|(frm, to)| (*frm, *to) == (m.frm(), m.to())) {
                    x.push((m.frm(), m.to()))
                }
            })
            .or_insert(vec![(m.frm(), m.to())]);
        game.board.update(m);
        log.push(*m);
    }
    println!("// file generated by bin/build_openings.rs");
    println!("// copy to src/openings.rs \n");
    let s = "pub fn library_moves(hash: u64) -> &'static [(usize, usize)] {
    match hash {";
    println!("{s}");
    for (k, v) in h {
        println!("        {k} => &{:?}[0..{}],", v, v.len())
    }
    let s = "        _ => &[][0..0], } }";
    println!("{s}");
}
