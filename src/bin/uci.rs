// UCI chess engine - to be used with a UCI GUI such as 
// http://www.pearlchess.com/
// http://www.cutechess.com/
// and others
// https://www.chessprogramming.org/UCI
//
// Engine communicates with GUI via stdin and stdout
// https://www.wbec-ridderkerk.nl/html/UCIProtocol.html
//

use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender};

fn main() {
    let stdin = io::stdin();
    let (tx, rx) = mpsc::channel();
    let is_searching = Arc::new(AtomicBool::new(false));
    let stop_search = Arc::new(AtomicBool::new(false));

    let tx_clone = tx.clone();
    thread::spawn(move || {
        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read line");
            handle_command(&line, tx_clone.clone(), Arc::clone(&is_searching), Arc::clone(&stop_search));
        }
    });

    while let Ok(message) = rx.recv() {
        println!("{}", message);
    }
}

fn handle_command(command: &str, tx: Sender<String>, is_searching: Arc<AtomicBool>, stop_search: Arc<AtomicBool>) {
    match command {
        "uci" => {
            tx.send("id name MyChessEngine".to_string()).unwrap();
            tx.send("id author YourName".to_string()).unwrap();
            // option
            tx.send("uciok".to_string()).unwrap();
        },
        "debug on" => {
        }
        "debug off" => {
        }
        "isready" => {
            tx.send("readyok".to_string()).unwrap();
        },
        cmd if cmd.starts_with("setoption name ") => {
        }
        "register later" => {
        }
        cmd if cmd.starts_with("register name") => {
        }
        "ucinewgame" => {
            // Initialize a new game
        },
        cmd if cmd.starts_with("position") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.len() > 1 && parts[1] == "startpos" {
                // Set the board to the starting position
            } else if parts.len() > 2 && parts[1] == "fen" {
                // Set the board to the specified FEN position
            }
            if let Some(moves_index) = parts.iter().position(|&x| x == "moves") {
                // Apply moves to the board
                for _mv in &parts[moves_index + 1..] {
                    // Apply each move
                }
            }
        },
        cmd if cmd.starts_with("go") => {
            if !is_searching.load(Ordering::SeqCst) {
                is_searching.store(true, Ordering::SeqCst);
                let tx_clone = tx.clone();
                let stop_search_clone = Arc::clone(&stop_search);
                let is_searching_clone = Arc::clone(&is_searching);
                thread::spawn(move || {
                    // Start searching for the best move
                    thread::sleep(Duration::from_secs(5));
                    if !stop_search_clone.load(Ordering::SeqCst) {
                        tx_clone.send("bestmove e2e4".to_string()).unwrap(); // Example response
                    }
                    is_searching_clone.store(false, Ordering::SeqCst);
                });
            }
        },
        "stop" => {
            stop_search.store(true, Ordering::SeqCst);
        },
        "ponderhit" => {
        },
        "quit" => {
            std::process::exit(0);
        },
        _ => {
            // unknown command
        }
    }
}

