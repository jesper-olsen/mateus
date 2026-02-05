// UCI chess engine - to be used with a UCI GUI such as
// https://www.gnu.org/software/xboard/
//    $ brew install xboard
//    $ xboard -fcp target -fd . -fUCI
// http://www.pearlchess.com/
// http://www.cutechess.com/
// and others
// https://www.chessprogramming.org/UCI
//
// Also can be used to set up a lichess bot:
// https://github.com/lichess-bot-devs/lichess-bot
//
// Engine communicates with GUI via stdin and stdout
// https://www.wbec-ridderkerk.nl/html/UCIProtocol.html
//
// TODO: handle more commands, e.g. "stop", "go ponder"

use clap::Parser;
use mateus::{
    Game, SearchConstraints,
    mgen::{Board, Move},
    misc::str2move,
    openings::library_moves,
    val,
    val::I2SQ,
    val::ROOT_FEN,
};
use rand::random;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1000000)]
    ///break off search threshold - positions generated
    n: usize,
}

fn move2uci_string(m: &Move) -> String {
    let p = match m.promote_kind() {
        val::QUEEN => "q",
        val::ROOK => "r",
        val::BISHOP => "b",
        val::KNIGHT => "n",
        _ => "",
    };
    format!("{}{}{p}", I2SQ[m.frm() as usize], I2SQ[m.to() as usize])
}

fn handle_position(game: &mut Game, parts: &[&str]) {
    let mut i = 0;

    if parts.get(i) == Some(&"startpos") {
        *game = Game::new(Board::from_fen(ROOT_FEN).unwrap());
        i += 1;
    } else if parts.get(i) == Some(&"fen") {
        i += 1;
        let fen_start = i;
        // Collect FEN parts until "moves" or end
        while i < parts.len() && parts[i] != "moves" {
            i += 1;
        }
        let fen = parts[fen_start..i].join(" ");
        *game = Game::new(Board::from_fen(&fen).expect("Invalid FEN"));
    }

    if parts.get(i) == Some(&"moves") {
        i += 1;
    }

    for s in &parts[i..] {
        let Some((frm, to)) = str2move(s) else {
            continue;
        };
        let promo_char = s.chars().nth(4);

        // Find move in legal moves
        // TODO - trust that the move is legal...
        if let Some(mv) = game.board.legal_moves().iter().find(|m| {
            (m.frm(), m.to()) == (frm, to)
                && (promo_char.is_none() || m.promote_kind() == match_promo(promo_char.unwrap()))
        }) {
            game.make_move(*mv);
        }
    }
}

// Helper for promotion piece mapping
fn match_promo(c: char) -> u8 {
    match c {
        'q' => val::QUEEN,
        'r' => val::ROOK,
        'b' => val::BISHOP,
        'n' => val::KNIGHT,
        _ => 0,
    }
}

fn pick_library_move(lmoves: &[(u8, u8)]) -> Option<(u8, u8)> {
    if lmoves.is_empty() {
        return None;
    }
    let i = random::<u32>() % lmoves.len() as u32;
    Some(lmoves[i as usize])
}

fn handle_go(game: &mut Game, sc: &mut SearchConstraints, parts: &[&str], overhead: u64) {
    let mut wtime: Option<u64> = None;
    let mut btime: Option<u64> = None;
    let mut winc: Option<u64> = None;
    let mut binc: Option<u64> = None;
    let mut movestogo: Option<u64> = None;
    let mut movetime: Option<u64> = None;

    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            "depth" => {
                sc.depth = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "movetime" => {
                movetime = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "wtime" => {
                wtime = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "btime" => {
                btime = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "winc" => {
                winc = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "binc" => {
                binc = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            "movestogo" => {
                movestogo = parts.get(i + 1).and_then(|s| s.parse().ok());
                i += 2;
            }
            _ => i += 1,
        }
    }

    // Calculate time allocation
    let time_ms = if let Some(mt) = movetime {
        // Direct movetime specified
        mt
    } else {
        // Calculate from clock time
        let our_time = if game.board.turn.is_white() {
            wtime
        } else {
            btime
        };
        let our_inc = if game.board.turn.is_white() {
            winc
        } else {
            binc
        };

        if let Some(time) = our_time {
            // Simple time management algorithm
            if let Some(mtg) = movestogo {
                // X/Y time control: divide remaining time by moves to go
                // Use a bit less to avoid time trouble
                (time / mtg.max(1)) * 8 / 10 + our_inc.unwrap_or(0)
            } else {
                // Increment time control: assume ~30 moves remaining
                time / 30 + our_inc.unwrap_or(0)
            }
        } else {
            // No time info - use a default
            0
        }
    };

    if time_ms > 0 {
        // Subtract overhead, but don't go below 1ms
        let final_time = time_ms.saturating_sub(overhead).max(1);
        sc.time = Some(Duration::from_millis(final_time));
        //sc.time = Some(Duration::from_millis(time_ms));
    }

    let moves = game.board.legal_moves();
    if moves.is_empty() {
        println!("bestmove (none)");
        return;
    }

    let lmoves = library_moves(game.board.hash);
    println!("info string lib moves: {}", lmoves.len());

    let l = if let Some((frm, to)) = pick_library_move(lmoves) {
        if let Some(m) = moves.iter().find(|m| (m.frm(), m.to()) == (frm, to)) {
            println!("info string Library hit: {m}");
            vec![(*m, 0i16)]
        } else {
            game.score_moves(&moves, sc, false).0
        }
    } else {
        game.score_moves(&moves, sc, false).0
    };

    if let Some((best_move, _score)) = l.get(0) {
        println!("bestmove {}", move2uci_string(best_move));
    } else {
        println!("bestmove (none)");
    }
}

fn uci_loop(mut sc: SearchConstraints) {
    let mut game = Game::new(Board::from_fen(ROOT_FEN).unwrap());
    let mut input = String::new();
    let mut move_overhead = 10; // Default 10ms

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "uci" => {
                println!("id name Mateus");
                println!("id author Jesper Olsen");
                println!("option name Move Overhead type spin default 10 min 0 max 5000");
                println!("option name Threads type spin default 1 min 1 max 128");
                println!("option name Hash type spin default 16 min 1 max 1024");
                println!("option name SyzygyPath type string default <empty>");
                println!("option name UCI_ShowWDL type check default false");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {
                game = Game::new(Board::from_fen(ROOT_FEN).unwrap());
                game.ttable.clear();
            }
            "position" => {
                // Example: position startpos moves e2e4 e7e5
                handle_position(&mut game, &parts[1..]);
            }
            "go" => {
                // Example: go depth 10 or go movetime 5000
                handle_go(&mut game, &mut sc, &parts[1..], move_overhead);
            }
            "quit" => break,
            "d" | "print" => {
                println!("{}", game.board);
            }
            "setoption" => {
                // Simple parser for "setoption name Move Overhead value 100"
                if let Some(name_idx) = parts.iter().position(|&x| x == "name")
                    && let Some(val_idx) = parts.iter().position(|&x| x == "value")
                {
                    let option_name = parts[name_idx + 1..val_idx].join(" ").to_lowercase();
                    let option_value = parts.get(val_idx + 1);
                    match option_name.as_str() {
                        "move overhead" => {
                            if let Some(v) = option_value.and_then(|s| s.parse::<u64>().ok()) {
                                move_overhead = v;
                            }
                        }
                        "threads" => {
                            if let Some(v) = option_value.and_then(|s| s.parse::<usize>().ok()) {
                                println!(
                                    "info string Setting threads to {v} (not yet implemented, staying on 1)"
                                );
                            }
                        }
                        "hash" => {
                            // TODO handle TT size here (optional)
                        }
                        "syzygypath" => {
                            if let Some(path) = option_value {
                                println!("info string SyzygyPath set to {path} (not yet utilized)");
                            }
                        }
                        "uci_showwdl" => {
                            if let Some(val) = option_value {
                                println!(
                                    "info string UCI_ShowWDL set to {val} (not yet implemented)"
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args = Args::parse();
    let sc = SearchConstraints::default().nodes(args.n);
    uci_loop(sc)
}
