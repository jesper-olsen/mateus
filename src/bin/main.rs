// Copyright (c) 2022 Jesper Olsen
// License: MIT, see License.txt
//
// Puccinia's Checkmate - small chess engine implemented in rust

use ::std::time::Instant;
use clap::Parser;
use mateus::Game;
use mateus::benchmark;
use mateus::mgen::{Board, Move};
use mateus::misc::str2move;
use mateus::openings::library_moves;
use mateus::val::*;
use rand::random;
use std::collections::hash_map::HashMap;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1000000)]
    ///break off search threshold - positions generated
    n: usize,
    #[arg(short, long, default_value_t = -1)]
    ///number of moves before stopping
    m: isize,
    #[arg(short, long, default_value_t = false)]
    ///play white (human-computer)
    w: bool,
    #[arg(short, long, default_value_t = false)]
    ///play black (human-computer)
    b: bool,
    #[arg(short, long, default_value_t = false)]
    ///no opening library
    l: bool,
    #[arg(short, long, default_value_t = 0)]
    ///benchmark test sets - Bratko-Kopec (1) / Kaufman (2) / Lasker (3) / Nolot (4) / CCR (5) / ERET (6) / BT-2450 (7) / BT-2630 (8)
    k: usize,
    #[arg(short, long, default_value_t = false)]
    ///verbose output
    v: bool,
    #[arg(short, long, default_value_t = String::from(ROOT_FEN))]
    ///fen board - start position
    f: String,
}

fn pick_move(game: &mut Game, moves: &[Move]) -> Vec<(Move, i16)> {
    let label = if game.board.turn.is_white() {
        "White"
    } else {
        "Black"
    };
    loop {
        println!("Your Move ({label}):");
        let s = get_input();
        match s.as_str() {
            "q" => std::process::exit(1),
            "m" => {
                print!("Moves: ");
                for m in moves {
                    print!("{} ", game.move2label(m, moves))
                }
                println!();
            }
            _ => {
                let Some((frm, to)) = str2move(s.as_str()) else {
                    println!("Not valid");
                    continue;
                };
                let l: Vec<_> = moves
                    .iter()
                    .filter(|m| (m.frm(), m.to()) == (frm, to))
                    .collect();
                match l.len() {
                    0 => println!("Not legal"),
                    1 => return vec![(*l[0], 0)],
                    _ => {
                        let mut n;
                        let label = format!("pick a number [0-{}]", l.len() - 1);
                        loop {
                            for (i, m) in l.iter().enumerate() {
                                println!("[{i}] Move: {m}");
                            }
                            n = get_number::<usize>(label.as_str());
                            if n < l.len() {
                                break;
                            }
                        }
                        return vec![(*l[n], 0)];
                    }
                }
            }
        }
    }
}

fn check_game_over(game: &Game, moves: &[Move], half_moves: isize) -> String {
    if game.board.rep_count() >= 3 {
        "1/2-1/2 Draw by repetition".to_string()
    } else if game.board.half_moves() >= 100 {
        "1/2-1/2 Draw by the 50-move rule".to_string()
    } else if half_moves != -1 && half_moves <= game.board.full_move_count as isize {
        format!("stopping after {} move(s)", game.board.full_move_count)
    } else if moves.is_empty() {
        if game.board.in_check(game.board.turn) {
            if game.board.turn.is_white() {
                "0-1"
            } else {
                "1-0"
            }
        } else {
            "1/2-1/2 Draw"
        }
        .to_string()
    } else {
        "".to_string()
    }
}

fn benchmark(verbose: bool, search_threshold: usize, tname: &str, tpos: &[(&str, &str)]) {
    println!("{} Test - search threshold: {search_threshold}", tname);
    let mut correct: Vec<usize> = vec![];
    let mut points: f64 = 0.0;
    let mut n_searched: usize = 0;
    let start = Instant::now();
    for (i, (fen, label)) in tpos.iter().enumerate() {
        let Ok(board) = Board::from_fen(fen) else {
            println!("Bad fen: {fen}");
            continue;
        };
        let mut game = Game::new(board);
        let moves = game.board.legal_moves();
        game.n_searched = 0;

        let l = game.score_moves(&moves, search_threshold, verbose);
        let (best, score) = l[0];
        n_searched += game.n_searched;
        let clabel = game.move2label(&best, &moves);
        let colour = if game.board.turn.is_white() {
            "white"
        } else {
            "black"
        };
        println!("{game}");

        for (i, (m, score)) in l.iter().enumerate() {
            if verbose {
                println!("{i}/{}: {m} {}/{score}", l.len(), m.val);
            }
            let clabel = game.move2label(m, &moves);
            if i < 4 && (*label).contains(clabel.as_str()) {
                // note - only the best move is accurately scored (pruning)
                points += match i {
                    0 => 1.0,
                    1 => 0.5,
                    2 => 0.25,
                    3 => 0.33,
                    _ => 0.0,
                }
            }
        }
        println!(
            "Position {:>2}; Searched: {:>9}, Score: {score:>5 }, Move ({colour}): {} = {clabel:>4 }; Expected: {label}\n",
            i + 1,
            game.n_searched,
            best
        );
        if (*label).contains(clabel.as_str()) {
            //if clabel.as_str() == *label {
            correct.push(i + 1);
        }
        println!("Correct: {:?} {}/{}", correct, correct.len(), tpos.len());
        println!("Points: {points}");

        let dur = (Instant::now() - start).as_millis();
        println!("Time: {dur} ms => {} ms/position", dur / (i + 1) as u128);
        let speed = if let Some(speed) = (n_searched as u128).checked_div(dur) {
            speed as usize
        } else {
            0
        };
        println!(
            "Search total: {n_searched:}; Time {} ms => {speed:} nodes/ms ",
            (Instant::now() - start).as_millis() as usize,
        );
    }
}

fn play(
    players: HashMap<Colour, bool>,
    verbose: bool,
    search_threshold: usize,
    half_moves: isize,
    library_bypass: bool,
    fen: &str,
) -> Result<(), String> {
    let mut game = Game::new(Board::from_fen(fen)?);

    let mut tot = 0;
    let mut moves = game.board.legal_moves();

    let start = Instant::now();
    loop {
        let msg = check_game_over(&game, &moves, half_moves);
        if !msg.is_empty() {
            println!("{}", msg);
            std::process::exit(1);
        }

        let l = if players[&game.board.turn] {
            pick_move(&mut game, &moves)
        } else {
            // try library 1st - compute if not there
            let lmoves = library_moves(game.board.hash);
            if !library_bypass && !lmoves.is_empty() {
                if verbose {
                    println!("#library moves from {}: {}", game.board.hash, lmoves.len());
                    println!("{:?}", lmoves);
                };
                let i = random::<u32>() % lmoves.len() as u32;
                let (frm, to) = lmoves[i as usize];
                if let Some(m) = moves.iter().find(|m| (m.frm(), m.to()) == (frm, to)) {
                    println!("Library Move {} ", m);
                    vec![(*m, 0i16)]
                } else {
                    panic!("Not a valid library move")
                }
            } else {
                game.score_moves(&moves, search_threshold, verbose)
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
            println!(
                "hash size r {} t {} ",
                game.board.rep.len(),
                game.ttable.len(),
            );
            for (i, (m, score)) in l.iter().enumerate() {
                println!("{}/{}: {} {}/{}", i, moves.len(), m, m.val, score);
            }
        }
        let (m, score) = l[0];

        let label = game.move2label(&m, &moves);
        game.make_move(m);
        println!("{game}");
        moves = game.board.legal_moves();
        println!("{}. {label}", game.board.move_number());

        if verbose {
            if game.board.rep_count() > 1 {
                println!("REP: {}", game.board.rep_count());
            }
            println!(
                "score: {}, material: {}, is_end_game: {}, pawns: {}, mobility: {}",
                score,
                game.board.material,
                game.board.is_end_game(),
                game.board.score_pawn_structure(),
                game.board.mobility()
            );
        }
    }
}

pub fn get_number<T: std::str::FromStr>(msg: &str) -> T {
    loop {
        println!("{msg}");
        let input: String = get_input();
        if let Ok(num) = input.trim().parse::<T>() {
            return num;
        } else {
            println!("Invalid input. Please enter a valid number.");
        };
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    String::from(s.trim())
}

fn main() {
    let args = Args::parse();

    if args.k > 0 {
        match args.k {
            1 => benchmark(args.v, args.n, "Bratko-Kopec", &benchmark::BRATKO_KOPEC),
            2 => benchmark(args.v, args.n, "Kaufman", &benchmark::KAUFMAN),
            3 => benchmark(args.v, args.n, "Lasker", &benchmark::LASKER),
            4 => benchmark(args.v, args.n, "Nolot", &benchmark::NOLOT),
            5 => benchmark(args.v, args.n, "CCR One Hour", &benchmark::CCR),
            6 => benchmark(
                args.v,
                args.n,
                "Eigenmann Rapid Engine Test",
                &benchmark::ERET,
            ),
            7 => benchmark(args.v, args.n, "BT-2450", &benchmark::BT2450),
            _ => benchmark(args.v, args.n, "BT-2630", &benchmark::BT2630),
        }
    } else {
        let players = HashMap::from([(Colour::white(), args.w), (Colour::black(), args.b)]);
        if let Err(m) = play(players, args.v, args.n, args.m, args.l, args.f.as_str()) {
            println!("Bad fen: {m}");
        }
    }
}
