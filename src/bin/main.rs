// Copyright (c) 2022 Jesper Olsen
// License: MIT, see License.txt
//
// Golden Monkey Chess - small chess engine implemented in rust

//use crate::val::*;
use ::std::time::Instant;
use clap::Parser;
use puccinia_s_checkmate::hashkeys_generated::WHITE_HASH;
use puccinia_s_checkmate::mgen::*;
use puccinia_s_checkmate::misc::str2move;
use puccinia_s_checkmate::openings::library_moves;
use puccinia_s_checkmate::val::*;
use puccinia_s_checkmate::Game;
use puccinia_s_checkmate::INFINITE;
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
    ///play black (human-computer)
    b: bool,
    #[arg(short, long, default_value_t = false)]
    ///library bypass
    l: bool,
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

// https://www.chessprogramming.org/Kaufman_Test
const KAUFMAN: [(&str, &str, &str); 25] = [
    (
        "1rbq1rk1/p1b1nppp/1p2p3/8/1B1pN3/P2B4/1P3PPP/2RQ1R1K",
        "w - - bm",
        "Nf6+",
    ),
    (
        "3r2k1/p2r1p1p/1p2p1p1/q4n2/3P4/PQ5P/1P1RNPP1/3R2K1",
        "b - - bm",
        "Nxd4",
    ),
    (
        "3r2k1/1p3ppp/2pq4/p1n5/P6P/1P6/1PB2QP1/1K2R3",
        "w - - am",
        "Rd1",
    ),
    (
        "r1b1r1k1/1ppn1p1p/3pnqp1/8/p1P1P3/5P2/PbNQNBPP/1R2RB1K",
        "w - - bm",
        "Rxb2",
    ),
    (
        "2r4k/pB4bp/1p4p1/6q1/1P1n4/2N5/P4PPP/2R1Q1K1",
        "b - - bm",
        "Qxc1",
    ),
    (
        "r5k1/3n1ppp/1p6/3p1p2/3P1B2/r3P2P/PR3PP1/2R3K1",
        "b - - am",
        "Rxa2",
    ),
    (
        "2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1",
        "b - - bm",
        "Bxe4",
    ),
    ("5r1k/6pp/1n2Q3/4p3/8/7P/PP4PK/R1B1q3", "b - - bm", "h6"),
    (
        "r3k2r/pbn2ppp/8/1P1pP3/P1qP4/5B2/3Q1PPP/R3K2R",
        "w KQkq - bm",
        "Be2",
    ),
    (
        "3r2k1/ppq2pp1/4p2p/3n3P/3N2P1/2P5/PP2QP2/K2R4",
        "b - - bm",
        "Nxc3",
    ),
    ("q3rn1k/2QR4/pp2pp2/8/P1P5/1P4N1/6n1/6K1", "w - - bm", "Nf5"),
    ("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1", "b - - bm", "Rd6"),
    ("1r4k1/7p/5np1/3p3n/8/2NB4/7P/3N1RK1", "w - - bm", "Nxd5"),
    (
        "1r2r1k1/p4p1p/6pB/q7/8/3Q2P1/PbP2PKP/1R3R2",
        "w - - bm",
        "Rxb2",
    ),
    (
        "r2q1r1k/pb3p1p/2n1p2Q/5p2/8/3B2N1/PP3PPP/R3R1K1",
        "w - - bm",
        "Bxf5",
    ),
    ("8/4p3/p2p4/2pP4/2P1P3/1P4k1/1P1K4/8", "w - - bm", "b4"),
    (
        "1r1q1rk1/p1p2pbp/2pp1np1/6B1/4P3/2NQ4/PPP2PPP/3R1RK1",
        "w - - bm",
        "e5",
    ),
    (
        "q4rk1/1n1Qbppp/2p5/1p2p3/1P2P3/2P4P/6P1/2B1NRK1",
        "b - - bm",
        "Qc8",
    ),
    (
        "r2q1r1k/1b1nN2p/pp3pp1/8/Q7/PP5P/1BP2RPN/7K",
        "w - - bm",
        "Qxd7",
    ),
    ("8/5p2/pk2p3/4P2p/2b1pP1P/P3P2B/8/7K", "w - - bm", "Bg4"),
    ("8/2k5/4p3/1nb2p2/2K5/8/6B1/8", "w - - bm", "Kxb5"),
    ("1B1b4/7K/1p6/1k6/8/8/8/8", "w - - bm", "Ba7"),
    (
        "rn1q1rk1/1b2bppp/1pn1p3/p2pP3/3P4/P2BBN1P/1P1N1PP1/R2Q1RK1",
        "b - - bm",
        "Ba6",
    ),
    ("8/p1ppk1p1/2n2p2/8/4B3/2P1KPP1/1P5P/8", "w - - bm", "Bxc6"),
    ("8/3nk3/3pp3/1B6/8/3PPP2/4K3/8", "w - - bm", "Bxd7"),
];

fn play(
    players: HashMap<bool, bool>,
    verbose: bool,
    search_threshold: usize,
    max_depth: usize,
    half_moves: isize,
    library_bypass: bool,
) {
    //let board = fen2board(KAUFMAN[23].0);
    //let mut game = Game::new(board);
    //game.log.push(NULL_MOVE); // dummy to force black

    let mut game = Game::new(ROOT_BOARD);
    println!("{}", game);

    let mut tot = 0;
    let mut moves = game.legal_moves();

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
            let lmoves = library_moves(game.hash);
            if !library_bypass && !lmoves.is_empty() {
                if verbose {
                    println!("#library moves from {}: {}", game.hash, lmoves.len());
                };
                let i = random::<usize>() % lmoves.len();
                let (frm, to) = lmoves[i];
                if let Some(m) = moves.iter().find(|m| (m.frm, m.to) == (frm, to)) {
                    println!("Library Move {} ", m);
                    vec![(*m, 0i32)]
                } else {
                    panic!("Not a valid library move")
                }
            } else {
                game.colour = game.turn();
                game.score_moves(&moves, search_threshold, max_depth, verbose)
                // game.n_searched = 0;
                // game.pvs(max_depth, 1, -INFINITE, INFINITE);
                // let (score, best) = moves
                //     .iter()
                //     .enumerate()
                //     .map(|(i, m)| {
                //         let key = game.hash ^ m.hash ^ WHITE_HASH;
                //         if let Some(e) = game.ttable.get(&key) {
                //             println!("{}/{}: {} {}/{}", i, moves.len(), m, e.m.val, -e.score);
                //             (-e.score, m)
                //         } else {
                //             println!("{}", m);
                //             panic!("move not in tt")
                //         }
                //     })
                //     .max_by(|&x, &y| y.0.cmp(&x.0))
                //     .unwrap();
                // vec![(*best, score)]
            }
        };
        for (i, (m, score)) in l.iter().enumerate() {
            println!("{}/{}: {} {}/{}", i, moves.len(), m, m.val, score);
        }
        // println!(
        //     "Searched: {}, Best: {} {} ",
        //     game.n_searched, l[0].0, l[0].1
        // );

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
                // let key = game.hash ^ m.hash ^ WHITE_HASH;
                // if let Some(e) = game.ttable.get(&key) {
                //     println!("tt {}", e.score);
                // }
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
            if game.rep_count() > 1 {
                println!("REP: {}", game.rep_count());
            }
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
    play(players, args.v, args.n, args.d, args.m, args.l);
}
