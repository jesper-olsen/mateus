pub mod bitmaps;
pub mod hashkeys;
pub mod hashkeys_generated;
use crate::hashkeys_generated::WHITE_HASH;
pub mod mgen;
pub mod misc;
pub mod openings;
pub mod val;
use core::cmp::max;
use core::cmp::min;
use hashkeys::*;
use mgen::*;
use std::collections::hash_map::{Entry, HashMap};
use std::fmt;
use val::*;

pub const INFINITE: i32 = 10000;

#[derive(Debug, Copy, Clone, PartialEq)]
enum BType {
    Exact,
    Lower,
    Upper,
}

#[derive(Debug, Copy, Clone)]
pub struct TTable {
    pub depth: usize,
    pub score: i32,
    pub m: Move,
    bound: BType,
}

#[derive(Debug)]
pub struct Game {
    pub log: Vec<Move>,
    pub board: [Piece; 64],
    pub n_searched: usize,
    material: i32,
    rep: HashMap<u64, usize>,
    pub ttable: HashMap<u64, TTable>,
    can_castle: Vec<[bool; 4]>, // white short, long, black short, long
    end_game: bool,
    pub hash: u64,
    bm_white: u64,
    bm_black: u64,
    bm_pawns: u64,
    bm_wking: u64,
    bm_bking: u64,
    log_bms: Vec<(u64, u64, u64, u64, u64)>,
    pub colour: bool,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in (0..8).rev() {
            write!(f, "{} ", y + 1)?;
            for x in 0..8 {
                write!(f, "{}", self.board[(7 - x) * 8 + y])?;
            }
            writeln!(f)?;
        }
        write!(f, "  ABCDEFGH")
    }
}

// fn print_moves(moves: &Vec<Move>) {
//     for (i, m) in moves.iter().enumerate() {
//         println!("{}/{}: {} {}", i, moves.len(), m, m.val);
//     }
// }

fn move_to_head(moves: &mut Vec<Move>, k: &Move) {
    if let Some(q) = moves.iter().position(|m| (m.frm, m.to) == (k.frm, k.to)) {
        if q != 0 {
            let m = moves.remove(q);
            //let m = moves.swap_remove(q);
            //println!("Move {} to head {}->0", m, q);
            moves.insert(0, m);
        }
    }
}

impl Game {
    pub fn new(board: [Piece; 64]) -> Self {
        let key = board2hash(&board, WHITE);
        Game {
            board,
            log: vec![],
            n_searched: 0,
            material: material(&board),
            rep: HashMap::from([(key, 1)]),
            ttable: HashMap::new(),
            can_castle: vec![[true; 4]],
            end_game: false,
            hash: key,
            bm_white: board2bm_colour(&board, WHITE),
            bm_black: board2bm_colour(&board, BLACK),
            bm_pawns: board2bm_white_pawns(&board) | board2bm_black_pawns(&board),
            log_bms: vec![],
            bm_wking: 0,
            bm_bking: 0,
            colour: WHITE,
        }
    }

    pub fn rep_len(&self) -> usize {
        self.rep.len()
    }

    pub fn ttable_len(&self) -> usize {
        self.ttable.len()
    }

    fn is_quiescent(&self) -> bool {
        if let Some(m) = self.log.last() {
            // quiescent unless last move was pawn near promotion
            self.board[m.to].ptype != PType::Pawn || (m.to % 8 != 6 && m.to % 8 != 1)
        } else {
            true
        }
    }

    fn ttable_clear(&mut self) {
        let key = self.hash;
        if self.ttable.contains_key(&key) {
            self.ttable = HashMap::from([(key, self.ttable[&key])]);
        } else {
            self.ttable = HashMap::new();
        }
    }

    fn rep_clear(&mut self) {
        let key = board2hash(&self.board, self.turn());
        self.rep = HashMap::from([(key, 1)]);
    }

    fn rep_inc(&mut self) {
        //*self.rep.entry(self.hash).or_default() += 1;
        self.rep
            .entry(self.hash)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    fn rep_dec(&mut self) {
        if let Entry::Occupied(entry) = self
            .rep
            .entry(self.hash)
            .and_modify(|x| *x = x.saturating_sub(1))
        {
            if *entry.get() == 0 {
                self.rep.remove(&self.hash);
            }
        }

        // self.rep
        //     .entry(self.hash)
        //     .and_modify(|x| *x = x.saturating_sub(1));
        // if let Some(0) = self.rep.get(&self.hash) {
        //     self.rep.remove(&self.hash);
        // }

        // self.rep
        //     .entry(self.hash)
        //     .and_modify(|x| *x = if *x == 0 { 0 } else { *x - 1 });
        // if let Some(count) = self.rep.get(&self.hash) {
        //     if *count == 0 {
        //         self.rep.remove(&self.hash);
        //     }
        // }
    }

    pub fn rep_count(&self) -> usize {
        if let Some(count) = self.rep.get(&self.hash) {
            *count
        } else {
            0
        }
    }

    pub fn make_move(&mut self, m: Move) {
        if m.kill.0 != NIL || [P1, P2].contains(&self.board[m.frm]) {
            self.rep_clear(); // ireversible move
        }
        self.ttable_clear();
        self.update(&m);

        //adjust king value in end game
        self.end_game = abs_material(&self.board) < END_GAME_MATERIAL;

        //update castling permissions
        let n = self.can_castle.len() - 1;
        if m.castle {
            println!("Castle!!");
            let i = if self.board[m.to] == K1 { 0 } else { 2 };
            for x in self.can_castle[n][i..=i + 1].iter_mut() {
                *x = false;
            }
        }
        // disable castling if rook moves
        for (i, frm) in [(0, 0), (1, 56), (2, 7), (3, 63)] {
            if self.can_castle[n][i] && m.frm == frm {
                self.can_castle[n][i] = false;
            }
        }
    }

    pub fn check_50_move_rule(&self) -> bool {
        self.rep.iter().map(|(_, &v)| v).sum::<usize>() >= 100
    }

    pub fn in_check(&self, colour: bool) -> bool {
        // true if other side can capture king

        in_check(
            &self.board,
            colour,
            self.bm_wking,
            self.bm_bking,
            self.bm_white | self.bm_black,
        )

        // let l = moves(
        //     &self.board,
        //     !colour,
        //     self.end_game,
        //     self.can_castle.last().unwrap(),
        //     self.log.last(),
        //     self.bm_white,
        //     self.bm_black,
        // );
        // self.n_searched += l.len(); // linked to stop criterion
        // l.iter().find(|&m| m.kill.0.ptype == PType::King).is_some()
    }

    fn legal_move(&mut self, m: &Move) -> bool {
        // verify move does not expose own king
        let colour = self.board[m.frm].colour;
        self.update(m);
        let flag = self.in_check(colour);
        self.backdate();
        !flag
    }

    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = self.moves(self.turn());
        moves.retain(|m| self.legal_move(m));
        moves
    }

    fn moves(&mut self, colour: bool) -> Vec<Move> {
        let mut l = moves(
            &self.board,
            colour,
            self.end_game,
            self.can_castle.last().unwrap(),
            self.log.last(),
            self.bm_white,
            self.bm_black,
        );
        l.sort_by(|b, a| a.val.cmp(&b.val));
        self.n_searched += l.len();
        l
    }

    pub fn turn(&self) -> bool {
        if self.log.len() % 2 == 0 {
            WHITE
        } else {
            BLACK
        }
    }

    pub fn update(&mut self, m: &Move) {
        self.log_bms.push((
            self.bm_pawns,
            self.bm_white,
            self.bm_black,
            self.bm_wking,
            self.bm_bking,
        ));
        self.log.push(*m);
        if m.castle {
            let mut cc = *self.can_castle.last().unwrap();
            let i = if self.board[m.frm] == K1 { 0 } else { 2 };
            for x in cc[i..=i + 1].iter_mut() {
                *x = false;
            }
            self.can_castle.push(cc);

            let (x, y) = if m.to <= 15 {
                (m.frm - 24, m.frm - 8) // short
            } else {
                (m.frm + 32, m.frm + 8) // long
            };
            self.board[y] = self.board[x];
            self.board[x] = NIL;
        }
        self.board[m.kill.1] = NIL; // enpassant
        if m.transform.1 != NIL {
            self.board[m.to] = m.transform.1;
        } else {
            self.board[m.to] = self.board[m.frm];
        }
        self.board[m.frm] = NIL;
        self.material += m.val;
        self.rep_inc();
        self.hash ^= m.hash ^ WHITE_HASH;

        self.bm_pawns = 0;
        self.bm_white = 0;
        self.bm_black = 0;
        for i in 0..64 {
            match self.board[i] {
                P1 => {
                    self.bm_pawns |= 1 << i;
                    self.bm_white |= 1 << i;
                }
                P2 => {
                    self.bm_pawns |= 1 << i;
                    self.bm_black |= 1 << i;
                }
                R1 | N1 | B1 | Q1 => self.bm_white |= 1 << i,
                R2 | N2 | B2 | Q2 => self.bm_black |= 1 << i,
                K1 => {
                    self.bm_white |= 1 << i;
                    self.bm_wking = 1 << i
                }
                K2 => {
                    self.bm_black |= 1 << i;
                    self.bm_bking = 1 << i
                }
                _ => (),
            }
        }
    }

    pub fn backdate(&mut self) {
        if let Some(bms) = self.log_bms.pop() {
            (
                self.bm_pawns,
                self.bm_white,
                self.bm_black,
                self.bm_wking,
                self.bm_bking,
            ) = bms;
        }
        if let Some(m) = self.log.pop() {
            self.hash ^= m.hash ^ WHITE_HASH;
            self.rep_dec();
            if m.castle {
                self.can_castle.pop();
                let (frm, to) = if m.to <= 15 {
                    (m.frm - 24, m.frm - 8) // short
                } else {
                    (m.frm + 32, m.frm + 8) // long
                };
                self.board[frm] = self.board[to];
                self.board[to] = NIL;
            }
            if m.transform.0 != NIL {
                self.board[m.frm] = m.transform.0;
            } else {
                self.board[m.frm] = self.board[m.to];
            }
            self.board[m.to] = NIL;

            if m.kill.0 != NIL {
                self.board[m.kill.1] = m.kill.0;
            }
            self.material -= m.val;
        }
    }

    fn ttstore(&mut self, depth: usize, score: i32, alpha: i32, beta: i32, m: &Move) {
        // TODO - implement more efficient hashing function
        let key = self.hash;
        let e = TTable {
            depth,
            score,
            bound: if score <= alpha {
                BType::Upper
            } else if score >= beta {
                BType::Lower
            } else {
                BType::Exact
            },
            m: *m,
        };
        self.ttable
            .entry(key)
            .and_modify(|x| {
                if x.depth < e.depth {
                    *x = e;
                }
            })
            .or_insert(e);
    }

    pub fn mobility(&self) -> i32 {
        count_moves(&self.board, WHITE, self.bm_white, self.bm_black) as i32
            - count_moves(&self.board, BLACK, self.bm_white, self.bm_black) as i32
    }

    pub fn eval(&self, colour: bool) -> i32 {
        let s = self.material + self.score_pawn_structure() + self.mobility();
        if colour {
            s
        } else {
            -s
        }
    }

    pub fn score_pawn_structure(&self) -> i32 {
        let mut pen: i32 = 0;
        let bm: [u64; 2] = [self.bm_pawns & self.bm_white, self.bm_pawns & self.bm_black];
        for (i, &p) in [P1, P2].iter().enumerate() {
            let nfiles = (0..8)
                .filter(|&q| 0b11111111 << (q * 8) & bm[i] > 0)
                .count() as i32;
            let npawns = bm[i].count_ones() as i32;
            let double_pawns = npawns - nfiles;

            let l = (0..8)
                .map(|q| (0b11111111 << (q * 8)) & bm[i] > 0)
                .collect::<Vec<bool>>();
            let isolated_pawns = (0..8)
                .filter(|&q| {
                    (q == 0 && l[0] && !l[1])
                        || (q > 0 && q < 7 && l[q] && !l[q - 1] && !l[q + 1])
                        || (q == 7 && l[7] && !l[6])
                })
                .count() as i32;

            let x = 20 * double_pawns + 4 * isolated_pawns;
            pen += if p == P1 { -x } else { x };
        }

        // passed pawn bonus
        for i in 0..8 {
            let file: u64 = 0b11111111 << (i * 8);
            let w = file & bm[0];
            let b = file & bm[1];
            if w > 0 && w > b {
                let k = 63 - w.leading_zeros();
                let q = (k % 8) as i32;
                pen += 2 * q * q;
            }
            if b > 0 && (w == 0 || b < w) {
                let k = b.trailing_zeros();
                let q = (7 - k % 8) as i32;
                pen -= 2 * q * q;
            }
        }

        pen
    }

    fn quiescence_fab(
        &mut self,
        ply: usize,
        alpha: i32,
        beta: i32,
        last_to: usize,
        quiescent: bool,
        rfab: bool,
    ) -> i32 {
        let colour = self.turn();

        let mut bscore = -INFINITE + ply as i32;
        let mut first = true;
        for m in self
            .moves(colour)
            .iter()
            .filter(|m| !quiescent || m.kill.0 != NIL)
            .filter(|m| !rfab || m.kill.1 == last_to)
        {
            self.update(m);
            if !self.in_check(colour) {
                // legal move
                first = false;
                let quiescent = if quiescent { true } else { self.is_quiescent() };
                let rfab = quiescent;
                let score = -self.quiescence_fab(
                    ply + 1,
                    -beta,
                    -max(alpha, bscore),
                    m.to,
                    quiescent,
                    rfab,
                );
                if score > bscore {
                    bscore = score;
                    if bscore >= beta {
                        self.backdate();
                        return bscore;
                    }
                }
            }
            self.backdate();
        }
        if first {
            self.eval(colour)
            // if colour {
            //     self.eval()
            // } else {
            //     -self.eval()
            // }
        } else {
            bscore
        }
    } // fn quiescence fab

    pub fn pvs(&mut self, dpt: usize, ply: usize, alp: i32, bet: i32) -> i32 {
        if self.rep_count() >= 2 {
            return 0;
        }

        let mut alpha = alp;
        let mut beta = bet;
        let mut bscore = -INFINITE + ply as i32;
        let mut bmove = None;
        let colour = self.turn();

        let depth = if self.in_check(colour) { dpt + 1 } else { dpt };

        let mut kmove = None;
        let key = self.hash;
        if let Some(e) = self.ttable.get(&key) {
            if e.depth >= depth {
                if e.bound == BType::Exact {
                    return e.score;
                } else if e.bound == BType::Lower {
                    alpha = max(alpha, e.score);
                } else if e.bound == BType::Upper {
                    beta = min(beta, e.score);
                }
                if alpha >= beta {
                    return e.score;
                }
            }
            kmove = Some(e.m);
        }

        if depth == 0 {
            let to = self.log.last().unwrap().to;
            return self.quiescence_fab(ply, alpha, beta, to, self.is_quiescent(), false);
            //return self.eval(colour);
        }

        let mut moves = self.moves(colour);
        if let Some(k) = kmove {
            move_to_head(&mut moves, &k);
        }
        for m in moves.iter() {
            self.update(m);
            if !self.in_check(colour) {
                // legal move
                if bmove.is_none() {
                    bscore = -self.pvs(depth - 1, ply + 1, -beta, -alpha); // full beam
                    bmove = Some(m);
                } else {
                    let mut score = -self.pvs(depth - 1, ply + 1, -alpha - 1, -max(bscore, alpha));
                    if score > bscore {
                        if score > max(bscore, alpha) && score < beta && depth > 2 {
                            score = -self.pvs(depth - 1, ply + 1, -beta, -score);
                        }
                        bscore = score;
                        bmove = Some(m);
                    }
                }
            }
            self.backdate();
            if let Some(m) = bmove {
                if bscore >= beta {
                    self.ttstore(depth, bscore, alp, beta, m);
                    return bscore;
                }
            }
        }
        if let Some(m) = bmove {
            self.ttstore(depth, bscore, alp, beta, m);
        }

        if bmove.is_none() && !self.in_check(colour) {
            0
        } else {
            bscore
        }
    }

    pub fn score_moves(
        &mut self,
        moves: &Vec<Move>,
        max_searched: usize,
        max_depth: usize,
        verbose: bool,
    ) -> Vec<(Move, i32)> {
        // top level pvs - does iterative deepening, sorts moves

        if moves.is_empty() {
            return vec![];
        }

        self.n_searched = 0;
        let mut pq0: Vec<(Move, i32)> = moves.iter().map(|m| (*m, 0)).collect();
        for depth in (2..=max_depth).step_by(1) {
            if depth > 1 && self.n_searched > max_searched {
                break;
            }
            let mut pq: Vec<(Move, i32)> = Vec::new();
            let mut alpha = -INFINITE;
            let beta = INFINITE;
            let mut bscore = alpha;

            for (i, (m, _v)) in pq0.iter().enumerate() {
                self.update(m);
                alpha = max(bscore, alpha);
                let mut score = if i == 0 {
                    // full beam
                    -self.pvs(depth - 1, 1, -beta, -alpha)
                } else {
                    -self.pvs(depth - 1, 1, -alpha - 1, -alpha)
                };

                if score > bscore {
                    if score > alpha && score < beta && depth > 2 {
                        score = -self.pvs(depth - 1, 1, -beta, -score);
                    }
                    bscore = score;
                }
                self.backdate();
                pq.push((*m, score));
            }
            pq.sort_by(|b, a| a.1.cmp(&b.1));
            pq0 = pq;
            if verbose {
                println!(
                    "Depth {:>2} #searched {:>8} bmove: {} bscore: {}",
                    depth, self.n_searched, pq0[0].0, bscore
                );
            }
            if !pq0.is_empty() && pq0[0].1.abs() >= INFINITE - depth as i32 {
                break;
            }
        }
        pq0
    } // fn score_moves
}
