// Copyright (c) 2022 Jesper Olsen
// License: MIT, see License.txt
//
// Puccinia's Checkmate - small chess engine implemented in rust

use ::std::time::Instant;
use clap::Parser;
use puccinia_s_checkmate::benchmark;
use puccinia_s_checkmate::mgen::*;
use puccinia_s_checkmate::misc::str2move;
use puccinia_s_checkmate::openings::library_moves;
use puccinia_s_checkmate::val::Piece::*;
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
    #[arg(short, long, default_value_t = 30)]
    ///max depth of regular search  
    d: u16,
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
    ///no opening library
    l: bool,
    #[arg(short, long, default_value_t = 0)]
    ///benchmark test positions - Bratko-Kopec (1) / Kaufman (2) / Lasker (3)
    k: usize,
    #[arg(short, long, default_value_t = false)]
    ///verbose output
    v: bool,
    #[arg(short, long, default_value_t = String::from(ROOT_FEN))]
    ///fen board - start position
    f: String,
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
    if m.castle() {
        if m.to() < 31 {
            label.push_str("0-0 ");
        } else {
            label.push_str("0-0-0 ");
        }
    } else if m.transform() {
        label.push('*');
    }
    if !matches!(board[m.frm()], Pawn(_)) {
        let p = format!("{}", board[m.frm()]).to_uppercase();
        label.push_str(&p);
    }

    let mut l = Vec::new();
    for m2 in moves {
        match (board[m.frm()], board[m2.frm()]) {
            (Rook(_), Rook(_)) | (Knight(_), Knight(_)) | (Bishop(_), Bishop(_))
                if m2.to() == m.to() =>
            {
                l.push(i2str(m2.to()))
            }
            _ => (),
        }
    }
    if l.len() > 1 {
        // push file - or rank if same
        let n = if l[0] == l[1] { 1 } else { 0 };
        label.push(l[0].chars().nth(n).unwrap());
    }
    if m.en_passant() || board[m.to()] != Nil {
        // TODO ?
        //if board[m.frm()].ptype == PType::Pawn {}
        label.push('x');
    }
    label.push_str(&i2str(m.to()));
    label
}

fn pick_move(moves: &[Move], label: &str) -> Vec<(Move, i16)> {
    loop {
        println!("Your Move ({label}):");
        let s = get_input();
        if s.as_str() == "q" {
            std::process::exit(1);
        } else {
            let (frm, to) = str2move(s.as_str());
            if let Some(m) = moves.iter().find(|m| (m.frm(), m.to()) == (frm, to)) {
                return vec![(*m, 0)];
            } else {
                println!("Not valid");
            }
        }
    }
}

fn check_game_over(game: &Game, moves: &[Move], half_moves: isize, log: &[Move]) -> String {
    if game.rep_count() >= 3 {
        "1/2-1/2 Draw by repetition".to_string()
    } else if game.check_50_move_rule() {
        "1/2-1/2 Draw by the 50-move rule".to_string()
    } else if half_moves != -1 && half_moves <= log.len() as isize {
        format!("stopping after {} half move(s)", log.len())
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

fn benchmark(
    verbose: bool,
    search_threshold: usize,
    max_depth: u16,
    tname: &str,
    tpos: &[(&str, &str, &str)],
) {
    println!(
        "{} Test - search threshold: {search_threshold}, max depth: {max_depth}",
        tname
    );
    let mut correct: Vec<usize> = vec![];
    let mut points: f64 = 0.0;
    let mut n_searched: usize = 0;
    let start = Instant::now();
    for (i, (fen, h, label)) in tpos.iter().enumerate() {
        let mut game = Game::from_fen(fen);
        let cc = game.can_castle.last_mut().unwrap();
        cc[0] = h.contains('K');
        cc[1] = h.contains('Q');
        cc[2] = h.contains('k');
        cc[3] = h.contains('q');

        if h.starts_with('b') {
            game.colour = BLACK;
        }
        let moves = game.legal_moves(None);
        game.n_searched = 0;

        let l = game.score_moves(&moves, search_threshold, max_depth, verbose);
        let (best, score) = l[0];
        n_searched += game.n_searched;
        let clabel = move2label(&game.board, &best, &moves);
        let colour = if game.colour { "white" } else { "black" };
        println!("{game}");

        for (i, (m, score)) in l.iter().enumerate() {
            if verbose {
                println!("{i}/{}: dpt {max_depth} {m} {}/{score}", l.len(), m.val);
            }
            let clabel = move2label(&game.board, m, &moves);
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
            i+1,
            game.n_searched, best);
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
    players: HashMap<bool, bool>,
    verbose: bool,
    search_threshold: usize,
    max_depth: u16,
    half_moves: isize,
    library_bypass: bool,
    fen: &str,
) {
    let mut game = Game::from_fen(fen);
    println!("{}", game);

    let mut tot = 0;
    let mut moves = game.legal_moves(None);
    let mut log = Vec::new();

    let start = Instant::now();
    loop {
        let msg = check_game_over(&game, &moves, half_moves, &log);
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
                    println!("{:?}", lmoves);
                };
                let i = random::<usize>() % lmoves.len();
                let (frm, to) = lmoves[i];
                if let Some(m) = moves.iter().find(|m| (m.frm(), m.to()) == (frm, to)) {
                    println!("Library Move {} ", m);
                    vec![(*m, 0i16)]
                } else {
                    panic!("Not a valid library move")
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
        log.push(m);
        println!("{game}");
        moves = game.legal_moves(Some(&m));
        let s = match (game.in_check(game.turn()), moves.is_empty()) {
            (true, true) => "#",
            (true, false) => "+",
            (false, _) => "",
        };
        println!("{}. {}{}", log.len() / 2 + 1, label, s);

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

    if args.k > 0 {
        match args.k {
            1 => benchmark(
                args.v,
                args.n,
                args.d,
                "Bratko-Kopec",
                &benchmark::BRATKO_KOPEC,
            ),
            2 => benchmark(args.v, args.n, args.d, "Kaufman", &benchmark::KAUFMAN),
            _ => benchmark(args.v, args.n, args.d, "Lasker", &benchmark::LASKER),
        }
    } else {
        let players = HashMap::from([(WHITE, args.w), (BLACK, args.b)]);
        play(
            players,
            args.v,
            args.n,
            args.d,
            args.m,
            args.l,
            args.f.as_str(),
        );
    }
}
