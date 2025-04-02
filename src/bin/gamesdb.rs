// Copyright (c) 2024 Jesper Olsen
// License: MIT, see License.txt
//
// Process .pgn format games extracted from the FICS Games Database:
// https://www.ficsgames.org/download.html

use clap::Parser;
use csv::Writer;
use flate2::read::GzDecoder;
use mateus::Game;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long = "elo", default_value_t = 2000)]
    ///minimum elo for both players
    e: usize,
    /// Input files
    #[arg(required = true)]
    files: Vec<String>,
}

#[derive(Debug)]
struct FicsG {
    event: String,
    site: String,
    game_no: usize,
    white: String,
    black: String,
    white_elo: usize,
    black_elo: usize,
    white_rd: String,
    black_rd: String,
    black_is_comp: String,
    time_control: String,
    date: String,
    time: String,
    white_clock: String,
    black_clock: String,
    eco: String,
    ply_count: usize,
    result: &'static str,
    moves: Vec<(usize, usize)>,
    fens: Vec<Vec<u8>>,
    comment: &'static str,
}

impl FicsG {
    pub fn new() -> Self {
        FicsG {
            event: String::new(),
            site: String::new(),
            game_no: 0,
            white: String::new(),
            black: String::new(),
            white_elo: 0,
            black_elo: 0,
            white_rd: String::new(),
            black_rd: String::new(),
            black_is_comp: String::new(),
            time_control: String::new(),
            date: String::new(),
            time: String::new(),
            white_clock: String::new(),
            black_clock: String::new(),
            eco: String::new(),
            ply_count: 0,
            result: "",
            comment: "",
            moves: Vec::new(),
            fens: Vec::new(),
        }
    }
}

const BLACK_CHECKMATED: &str = "Black checkmated";
const BLACK_FORFEITS_BY_DISCONNECTION: &str = "Black forfeits by disconnection";
const BLACK_FORFEITS_ON_TIME: &str = "Black forfeits on time";
const BLACK_RAN_OUT_OF_TIME_WHITE_NO_MATERIAL: &str =
    "Black ran out of time and White has no material to mate";
const BLACK_RESIGNS: &str = "Black resigns";
const BLACK_WINS_BY_ADJUDICATION: &str = "Black wins by adjudication";
const GAME_DRAWN_BY_ADJUDICATION: &str = "Game drawn by adjudication";
const GAME_DRAWN_BY_MUTUAL_AGREEMENT: &str = "Game drawn by mutual agreement";
const GAME_DRAWN_BY_REPETITION: &str = "Game drawn by repetition";
const GAME_DRAWN_BY_STALEMATE: &str = "Game drawn by stalemate";
const GAME_DRAWN_BY_50_MOVE_RULE: &str = "Game drawn by the 50 move rule";
const NEITHER_PLAYER_HAS_MATING_MATERIAL: &str = "Neither player has mating material";
const WHITE_CHECKMATED: &str = "White checkmated";
const WHITE_FORFEITS_BY_DISCONNECTION: &str = "White forfeits by disconnection";
const WHITE_FORFEITS_ON_TIME: &str = "White forfeits on time";
const WHITE_RAN_OUT_OF_TIME_BLACK_NO_MATERIAL: &str =
    "White ran out of time and Black has no material to mate";
const WHITE_RESIGNS: &str = "White resigns";
const WHITE_WINS_BY_ADJUDICATION: &str = "White wins by adjudication";

fn conclusive(s: &str) -> bool {
    match s {
        BLACK_CHECKMATED => true,
        BLACK_FORFEITS_BY_DISCONNECTION => false,
        BLACK_FORFEITS_ON_TIME => false,
        BLACK_RAN_OUT_OF_TIME_WHITE_NO_MATERIAL => false,
        BLACK_RESIGNS => true,
        BLACK_WINS_BY_ADJUDICATION => true,
        GAME_DRAWN_BY_ADJUDICATION => true,
        GAME_DRAWN_BY_MUTUAL_AGREEMENT => true,
        GAME_DRAWN_BY_REPETITION => true,
        GAME_DRAWN_BY_STALEMATE => true,
        GAME_DRAWN_BY_50_MOVE_RULE => true,
        NEITHER_PLAYER_HAS_MATING_MATERIAL => true,
        WHITE_CHECKMATED => true,
        WHITE_FORFEITS_BY_DISCONNECTION => false,
        WHITE_FORFEITS_ON_TIME => false,
        WHITE_RAN_OUT_OF_TIME_BLACK_NO_MATERIAL => false,
        WHITE_RESIGNS => true,
        WHITE_WINS_BY_ADJUDICATION => true,
        _ => false,
    }
}

fn static_outcome(s: &str) -> &'static str {
    match s {
        "0-1" => "0-1",
        "1-0" => "1-0",
        "1/2-1/2" => "1/2-1/2",
        _ => "",
    }
}

fn static_comment(s: &str) -> &'static str {
    match s {
        BLACK_CHECKMATED => BLACK_CHECKMATED,
        BLACK_FORFEITS_BY_DISCONNECTION => BLACK_FORFEITS_BY_DISCONNECTION,
        BLACK_FORFEITS_ON_TIME => BLACK_FORFEITS_ON_TIME,
        BLACK_RAN_OUT_OF_TIME_WHITE_NO_MATERIAL => BLACK_RAN_OUT_OF_TIME_WHITE_NO_MATERIAL,
        BLACK_RESIGNS => BLACK_RESIGNS,
        BLACK_WINS_BY_ADJUDICATION => BLACK_WINS_BY_ADJUDICATION,
        GAME_DRAWN_BY_ADJUDICATION => GAME_DRAWN_BY_ADJUDICATION,
        GAME_DRAWN_BY_MUTUAL_AGREEMENT => GAME_DRAWN_BY_MUTUAL_AGREEMENT,
        GAME_DRAWN_BY_REPETITION => GAME_DRAWN_BY_REPETITION,
        GAME_DRAWN_BY_STALEMATE => GAME_DRAWN_BY_STALEMATE,
        GAME_DRAWN_BY_50_MOVE_RULE => GAME_DRAWN_BY_50_MOVE_RULE,
        NEITHER_PLAYER_HAS_MATING_MATERIAL => NEITHER_PLAYER_HAS_MATING_MATERIAL,
        WHITE_CHECKMATED => WHITE_CHECKMATED,
        WHITE_FORFEITS_BY_DISCONNECTION => WHITE_FORFEITS_BY_DISCONNECTION,
        WHITE_FORFEITS_ON_TIME => WHITE_FORFEITS_ON_TIME,
        WHITE_RAN_OUT_OF_TIME_BLACK_NO_MATERIAL => WHITE_RAN_OUT_OF_TIME_BLACK_NO_MATERIAL,
        WHITE_RESIGNS => WHITE_RESIGNS,
        WHITE_WINS_BY_ADJUDICATION => WHITE_WINS_BY_ADJUDICATION,
        _ => "",
    }
}

fn parse_moves(fg: &mut FicsG, line: String) -> (Vec<(usize, usize)>, Vec<Vec<u8>>, &'static str) {
    let result = "000"; // dummy
    let mut lmoves = Vec::new();
    let mut lfens = Vec::new();
    let mut game = Game::default();

    let re_move_number = Regex::new(r"^\d+\.$").unwrap();
    let re_comment = Regex::new(r"\{([^}]*)\}").unwrap();

    let mut comments = Vec::new();
    for caps in re_comment.captures_iter(line.as_str()) {
        comments.push(caps[1].to_string());
    }

    if comments.len() == 1 {
        let s = comments[0].as_str();
        if !conclusive(s) {
            print!("{s}");
            return (lmoves, lfens, result);
        }
        fg.comment = static_comment(s);
    } else if comments.len() > 1 {
        println!("Multiple comments: {line}");
        return (lmoves, lfens, result);
    }

    let line = re_comment.replace_all(line.as_str(), "").to_string();

    //println!("Comments: {comments:?}");
    //println!("CLine: {line}");

    let mut last_move;
    let mut last = None;
    for s in line.split_whitespace() {
        if re_move_number.is_match(s) || re_comment.is_match(s) {
            continue;
        }
        if matches!(s, "0-1" | "1-0" | "1/2-1/2") {
            return (lmoves, lfens, static_outcome(s));
        }

        let moves = game.legal_moves(last);
        let alg_labels: Vec<_> = moves.iter().map(|m| game.move2label(m, &moves)).collect();
        match alg_labels.iter().position(|r| r == s) {
            Some(index) => {
                game.make_move(moves[index]);
                lmoves.push((moves[index].frm(), moves[index].to()));
                lfens.push(game.board.to_csv());
                last_move = moves[index];
                last = Some(&last_move);
            }
            None => {
                println!("Logged moves: {line}");
                println!("{game}");
                println!("Computed moves: {:?}", alg_labels);
                print!("Logged move '{}' not valid?", s);
                break;
            }
        }
    }
    (lmoves, lfens, result)
}

fn read_games(fname: &str, min_elo: usize) -> io::Result<Vec<FicsG>> {
    //let input = File::open(fname)?;
    //let buffered = io::BufReader::new(input);
    let input = File::open(fname).expect("Open file");
    let buffered = BufReader::new(input);
    let decoder = GzDecoder::new(buffered);
    let buffered = BufReader::new(decoder);

    let mut games = Vec::new();
    let mut current_game = FicsG::new();
    let mut in_moves_section = false;

    for line in buffered.lines() {
        let line = line?;
        if line.starts_with('[') {
            let parts: Vec<&str> = line[1..line.len() - 1].splitn(2, ' ').collect();
            let key = parts[0];
            let value = parts[1].trim_matches('"');
            match key {
                "Event" => current_game.event = value.to_string(),
                "Site" => current_game.site = value.to_string(),
                "FICSGamesDBGameNo" => {
                    current_game.game_no = value.parse::<usize>().expect("expected usize")
                }
                "White" => current_game.white = value.to_string(),
                "Black" => current_game.black = value.to_string(),
                "WhiteElo" => {
                    current_game.white_elo = value.parse::<usize>().expect("expected usize")
                }
                "BlackElo" => {
                    current_game.black_elo = value.parse::<usize>().expect("expected usize")
                }
                "WhiteRD" => current_game.white_rd = value.to_string(),
                "BlackRD" => current_game.black_rd = value.to_string(),
                "BlackIsComp" => current_game.black_is_comp = value.to_string(),
                "TimeControl" => current_game.time_control = value.to_string(),
                "Date" => current_game.date = value.to_string(),
                "Time" => current_game.time = value.to_string(),
                "WhiteClock" => current_game.white_clock = value.to_string(),
                "BlackClock" => current_game.black_clock = value.to_string(),
                "ECO" => current_game.eco = value.to_string(),
                "PlyCount" => {
                    current_game.ply_count = value.parse::<usize>().expect("expected usize")
                }
                "Result" => {
                    current_game.result = match value {
                        "0-1" => "0-1",
                        "1-0" => "1-0",
                        "1/2-1/2" => "1/2-1/2",
                        _ => panic!("Unexpected game Result"),
                    }
                }
                _ => {}
            }
        } else if !line.is_empty() && !line.starts_with(' ') {
            in_moves_section = true;
            let (moves, fens, res) = parse_moves(&mut current_game, line);
            if res == "000" {
                println!("; Ignoring game no {};", current_game.game_no);
            } else if res != current_game.result {
                println!("Result mismatch in game no {}", current_game.game_no);
            } else {
                current_game.moves.extend(moves);
                current_game.fens.extend(fens);
            }
        } else if line.is_empty() && in_moves_section {
            if matches!(current_game.result, "1-0" | "0-1")
                && current_game.white_elo >= min_elo
                && current_game.black_elo >= min_elo
            {
                // add final outcome to positions
                let z = match current_game.result {
                    "0-1" => 0,
                    "1-0" => 1,
                    "1/2-1/2" => 2,
                    _ => panic!("Unexpected game Result"),
                };
                for v in &mut current_game.fens {
                    v.push(z);
                }
                games.push(current_game);
            }
            current_game = FicsG::new();
            in_moves_section = false;
        }
    }

    Ok(games)
}

fn summarise_games(games: &[FicsG]) {
    println!("#games: {}", games.len());

    let welo = games.iter().map(|g| g.white_elo).sum::<usize>() as f32 / games.len() as f32;
    let belo = games.iter().map(|g| g.black_elo).sum::<usize>() as f32 / games.len() as f32;
    println!("Average white ELO: {welo}");
    println!("Average black ELO: {belo}");

    let mut acounts = HashMap::new();
    let mut rcounts = HashMap::new();
    let mut pcounts = HashMap::new();
    games.iter().for_each(|g| {
        let counter = acounts.entry(g.comment).or_insert(0);
        *counter += 1;
        let counter = rcounts.entry(g.result).or_insert(0);
        *counter += 1;
        let counter = pcounts.entry(g.white.as_str()).or_insert(0);
        *counter += 1;
        let counter = pcounts.entry(g.black.as_str()).or_insert(0);
        *counter += 1
    });

    let do_output = |title: &str, map: &HashMap<&str, i32>| {
        println!("{title}");
        let mut sorted_counts: Vec<(&&str, &i32)> = map.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

        for (i, (key, value)) in sorted_counts.into_iter().enumerate() {
            println!("{i}) {}: {}", key, value);
        }
    };

    do_output("\nAnnotations:", &acounts);
    do_output("\nOutcomes:", &rcounts);
    do_output("\nPlayers:", &pcounts);
}

fn write_games(fname: &str, l: &[FicsG]) -> Result<()> {
    println!("output to {}", fname);
    let f = File::create(fname)?;
    let mut wtr = Writer::from_writer(f);
    for g in l {
        for vec in &g.fens {
            let row: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
            wtr.write_record(&row)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let mut games = Vec::new();
    for fname in args.files {
        println!("Processing {fname}");
        match read_games(fname.as_str(), args.e) {
            Err(m) => println!("error: {m}"),
            Ok(l) => {
                let path = Path::new(&fname);
                let mut new_path = PathBuf::from(path);
                new_path.set_extension("csv");
                let fname2 = new_path.to_str().unwrap();
                match write_games(fname2, &l) {
                    Ok(()) => (),
                    Err(m) => println!("Failed to write games: {m}"),
                }
                games.extend(l)
            }
        }
    }

    summarise_games(&games);
}
