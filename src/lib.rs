pub mod benchmark;
pub mod bitmaps;
pub mod hashkeys_generated;
pub mod mgen;
pub mod misc;
pub mod openings;
pub mod transposition;
pub mod val;

use crate::Colour;
use core::cmp::{max, min};
use mgen::*;
use std::fmt;
use transposition::Transpositions;
use val::*;
use val::{BPAWN, WPAWN};

const INFINITE: i16 = 32000;

pub struct Game {
    pub board: Board,
    pub n_searched: usize,
    pub ttable: Transpositions,
    end_game: bool,
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

// struct MovePicker {
//     moves: Vec<Move>,
// }

// impl MovePicker {
//     fn new(board: &Board, kmove: Option<(u8, u8)>) -> Self {
//         MovePicker { moves: vec![] }
//     }
// }

fn move_to_head(moves: &mut Vec<Move>, frmto: &(u8, u8)) {
    if let Some(q) = moves
        .iter()
        .position(|m| (m.frm(), m.to()) == (frmto.0, frmto.1))
    {
        if q != 0 {
            let m = moves.remove(q);
            //let m = moves.swap_remove(q);
            //println!("Move {} to head {}->0", m, q);
            moves.insert(0, m);
        }
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
        if m.castle() {
            if m.to() < 31 {
                label.push_str("O-O");
            } else {
                label.push_str("O-O-O");
            }
        } else {
            if self.board[m.frm() as usize].kind() == PAWN {
                if self.board[m.to() as usize] != EMPTY || m.en_passant() {
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
            if m.en_passant() || self.board[m.to() as usize] != EMPTY {
                label.push('x');
            }
            label.push_str(I2SQ[m.to() as usize]);
            if m.transform() {
                label.push_str(m.promote_label())
            }
        }

        self.board.update(m);
        let in_check = self.in_check(self.turn());
        if self.legal_moves().is_empty() && in_check {
            label.push('#')
        } else if self.in_check(self.turn()) {
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
        if m.en_passant()
            || self.board[m.to() as usize] != EMPTY
            || self.board[m.frm() as usize].kind() == PAWN
        {
            self.board.rep.clear(); // ireversible move
            self.board.half_move_clock = 0;
        }
        self.ttable.clear();
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

    pub fn in_check(&self, colour: Colour) -> bool {
        // true if other side can capture king
        self.board.in_check(colour)
    }

    fn legal_move(&mut self, m: &Move) -> bool {
        // verify move does not expose own king
        self.board.update(m);
        let flag = self.board.in_check(self.board.colour.opposite());
        self.board.backdate(m);
        !flag
    }

    pub fn legal_moves(&mut self) -> Vec<Move> {
        let in_check = self.in_check(self.board.colour);
        let mut moves = self.moves(in_check);
        moves.retain(|m| self.legal_move(m));
        moves
    }

    fn moves(&mut self, in_check: bool) -> Vec<Move> {
        let mut l = self.board.moves(in_check, self.end_game);
        if self.board.colour.is_white() {
            //l.sort_by(|b, a| a.val.cmp(&b.val)); // decreasing
            l.sort_unstable_by(|b, a| a.val.cmp(&b.val)); // decreasing
        } else {
            //l.sort_by(|a, b| a.val.cmp(&b.val)); // increasing
            l.sort_unstable_by(|a, b| a.val.cmp(&b.val)); // increasing
        }
        self.n_searched += l.len();
        l
    }

    pub fn turn(&self) -> Colour {
        self.board.colour
    }

    fn quiescence_fab(&mut self, alp: i16, beta: i16, last: &Move, rfab: bool) -> i16 {
        let colour = self.board.colour;

        let mut bscore = None;
        let mut alpha = alp;
        let in_check = false; // TODO - calculate?
        let mut moves = self.moves(in_check);
        moves.retain(|m|
            //let ic = self.in_check(colour);
            if rfab {
                m.to() == last.to()
            } else {
                m.en_passant() || self.board[m.to() as usize] != EMPTY
            }
        );
        for m in moves {
            self.board.update(&m);
            if !self.in_check(colour) {
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

    pub fn pvs(&mut self, depth: u16, ply: usize, alpha: i16, beta: i16, last: &Move) -> i16 {
        if let Some(count) = self.board.rep.get(&self.board.hash) {
            if *count >= 2 {
                return 0;
            }
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let mut bscore = -INFINITE + ply as i16;
        let mut bmove = None;
        let colour = self.board.colour;

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
            Some(e.frmto())
        } else {
            None
        };

        let in_check = self.in_check(colour);
        let depth = match (depth, in_check) {
            (_, true) => depth + 1,
            (0, false) if self.is_quiescent(last) => {
                return self.quiescence_fab(alpha, beta, last, false);
            }
            (0, false) => 1,
            (_, false) => depth,
        };

        let mut moves = self.moves(in_check);
        if let Some(k) = kmove {
            move_to_head(&mut moves, &k);
        }
        for m in moves.iter() {
            self.board.update(m);
            if !self.in_check(colour) {
                // legal move
                if bmove.is_none() {
                    bscore = -self.pvs(depth - 1, ply + 1, -beta, -alpha, m); // full beam
                    bmove = Some(m);
                } else {
                    let mut score = -self.pvs(
                        depth - 1,
                        ply + 1,
                        -max(alpha, bscore) - 1,
                        -max(alpha, bscore),
                        m,
                    );
                    if score > bscore {
                        if score > max(bscore, alpha) && score < beta && depth > 2 {
                            score = -self.pvs(depth - 1, ply + 1, -beta, -score, m);
                        }
                        bscore = score;
                        bmove = Some(m);
                    }
                }
            }
            self.board.backdate(m);
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
        max_searched: usize,
        verbose: bool,
    ) -> Vec<(Move, i16)> {
        // top level pvs - does iterative deepening, sorts moves
        // note that only the best move has exact scoring...

        if moves.is_empty() {
            return vec![];
        }

        self.n_searched = 0;
        let mut pq0: Vec<(Move, i16)> = moves.iter().map(|m| (*m, 0)).collect();
        for depth in (2..).step_by(1) {
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
                    "Depth {:>2} #searched {:>8} bmove: {} bscore: {}",
                    depth, self.n_searched, pq0[0].0, bscore
                );
            }
            if self.n_searched > max_searched || pq0[0].1.abs() >= INFINITE - 1000 {
                break;
            }
        }
        pq0
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
