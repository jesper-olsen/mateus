pub mod benchmark;
pub mod bitmaps;
pub mod hashkeys_generated;
pub mod mgen;
pub mod misc;
pub mod openings;
pub mod val;
use crate::{Colour::*, Piece::*};
use core::cmp::{max, min};
use mgen::*;
use std::collections::hash_map::HashMap;
use std::fmt;
use val::*;

const INFINITE: i16 = 10000;
const EXACT_BIT: u16 = 1 << 12;
const LOWER_BIT: u16 = 1 << 13;
const UPPER_BIT: u16 = 1 << 14;

#[derive(Debug, Copy, Clone)]
pub struct TTable {
    depth: u16,
    score: i16,
    data: u16, // frm, to, bound: 2x6 bits + 3 bits
}

pub struct Game {
    pub board: Board,
    pub n_searched: usize,
    pub ttable: HashMap<u64, TTable>,
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

fn move_to_head(moves: &mut Vec<Move>, frmto: &(u8, u8)) {
    if let Some(q) = moves
        .iter()
        .position(|m| (m.frm(), m.to()) == (frmto.0 as usize, frmto.1 as usize))
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
        //println!("size of TTable {}", std::mem::size_of::<TTable>());
        Game {
            board,
            n_searched: 0,
            ttable: HashMap::new(),
            end_game: false,
        }
    }

    pub fn to_csv(&self) -> Vec<u8> {
        const CSV_SIZE: usize = 2 * 6 * 64 + 1 + 4 + 64 + 1;
        let mut v = Vec::with_capacity(CSV_SIZE);
        for p in [
            Pawn(White),
            Rook(White),
            Knight(White),
            Bishop(White),
            Queen(White),
            King(White),
            Pawn(Black),
            Rook(Black),
            Knight(Black),
            Bishop(Black),
            Queen(Black),
            King(Black),
        ] {
            for pb in &self.board {
                v.push(if p == *pb { 1 } else { 0 });
            }
        }

        //turn
        v.push(if self.turn().is_white() { 1 } else { 0 });

        // O-O O-O-O
        v.extend(
            [CASTLE_W_SHORT, CASTLE_W_LONG, CASTLE_B_SHORT, CASTLE_B_LONG]
                .map(|c| (self.board.can_castle & c != 0) as u8),
        );

        // en passant
        if let Some(last) = self.board.move_log.last() {
            if matches!(self.board[last.to()], Pawn(_)) && last.to().abs_diff(last.frm()) == 2 {
                let idx = last.to() as isize + if self.board.colour.is_white() { 1 } else { -1 };
                for i in 0..64 {
                    v.push((i == idx) as u8);
                }
            } else {
                v.resize(v.len() + 64, 0);
            }
        } else {
            v.resize(v.len() + 64, 0);
        };

        v
    }

    //https://cheatography.com/davechild/cheat-sheets/chess-algebraic-notation/
    pub fn move2label(&mut self, m: &Move, moves: &[Move]) -> String {
        fn i2xy(i: usize) -> (usize, usize) {
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
            if matches!(self.board[m.frm()], Pawn(_)) {
                if self.board[m.to()] != Nil || m.en_passant() {
                    label.push_str(&I2SQ[m.frm()][0..1])
                }
            } else {
                label.push_str(&self.board[m.frm()].to_string().to_uppercase());
            }

            // If two or more pieces of the same type can move to the same sq we need to disambiguate
            // by adding file if file is unique, otherwise row
            let mut nx = 0;
            let mut ny = 0;
            let mut n = 0;
            let (x0, y0) = i2xy(m.frm());
            for m2 in moves {
                match (self.board[m.frm()], self.board[m2.frm()]) {
                    (Rook(_), Rook(_))
                    | (Knight(_), Knight(_))
                    | (Bishop(_), Bishop(_))
                    | (Queen(_), Queen(_))
                        if m2.to() == m.to() =>
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
                    _ => (),
                }
            }
            if n > 1 {
                if nx > 1 && ny > 1 {
                    label.push_str(I2SQ[m.frm()])
                } else if nx <= ny {
                    label.push_str(&I2SQ[m.frm()][0..1])
                } else {
                    label.push_str(&I2SQ[m.frm()][1..2])
                }
            }
            if m.en_passant() || self.board[m.to()] != Nil {
                label.push('x');
            }
            label.push_str(I2SQ[m.to()]);
            if m.transform() {
                match m.ptransform(self.board.colour) {
                    Rook(_) => label.push_str("=R"),
                    Knight(_) => label.push_str("=N"),
                    Bishop(_) => label.push_str("=B"),
                    _ => label.push_str("=Q"),
                }
            }
        }

        self.board.update(m);
        let in_check = self.in_check(self.turn());
        if self.legal_moves(Some(m)).is_empty() && in_check {
            label.push('#')
        } else if self.in_check(self.turn()) {
            label.push('+')
        }
        self.board.backdate(m);
        label
    }

    pub fn rep_len(&self) -> usize {
        self.board.rep.len()
    }

    pub fn ttable_len(&self) -> usize {
        self.ttable.len()
    }

    fn is_quiescent(&self, last: &Move) -> bool {
        // quiescent unless last move was pawn near promotion
        // !self.in_check(self.colour) &&
        match self.board[last.to()] {
            Pawn(White) => last.to() % 8 != 6,
            Pawn(Black) => last.to() % 8 != 1,
            _ => true,
        }
    }

    fn ttable_clear(&mut self) {
        let key = self.board.hash;
        if self.ttable.contains_key(&key) {
            self.ttable = HashMap::from([(key, self.ttable[&key])]);
        } else {
            //self.ttable = HashMap::new();
            self.ttable.clear();
        }
    }

    fn rep_clear(&mut self) {
        self.board.rep.clear();
    }

    pub fn rep_count(&self) -> usize {
        if let Some(count) = self.board.rep.get(&self.board.hash) {
            *count
        } else {
            0
        }
    }

    pub fn make_move(&mut self, m: Move) {
        if m.en_passant() || self.board[m.to()] != Nil || matches!(self.board[m.frm()], Pawn(_)) {
            self.rep_clear(); // ireversible move
            self.board.half_move_clock = 0;
        }
        self.ttable_clear();
        self.board.update(&m);

        //adjust king value in end game
        self.end_game = self.board.is_end_game();
        self.board.move_log.push(m);

        //update castling permissions
        match (self.board[m.to()], m.frm()) {
            (King(White), 24) => self.board.can_castle &= !CASTLE_W_SHORT & !CASTLE_W_LONG,
            (King(Black), 31) => self.board.can_castle &= !CASTLE_B_SHORT & !CASTLE_B_LONG,
            (Rook(White), 0) => self.board.can_castle &= !CASTLE_W_SHORT,
            (Rook(White), 56) => self.board.can_castle &= !CASTLE_W_LONG,
            (Rook(Black), 7) => self.board.can_castle &= !CASTLE_B_SHORT,
            (Rook(Black), 63) => self.board.can_castle &= !CASTLE_B_LONG,
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

    pub fn legal_moves(&mut self, last: Option<&Move>) -> Vec<Move> {
        let in_check = self.in_check(self.board.colour);
        let mut moves = self.moves(self.board.colour, last, in_check);
        moves.retain(|m| self.legal_move(m));
        moves
    }

    fn moves(&mut self, colour: Colour, last: Option<&Move>, in_check: bool) -> Vec<Move> {
        let mut l = self
            .board
            .moves(in_check, self.end_game, self.board.can_castle, last);
        if colour.is_white() {
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

    fn ttstore(&mut self, depth: u16, score: i16, alpha: i16, beta: i16, m: &Move) {
        // TODO - implement more efficient hashing function
        let key = self.board.hash;
        let bound = if score <= alpha {
            UPPER_BIT
        } else if score >= beta {
            LOWER_BIT
        } else {
            EXACT_BIT
        };
        let data = (m.data & (mgen::FRM_MASK | mgen::TO_MASK)) | bound;
        let e = TTable { depth, score, data };
        self.ttable
            .entry(key)
            .and_modify(|x| {
                if x.depth < e.depth {
                    *x = e;
                }
            })
            .or_insert(e);
    }

    fn quiescence_fab(&mut self, alp: i16, beta: i16, last: &Move, rfab: bool) -> i16 {
        let colour = self.board.colour;

        let mut bscore = None;
        let mut alpha = alp;
        let in_check = false; // TODO - calculate?
        let mut moves = self.moves(colour, Some(last), in_check);
        moves.retain(|m|
            //let ic = self.in_check(colour);
            if rfab {
                m.to() == last.to()
            } else {
                m.en_passant() || self.board[m.to()] != Nil
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
        if self.rep_count() >= 2 {
            return 0;
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let mut bscore = -INFINITE + ply as i16;
        let mut bmove = None;
        let colour = self.board.colour;

        let kmove = if let Some(e) = self.ttable.get(&self.board.hash) {
            if e.depth >= depth {
                match e.data & (EXACT_BIT | UPPER_BIT | LOWER_BIT) {
                    EXACT_BIT => return e.score,
                    LOWER_BIT => alpha = max(alpha, e.score),
                    UPPER_BIT => beta = min(beta, e.score),
                    _ => unreachable!(),
                }
                if alpha >= beta {
                    return e.score;
                }
            }
            let frm = mgen::ext_frm(e.data);
            let to = mgen::ext_to(e.data);
            Some((frm, to))
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

        let mut moves = self.moves(colour, Some(last), in_check);
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
                self.ttstore(depth, bscore, alpha, beta, m);
                bscore
            }
        }
    }

    pub fn score_moves(
        &mut self,
        moves: &[Move],
        max_searched: usize,
        max_depth: u16,
        verbose: bool,
    ) -> Vec<(Move, i16)> {
        // top level pvs - does iterative deepening, sorts moves
        // note that only the best move has exact scoring...

        if moves.is_empty() {
            return vec![];
        }

        self.n_searched = 0;
        let mut pq0: Vec<(Move, i16)> = moves.iter().map(|m| (*m, 0)).collect();
        for depth in (2..=max_depth).step_by(1) {
            if depth > 1 && self.n_searched > max_searched {
                break;
            }
            let mut pq: Vec<(Move, i16)> = Vec::new();
            let mut alpha = -INFINITE;
            let beta = INFINITE;
            let mut bscore = alpha;

            for (i, (m, _v)) in pq0.iter().enumerate() {
                self.board.update(m);
                alpha = max(bscore, alpha);
                let mut score = if i == 0 {
                    // full beam
                    -self.pvs(depth - 1, 1, -beta, -alpha, m)
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
            if !pq0.is_empty() && pq0[0].1.abs() >= INFINITE - depth as i16 {
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
            game.to_fen().as_str(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        )
    }
}
