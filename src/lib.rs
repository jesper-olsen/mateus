pub mod benchmark;
pub mod bitmaps;
pub mod hashkeys_generated;
pub mod mgen;
pub mod misc;
pub mod openings;
pub mod transposition;
pub mod val;

use core::cmp::{max, min};
use mgen::*;
use std::{fmt, time, time::Duration};
use transposition::Transpositions;
use val::*;

const INFINITE: i16 = 32000;

pub struct Game {
    pub board: Board,
    pub n_searched: usize,
    pub ttable: Transpositions,
    end_game: bool,
}

#[derive(Default)]
pub struct SearchInfo {
    pub depth: u8,
    pub nodes: usize,
    pub time: Duration,
}

#[derive(Default, Clone)]
pub struct SearchConstraints {
    pub depth: Option<u8>,
    pub nodes: Option<usize>,
    pub time: Option<Duration>,
}

impl SearchConstraints {
    pub fn depth(mut self, depth: u8) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn nodes(mut self, nodes: usize) -> Self {
        self.nodes = Some(nodes);
        self
    }

    pub fn time(mut self, time: Duration) -> Self {
        self.time = Some(time);
        self
    }

    pub fn time_millis(mut self, millis: u64) -> Self {
        self.time = Some(Duration::from_millis(millis));
        self
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new(Board::default())
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.board.to_fen())?;
        write!(f, "{}", self.board)
    }
}

impl Game {
    pub fn new(board: Board) -> Self {
        //println!("size of TEntry {}", std::mem::size_of::<TEntry>());
        Game {
            board,
            n_searched: 0,
            ttable: Transpositions::default(),
            end_game: false,
        }
    }

    //https://cheatography.com/davechild/cheat-sheets/chess-algebraic-notation/
    pub fn move2label(&mut self, m: &Move, moves: &[Move]) -> String {
        fn i2xy(i: u8) -> (u8, u8) {
            let x = 7 - i / 8; // col
            let y = i % 8; // row
            (x, y)
        }

        let mut label = String::new();
        //if m.castle() {
        if self.board.is_castle(m) {
            if m.to() < 31 {
                label.push_str("O-O");
            } else {
                label.push_str("O-O-O");
            }
        } else {
            if self.board[m.frm() as usize].kind() == PAWN {
                if self.board[m.to() as usize] != EMPTY || self.board.is_en_passant(m) {
                    label.push_str(&I2SQ[m.frm() as usize][0..1])
                }
            } else {
                label.push_str(&self.board[m.frm() as usize].to_string().to_uppercase());
            }

            // If two or more pieces of the same type can move to the same sq we need to disambiguate
            // by adding file if file is unique, otherwise row
            let mut nx = 0;
            let mut ny = 0;
            let mut n = 0;
            let (x0, y0) = i2xy(m.frm());
            for m2 in moves {
                if m2.to() == m.to()
                    && self.board[m.frm() as usize].is_officer()
                    && self.board[m.frm() as usize] == self.board[m2.frm() as usize]
                {
                    n += 1;
                    let (x, y) = i2xy(m2.frm());
                    if x == x0 {
                        nx += 1;
                    }
                    if y == y0 {
                        ny += 1;
                    }
                }
            }
            if n > 1 {
                if nx > 1 && ny > 1 {
                    label.push_str(I2SQ[m.frm() as usize])
                } else if nx <= ny {
                    label.push_str(&I2SQ[m.frm() as usize][0..1])
                } else {
                    label.push_str(&I2SQ[m.frm() as usize][1..2])
                }
            }
            if self.board.is_en_passant(m) || self.board[m.to() as usize] != EMPTY {
                label.push('x');
            }
            label.push_str(I2SQ[m.to() as usize]);
            if m.is_promote() {
                label.push_str(m.promote_label())
            }
        }

        self.board.update(m);
        let in_check = self.board.in_check(self.board.turn);
        if self.board.legal_moves().is_empty() && in_check {
            label.push('#')
        } else if self.board.in_check(self.board.turn) {
            label.push('+')
        }
        self.board.backdate(m);
        label
    }

    fn is_quiescent(&self, last: &Move) -> bool {
        // quiescent unless last move was pawn near promotion
        // !self.in_check(self.colour) &&
        match self.board[last.to() as usize] {
            WPAWN => last.to() % 8 != 6,
            BPAWN => last.to() % 8 != 1,
            _ => true,
        }
    }

    pub fn make_move(&mut self, m: Move) {
        if self.board.is_en_passant(&m)
            || self.board[m.to() as usize] != EMPTY
            || self.board[m.frm() as usize].kind() == PAWN
        {
            self.board.rep.clear(); // ireversible move
            self.board.half_move_clock = 0;
        }
        //self.ttable.clear();
        self.board.update(&m);

        //adjust king value in end game
        self.end_game = self.board.is_end_game();
        self.board.full_move_count += 1;

        //update castling permissions
        match (self.board[m.to() as usize], m.frm()) {
            (WKING, 24) => self.board.can_castle &= !CASTLE_W_SHORT & !CASTLE_W_LONG,
            (BKING, 31) => self.board.can_castle &= !CASTLE_B_SHORT & !CASTLE_B_LONG,
            (WROOK, 0) => self.board.can_castle &= !CASTLE_W_SHORT,
            (WROOK, 56) => self.board.can_castle &= !CASTLE_W_LONG,
            (BROOK, 7) => self.board.can_castle &= !CASTLE_B_SHORT,
            (BROOK, 63) => self.board.can_castle &= !CASTLE_B_LONG,
            _ => (),
        }
    }

    fn quiescence_fab(&mut self, alp: i16, beta: i16, last: &Move, rfab: bool) -> i16 {
        let colour = self.board.turn;

        let mut bscore = None;
        let mut alpha = alp;
        let in_check = false; // TODO - calculate?
        let mut moves = self.board.moves(in_check, self.end_game);
        if rfab {
            moves.retain(|m| self.board.is_en_passant(m) || m.to() == last.to())
        } else {
            moves.retain(|m| self.board.is_en_passant(m) || self.board[m.to() as usize] != EMPTY);
        }
        if self.board.turn.is_white() {
            moves.sort_unstable_by(|b, a| a.val.cmp(&b.val)); // decreasing
        } else {
            moves.sort_unstable_by(|a, b| a.val.cmp(&b.val)); // increasing
        }
        for m in moves {
            self.board.update(&m);
            if !self.board.in_check(colour) {
                // legal move
                let score = -self.quiescence_fab(-beta, -alpha, &m, true);
                match bscore {
                    Some(bs) if score <= bs => (),
                    _ => {
                        if score >= beta {
                            self.board.backdate(&m);
                            return score;
                        }
                        bscore = Some(score);
                        alpha = max(alpha, score);
                    }
                }
            }
            self.board.backdate(&m);
        }
        if let Some(bs) = bscore {
            bs
        } else {
            self.board.eval()
        }
    } // fn quiescence fab

    pub fn pvs(&mut self, depth: u8, ply: usize, alpha: i16, beta: i16, last: &Move) -> i16 {
        if let Some(count) = self.board.rep.get(&self.board.hash)
            && *count >= 2
        {
            return 0;
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let mut bscore = -INFINITE + ply as i16;
        let mut bmove = None;
        let colour = self.board.turn;

        let kmove = if let Some(e) = self.ttable.probe(self.board.hash) {
            if e.depth() >= depth {
                if e.exact_bound() {
                    return e.score();
                } else if e.lower_bound() {
                    alpha = max(alpha, e.score())
                } else {
                    beta = min(beta, e.score())
                }
                if alpha >= beta {
                    return e.score();
                }
            }
            let (frm, to) = e.frmto();
            Some(self.board.infer_move(frm, to))
        } else {
            None
        };

        let in_check = self.board.in_check(colour);
        let depth = match (depth, in_check) {
            (_, true) => depth + 1,
            (0, false) if self.is_quiescent(last) => {
                return self.quiescence_fab(alpha, beta, last, false);
            }
            (0, false) => 1,
            (_, false) => depth,
        };

        // let mut moves = self.board.moves(in_check, self.end_game);
        // if self.board.turn.is_white() {
        //     moves.sort_unstable_by(|a, b| a.val.cmp(&b.val)); // increasing
        // } else {
        //     moves.sort_unstable_by(|b, a| a.val.cmp(&b.val)); // decreasing
        // }
        // if let Some(k) = kmove {
        //     if let Some(q) = moves.iter().position(|&m| m == k) {
        //         if q != 0 {
        //             let m = moves.remove(q);
        //             //moves.insert(0, m);
        //             moves.push(m);
        //         }
        //     }
        // }

        let mut moves = Vec::new();
        if let Some(k) = kmove {
            moves.push(k);
        }
        let mut generated = false;
        loop {
            let m = match moves.pop() {
                Some(m) => m,
                None if !generated => {
                    generated = true;
                    moves = self.board.moves(in_check, self.end_game);
                    if let Some(k) = kmove
                        && let Some(q) = moves.iter().position(|&m| m == k)
                    {
                        moves.remove(q);
                    }
                    if self.board.turn.is_white() {
                        moves.sort_unstable_by(|a, b| a.val.cmp(&b.val)); // increasing
                    } else {
                        moves.sort_unstable_by(|b, a| a.val.cmp(&b.val)); // decreasing
                    }
                    continue;
                }
                None => break,
            };
            self.n_searched += 1;
            self.board.update(&m);
            if !self.board.in_check(colour) {
                // legal move
                if bmove.is_none() {
                    bscore = -self.pvs(depth - 1, ply + 1, -beta, -alpha, &m); // full beam
                    bmove = Some(m);
                } else {
                    let mut score = -self.pvs(
                        depth - 1,
                        ply + 1,
                        -max(alpha, bscore) - 1,
                        -max(alpha, bscore),
                        &m,
                    );
                    if score > bscore {
                        if score > max(bscore, alpha) && score < beta && depth > 2 {
                            score = -self.pvs(depth - 1, ply + 1, -beta, -score, &m);
                        }
                        bscore = score;
                        bmove = Some(m);
                    }
                }
            }
            self.board.backdate(&m);
            if bscore >= beta {
                break;
            }
        }

        match (bmove, in_check) {
            (None, false) => 0,
            (None, true) => bscore,
            (Some(m), _) => {
                self.ttable
                    .store(self.board.hash, depth, bscore, alpha, beta, m);
                bscore
            }
        }
    }

    pub fn score_moves(
        &mut self,
        moves: &[Move],
        sc: &SearchConstraints,
        verbose: bool,
    ) -> (Vec<(Move, i16)>, SearchInfo) {
        // top level pvs - does iterative deepening, sorts moves
        // note that only the best move has exact scoring...

        let mut search_info = SearchInfo::default();
        let start = time::Instant::now();

        if moves.is_empty() {
            return (vec![], search_info);
        }

        self.n_searched = 0;
        let mut pq0: Vec<(Move, i16)> = moves.iter().map(|m| (*m, 0)).collect();

        let mut last_depth_time = Duration::from_millis(0);

        if let Some(time_limit) = sc.time {
            println!("info time_limit={time_limit:?}");
        }
        for depth in (2..255).step_by(1) {
            if let Some(time_limit) = sc.time {
                let elapsed = start.elapsed();
                if elapsed > (time_limit * 6 / 10) || elapsed + last_depth_time * 3 >= time_limit {
                    break;
                }
            }

            let depth_start = time::Instant::now();
            let mut pq: Vec<(Move, i16)> = Vec::new();
            let mut alpha = -INFINITE;
            let beta = INFINITE;
            let mut bscore = alpha;

            for (i, (m, _v)) in pq0.iter().enumerate() {
                self.board.update(m);
                alpha = max(bscore, alpha);
                let mut score = if i == 0 {
                    -self.pvs(depth - 1, 1, -beta, -alpha, m) // full beam
                } else {
                    -self.pvs(depth - 1, 1, -alpha - 1, -alpha, m)
                };

                if score > bscore {
                    if score > alpha && score < beta && depth > 2 {
                        score = -self.pvs(depth - 1, 1, -beta, -score, m);
                    }
                    bscore = score;
                }
                self.board.backdate(m);
                pq.push((*m, score));
            }
            pq.sort_by(|b, a| a.1.cmp(&b.1)); // decreasing
            pq0 = pq;
            if verbose {
                println!(
                    "info Depth {depth:>2} #searched {:>8} bmove: {} bscore: {bscore}",
                    self.n_searched, pq0[0].0
                );
            }
            let elapsed = start.elapsed();
            search_info.nodes = self.n_searched;
            search_info.depth = depth;
            search_info.time = elapsed;
            if (sc.nodes.is_some() && self.n_searched >= sc.nodes.unwrap())
                || (sc.depth.is_some() && depth >= sc.depth.unwrap())
                || (sc.time.is_some() && elapsed >= sc.time.unwrap())
                || pq0[0].1.abs() >= INFINITE - 1000
            {
                break;
            }
            last_depth_time = depth_start.elapsed();
        }
        (pq0, search_info)
    } // fn score_moves
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fen() {
        let game = Game::new(Board::default());
        assert_eq!(
            game.board.to_fen().as_str(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        )
    }
}
