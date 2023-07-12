// Copyright (c) 2022 Jesper Olsen
// License: MIT, see License.txt
//
// Golden Monkey Chess - small chess engine implemented in rust

//use crate::val::*;
use ::std::time::Instant;
use clap::Parser;
use puccinia_s_checkmate::mgen::*;
use puccinia_s_checkmate::openings::*;
use puccinia_s_checkmate::val::*;
use puccinia_s_checkmate::Game;
use rand::random;
use std::collections::hash_map::HashMap;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1000000)]
    ///break off search threshold - positions generated
    n: usize,
    #[arg(short, long, default_value_t = 25)]
    ///max depth of regular search  
    d: usize,
    #[arg(short, long, default_value_t = -1)]
    ///number of (half) moves before stopping
    m: isize,
    #[arg(short, long, default_value_t = false)]
    ///play white (human-computer)
    w: bool,
    #[arg(short, long, default_value_t = false)]
    ///play black (computer-human)
    b: bool,
    #[arg(short, long, default_value_t = false)]
    ///verbose output
    v: bool,
}

fn i2str(i: usize) -> String {
    let s = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let x = 7 - i / 8;
    let y = i % 8 + 1;
    format!("{}{}", s[x], y)
}

//https://cheatography.com/davechild/cheat-sheets/chess-algebraic-notation/
fn move2label(board: &[Piece; 64], m: &Move, moves: &Vec<Move>) -> String {
    let mut label = String::new();
    if m.castle {
        if m.to < 31 {
            label.push_str("0-0 ");
        } else {
            label.push_str("0-0-0 ");
        }
    } else if m.transform.0 != NIL {
        label.push('*');
    }
    if board[m.frm].ptype != PType::Pawn {
        let p = format!("{}", board[m.frm]).to_uppercase();
        label.push_str(&p);
    }

    let mut l = Vec::new();
    for m2 in moves {
        if m2.to == m.to && board[m.frm].ptype == board[m2.frm].ptype {
            l.push(i2str(m2.to));
        }
    }
    if l.len() > 1 {
        // push file - or rank if same
        let n = if l[0] == l[1] { 1 } else { 0 };
        label.push(l[0].chars().nth(n).unwrap());
    }

    if m.kill.0 != NIL {
        if board[m.frm].ptype == PType::Pawn {}
        label.push('x');
    }
    label.push_str(&i2str(m.to));
    label
}

fn pick_move(moves: &[Move], label: &str) -> Vec<(Move, i32)> {
    loop {
        println!("Your Move ({label}):");
        let s = get_input();
        if s.as_str() == "q" {
            std::process::exit(1);
        } else {
            let (frm, to) = str2move(s.as_str());
            if let Some(m) = moves.iter().find(|m| (m.frm, m.to) == (frm, to)) {
                return vec![(*m, 0)];
            } else {
                println!("Not valid");
            }
        }
    }
}

fn check_game_over(game: &Game, moves: &Vec<Move>, half_moves: isize) -> String {
    if game.rep_count() >= 3 {
        "1/2-1/2 Draw by repetition".to_string()
    } else if game.check_50_move_rule() {
        "1/2-1/2 Draw by the 50-move rule".to_string()
    } else if half_moves != -1 && half_moves <= game.log.len() as isize {
        format!("stopping after {} half move(s)", game.log.len())
    } else if moves.is_empty() {
        (match (game.in_check(game.turn()), game.turn()) {
            (true, BLACK) => "1-0",
            (true, WHITE) => "0-1",
            (false, _) => "1/2-1/2 Draw",
        })
        .to_string()
    } else {
        "".to_string()
    }
}

fn play(
    players: HashMap<bool, bool>,
    verbose: bool,
    search_threshold: usize,
    max_depth: usize,
    half_moves: isize,
) {
    let mut game = Game::new(ROOT_BOARD);
    println!("{}", game);

    let mut tot = 0;
    let mut moves = game.legal_moves();
    let openings = openings();
    if verbose {
        let x: usize = openings.values().map(|v| v.len()).sum();
        println!("Opening positions: {v} - Moves: {x}", v = openings.len());
    }

    let start = Instant::now();
    loop {
        let msg = check_game_over(&game, &moves, half_moves);
        if !msg.is_empty() {
            println!("{}", msg);
            std::process::exit(1);
        }

        let l = if players[&game.turn()] {
            let label = if game.turn() == WHITE {
                "White"
            } else {
                "Black"
            };
            pick_move(&moves, label)
        } else {
            // try library 1st - compute if not there
            if let Some(o) = openings.get(&game.hash) {
                let i = random::<usize>() % o.len();
                for (q, x) in o.iter().enumerate() {
                    if verbose {
                        if q == i {
                            println!("Opening: {},{} (picked)", i2str(x.0), i2str(x.1));
                        } else {
                            println!("Opening: {},{}", i2str(x.0), i2str(x.1));
                        }
                    }
                }
                if let Some(m) = moves.iter().find(|m2| (m2.frm, m2.to) == o[i]) {
                    vec![(*m, 0i32)]
                } else {
                    panic!("Invalid library move")
                }
            } else {
                game.score_moves(&moves, search_threshold, max_depth, verbose)
            }
        };

        if verbose {
            tot += game.n_searched;
            let speed = if let Some(speed) =
                (tot as u128).checked_div((Instant::now() - start).as_millis())
            {
                speed as usize
            } else {
                0
            };
            println!(
                "Search total: {} / {} ms / {} nodes/ms",
                tot,
                (Instant::now() - start).as_millis() as usize,
                speed
            );
            println!("hash size r {} t {} ", game.rep_len(), game.ttable_len(),);
            for (i, (m, score)) in l.iter().enumerate() {
                println!("{}/{}: {} {}/{}", i, moves.len(), m, m.val, score);
            }
        }
        let (m, score) = l[0];

        let label = move2label(&game.board, &m, &moves);
        game.make_move(m);
        println!("{}", game);
        moves = game.legal_moves();
        let s = match (game.in_check(game.turn()), moves.is_empty()) {
            (true, true) => "#",
            (true, false) => "+",
            (false, _) => "",
        };
        println!("{}. {}{}", game.log.len() / 2 + 1, label, s);

        if verbose {
            println!(
                "score: {}, material: {}, abs: {}, pawns: {}, mobility: {}",
                score,
                material(&game.board),
                abs_material(&game.board),
                game.score_pawn_structure(),
                game.mobility()
            );
        }
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    String::from(s.trim())
}

fn main() {
    let args = Args::parse();

    let players = HashMap::from([(WHITE, args.w), (BLACK, args.b)]);
    play(players, args.v, args.n, args.d, args.m);
}
