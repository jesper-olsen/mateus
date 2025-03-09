pub mod bitmaps;
pub mod hashkeys;
pub mod hashkeys_generated;
use crate::hashkeys_generated::WHITE_HASH;
pub mod benchmark;
pub mod mgen;
pub mod misc;
pub mod openings;
pub mod val;
use crate::Piece::*;
use core::cmp::max;
use core::cmp::min;
use hashkeys::*;
use mgen::*;
use std::collections::hash_map::{Entry, HashMap};
use std::fmt;
use val::*;

pub const INFINITE: i16 = 10000;

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
    pub board: [Piece; 64],
    pub colour: bool,
    pub n_searched: usize,
    material: i16,
    half_move_clock: usize, // since last irreversible move
    full_move_count: usize,
    rep: HashMap<u64, usize>,
    pub ttable: HashMap<u64, TTable>,
    pub can_castle: Vec<[bool; 4]>, // white short, long, black short, long
    pub move_log: Vec<Move>,
    end_game: bool,
    pub hash: u64,
    bm_white: u64,
    bm_black: u64,
    bm_pawns: u64,
    bm_wking: u64,
    bm_bking: u64,
    log_bms: Vec<(u64, u64, u64, u64, u64, Piece, u64)>,
}

impl fmt::Debug for Game {
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

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape codes for background and foreground colors
        let light_square_bg = "\x1b[48;5;229m"; // Light background
        let dark_square_bg = "\x1b[48;5;94m"; // Dark background
        //let light_square_bg = "\x1b[48;5;15m"; // White background
        //let dark_square_bg = "\x1b[48;5;8m";   // Gray background
        let black_fg = "\x1b[38;5;0m"; // Black foreground
        let white_fg = "\x1b[38;5;15m"; // White foreground
        let reset_colour = "\x1b[0m"; // Reset to default colour

        writeln!(f, "{}", self.to_fen())?;
        for y in (0..8).rev() {
            write!(f, "{} ", y + 1)?;
            for x in 0..8 {
                let i = (7 - x) * 8 + y;
                let ch = self.board[i].to_unicode();
                let fg = if self.board[i].is_white() {
                    white_fg
                } else {
                    black_fg
                };
                let is_light_square = (x + y) % 2 != 0;
                let background_color = if is_light_square {
                    light_square_bg
                } else {
                    dark_square_bg
                };
                let fg_colour = if is_light_square && fg == white_fg {
                    black_fg
                } else {
                    fg
                };
                write!(f, "{background_color}{fg_colour} {ch} {reset_colour}")?;
            }
            writeln!(f)?;
        }
        write!(f, "   A  B  C  D  E  F  G  H")
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
    pub fn new(board: [Piece; 64]) -> Self {
        //println!("size of TTable {}", std::mem::size_of::<TTable>());
        let key = board2hash(&board, WHITE);
        let (bm_white, bm_black) = board2bm(&board);
        Game {
            board,
            colour: WHITE,
            n_searched: 0,
            material: material(&board),
            half_move_clock: 0,
            full_move_count: 0,
            rep: HashMap::from([(key, 1)]),
            ttable: HashMap::new(),
            can_castle: vec![[true; 4]],
            move_log: Vec::new(),
            end_game: false,
            hash: key,
            bm_white,
            bm_black,
            bm_pawns: board2bm_pawns(&board),
            log_bms: vec![],
            bm_wking: 0,
            bm_bking: 0,
        }
    }

    const CSV_SIZE: usize = 2 * 6 * 64 + 1 + 4 + 64 + 1;
    pub fn to_csv(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(Self::CSV_SIZE);
        for p in PIECES {
            for pb in self.board {
                v.push(if p == pb { 1 } else { 0 });
            }
        }

        //turn
        v.push(if self.turn() == WHITE { 1 } else { 0 });

        // O-O O-O-O
        for c in self.can_castle.last().unwrap() {
            v.push(if *c { 1 } else { 0 });
        }

        // en passant
        if let Some(last) = self.move_log.last() {
            if matches!(self.board[last.to()], Pawn(_)) && last.to().abs_diff(last.frm()) == 2 {
                let idx = last.to() as isize + if self.colour { 1 } else { -1 };
                for i in 0..64 {
                    v.push(if i == idx { 1 } else { 0 });
                }
            } else {
                v.resize(v.len() + 64, 0);
            }
        } else {
            v.resize(v.len() + 64, 0);
        };

        v
    }

    pub fn to_fen(&self) -> String {
        let mut s = String::new();
        for y in (0..=7).rev() {
            let mut n = 0;
            for x in (0..=7).rev() {
                let idx = x * 8 + y;
                if self.board[idx] == Nil {
                    n += 1;
                } else {
                    if n > 0 {
                        s.push_str(format!("{}", n).as_str());
                        n = 0;
                    }
                    s.push(self.board[idx].to_ascii())
                }
            }
            if n > 0 {
                s.push_str(format!("{}", n).as_str())
            }

            if y != 0 {
                s.push('/')
            }
        }
        s.push_str(if self.turn() == WHITE { " w" } else { " b" });
        let cc = self.can_castle.last().unwrap();
        s.push(' ');
        if *cc == [false, false, false, false] {
            s.push('-');
        } else {
            if matches!(cc, [true, _, _, _]) {
                s.push('K');
            }
            if matches!(cc, [_, true, _, _]) {
                s.push('Q');
            }
            if matches!(cc, [_, _, true, _]) {
                s.push('k');
            }
            if matches!(cc, [_, _, _, true]) {
                s.push('q');
            }
        }

        // en passant sq
        s.push(' ');
        if let Some(last) = self.move_log.last() {
            if matches!(self.board[last.to()], Pawn(_)) && last.to().abs_diff(last.frm()) == 2 {
                let idx = last.to() as isize + if self.colour { 1 } else { -1 };
                s.push_str(I2SQ[idx as usize])
            } else {
                s.push('-');
            }
        } else {
            s.push('-');
        }

        // reversible moves,move nr
        s.push_str(
            format!(
                " {} {}",
                self.check_50_move_rule() - 1,
                self.full_move_count + self.move_log.len() / 2 + 1
            )
            .as_str(),
        );
        s
    }

    pub fn from_fen(s: &str) -> Self {
        let mut board = [Nil; 64];
        let mut offset = 0i16;
        let parts = s.split(' ').collect::<Vec<&str>>();
        for (i, c) in parts[0].chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                offset += d as i16 - 1;
            } else if c == '/' {
                offset -= 1;
            } else {
                let k: usize = (i as i16 + offset).try_into().unwrap();
                let x = 7 - k % 8;
                let y = 7 - k / 8;
                let q = x * 8 + y;
                board[q] = Piece::from_ascii(c);
            }
        }

        let mut game = Game::new(board);
        if parts.len() > 1 {
            game.colour = matches!(parts[1].chars().nth(0), Some('w') | Some('W'));
        }
        if parts.len() > 2 {
            let cc = game.can_castle.last_mut().unwrap();
            cc[0] = parts[2].contains('K');
            cc[1] = parts[2].contains('Q');
            cc[2] = parts[2].contains('k');
            cc[3] = parts[2].contains('q');
        }
        if parts.len() > 3 {
            // en passant attack
            if let Some(sq) = misc::parse_chess_coord(parts[3]) {
                let sq = sq as isize;
                let o = if !game.colour { 1 } else { -1 };
                let to: usize = (sq + o).try_into().expect("must be positive");
                let frm: usize = (sq - o).try_into().expect("must be positive");
                let m = mgen::Move::new(false, true, frm, to);
                game.move_log.push(m);
            }
        }
        if parts.len() > 4 {
            // moves since last irreversible move
            if let Ok(value) = parts[4].parse::<usize>() {
                game.half_move_clock = value;
            }
        }

        if parts.len() > 5 {
            // full-move count
            if let Ok(value) = parts[5].parse::<usize>() {
                game.full_move_count = value;
            }
        }
        game
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
                match m.ptransform(self.colour) {
                    Rook(_) => label.push_str("=R"),
                    Knight(_) => label.push_str("=N"),
                    Bishop(_) => label.push_str("=B"),
                    _ => label.push_str("=Q"),
                }
            }
        }

        self.update(m);
        let in_check = self.in_check(self.turn());
        if self.legal_moves(Some(m)).is_empty() && in_check {
            label.push('#')
        } else if self.in_check(self.turn()) {
            label.push('+')
        }
        self.backdate(m);
        label
    }

    pub fn rep_len(&self) -> usize {
        self.rep.len()
    }

    pub fn ttable_len(&self) -> usize {
        self.ttable.len()
    }

    fn is_quiescent(&self, last: &Move) -> bool {
        // quiescent unless last move was pawn near promotion
        // !self.in_check(self.colour) &&
        match self.board[last.to()] {
            Pawn(WHITE) => last.to() % 8 != 6,
            Pawn(BLACK) => last.to() % 8 != 1,
            _ => true,
        }
    }

    fn ttable_clear(&mut self) {
        let key = self.hash;
        if self.ttable.contains_key(&key) {
            self.ttable = HashMap::from([(key, self.ttable[&key])]);
        } else {
            //self.ttable = HashMap::new();
            self.ttable.clear();
        }
    }

    fn rep_clear(&mut self) {
        self.rep.clear();
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
        if m.en_passant() || self.board[m.to()] != Nil || matches!(self.board[m.frm()], Pawn(_)) {
            self.rep_clear(); // ireversible move
            self.half_move_clock = 0;
        }
        self.ttable_clear();
        self.update(&m);

        //adjust king value in end game
        self.end_game = abs_material(&self.board) < END_GAME_MATERIAL / 3;
        self.move_log.push(m);

        //update castling permissions
        let cc = self.can_castle.last_mut().unwrap();
        match (*cc, self.board[m.to()], m.frm()) {
            ([true, _, _, _], King(WHITE), 24) => (cc[0], cc[1]) = (false, false),
            ([_, true, _, _], King(WHITE), 24) => (cc[0], cc[1]) = (false, false),
            ([_, _, true, _], King(BLACK), 31) => (cc[2], cc[3]) = (false, false),
            ([_, _, _, true], King(BLACK), 31) => (cc[2], cc[3]) = (false, false),
            ([true, _, _, _], Rook(WHITE), 0) => cc[0] = false,
            ([_, true, _, _], Rook(WHITE), 56) => cc[1] = false,
            ([_, _, true, _], Rook(BLACK), 7) => cc[2] = false,
            ([_, _, _, true], Rook(BLACK), 63) => cc[3] = false,
            _ => (),
        }
    }

    pub fn check_50_move_rule(&self) -> usize {
        self.half_move_clock + self.rep.iter().map(|(_, &v)| v).sum::<usize>()
    }

    pub fn in_check(&self, colour: bool) -> bool {
        // true if other side can capture king

        mgen::in_check(
            &self.board,
            colour,
            self.bm_wking,
            self.bm_bking,
            self.bm_white | self.bm_black,
        )
    }

    fn legal_move(&mut self, m: &Move) -> bool {
        // verify move does not expose own king
        self.update(m);
        let flag = match self.board[m.to()] {
            Rook(c) | Knight(c) | Bishop(c) | Queen(c) | King(c) | Pawn(c) => self.in_check(c),
            _ => false,
        };
        self.backdate(m);
        !flag
    }

    pub fn legal_moves(&mut self, last: Option<&Move>) -> Vec<Move> {
        let in_check = self.in_check(self.colour);
        let mut moves = self.moves(self.colour, last, in_check);
        moves.retain(|m| self.legal_move(m));
        moves
    }

    fn moves(&mut self, colour: bool, last: Option<&Move>, in_check: bool) -> Vec<Move> {
        let mut l = moves(
            &self.board,
            colour,
            in_check,
            self.end_game,
            self.can_castle.last().unwrap(),
            last,
            self.bm_white,
            self.bm_black,
        );
        if colour {
            //l.sort_by(|b, a| a.val.cmp(&b.val)); // decreasing
            l.sort_unstable_by(|b, a| a.val.cmp(&b.val)); // decreasing
        } else {
            //l.sort_by(|a, b| a.val.cmp(&b.val)); // increasing
            l.sort_unstable_by(|a, b| a.val.cmp(&b.val)); // increasing
        }
        self.n_searched += l.len();
        l
    }

    pub fn turn(&self) -> bool {
        self.colour
    }

    pub fn update(&mut self, m: &Move) {
        self.log_bms.push((
            self.bm_pawns,
            self.bm_white,
            self.bm_black,
            self.bm_wking,
            self.bm_bking,
            self.board[m.to()],
            self.hash,
        ));
        let hash;
        self.board[m.to()] = if m.castle() {
            let cc = self.can_castle.last().unwrap();
            match self.board[m.frm()] {
                King(WHITE) => self.can_castle.push([false, false, cc[2], cc[3]]),
                King(BLACK) => self.can_castle.push([cc[0], cc[1], false, false]),
                _ => (),
            }

            let (x, y) = if m.to() <= 15 {
                (m.frm() - 24, m.frm() - 8) // short
            } else {
                (m.frm() + 32, m.frm() + 8) // long
            };
            hash = self.board[m.frm()].hashkey(m.to())
                ^ self.board[m.frm()].hashkey(m.frm())
                ^ self.board[x].hashkey(y)
                ^ self.board[x].hashkey(x);
            self.board[y] = self.board[x]; // move rook
            self.board[x] = Nil;
            self.board[m.frm()]
        } else if m.transform() {
            let p = m.ptransform(self.colour);
            hash = p.hashkey(m.to())
                ^ self.board[m.frm()].hashkey(m.frm())
                ^ self.board[m.to()].hashkey(m.to());
            p
        } else if m.en_passant() {
            // +9  +1 -7
            // +8   0 -8
            // +7  -1 -9
            let x = match m.to() > m.frm() {
                true => m.frm() + 8,  // west
                false => m.frm() - 8, // east
            };
            hash = self.board[m.frm()].hashkey(m.to())
                ^ self.board[m.frm()].hashkey(m.frm())
                ^ self.board[x].hashkey(x);
            self.board[x] = Nil;
            self.board[m.frm()]
        } else {
            hash = self.board[m.frm()].hashkey(m.to())
                ^ self.board[m.frm()].hashkey(m.frm())
                ^ self.board[m.to()].hashkey(m.to());
            self.board[m.frm()]
        };
        self.board[m.frm()] = Nil;
        self.material += m.val;
        self.rep_inc();
        //self.hash ^= m.hash ^ WHITE_HASH;
        self.hash ^= hash ^ WHITE_HASH;

        // update bitmaps - TODO calculate incrementally; ~6% faster?
        self.bm_pawns = 0;
        self.bm_white = 0;
        self.bm_black = 0;
        for i in 0..64 {
            match self.board[i] {
                Pawn(WHITE) => {
                    self.bm_pawns |= 1 << i;
                    self.bm_white |= 1 << i;
                }
                Pawn(BLACK) => {
                    self.bm_pawns |= 1 << i;
                    self.bm_black |= 1 << i;
                }
                Rook(WHITE) | Knight(WHITE) | Bishop(WHITE) | Queen(WHITE) => {
                    self.bm_white |= 1 << i
                }
                Rook(BLACK) | Knight(BLACK) | Bishop(BLACK) | Queen(BLACK) => {
                    self.bm_black |= 1 << i
                }
                King(WHITE) => {
                    self.bm_white |= 1 << i;
                    self.bm_wking = 1 << i
                }
                King(BLACK) => {
                    self.bm_black |= 1 << i;
                    self.bm_bking = 1 << i
                }
                _ => (),
            }
        }
        self.colour = !self.colour;
    }

    pub fn backdate(&mut self, m: &Move) {
        let bms = self.log_bms.pop().unwrap();
        let capture;
        (
            self.bm_pawns,
            self.bm_white,
            self.bm_black,
            self.bm_wking,
            self.bm_bking,
            capture,
            self.hash,
        ) = bms;
        self.colour = !self.colour;
        //self.hash ^= m.hash ^ WHITE_HASH;
        self.rep_dec();
        if m.castle() {
            self.can_castle.pop();
            let (frm, to) = if m.to() <= 15 {
                (m.frm() - 24, m.frm() - 8) // short
            } else {
                (m.frm() + 32, m.frm() + 8) // long
            };
            self.board[frm] = self.board[to]; // move rook
            self.board[to] = Nil;
        }
        self.board[m.frm()] = if m.transform() {
            Pawn(self.colour)
        } else {
            self.board[m.to()]
        };
        self.board[m.to()] = capture;

        if m.en_passant() {
            let x = match m.to() > m.frm() {
                true => m.frm() + 8,  // west
                false => m.frm() - 8, // w east
            };
            self.board[x] = match self.board[m.frm()] {
                Pawn(WHITE) => Pawn(BLACK),
                Pawn(BLACK) => Pawn(WHITE),
                _ => unreachable!(),
            }
        }

        self.material -= m.val;
    }

    fn ttstore(&mut self, depth: u16, score: i16, alpha: i16, beta: i16, m: &Move) {
        // TODO - implement more efficient hashing function
        let key = self.hash;
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

    pub fn mobility(&self) -> i16 {
        count_moves(&self.board, WHITE, self.bm_white, self.bm_black) as i16
            - count_moves(&self.board, BLACK, self.bm_white, self.bm_black) as i16
    }

    pub fn eval(&self, colour: bool) -> i16 {
        let s = self.material + self.score_pawn_structure() + self.mobility();
        if colour { s } else { -s }
        //s * (2 * (colour as i16) - 1)
    }

    pub fn score_pawn_structure(&self) -> i16 {
        let mut pen: i16 = 0;
        let bm: [u64; 2] = [self.bm_pawns & self.bm_white, self.bm_pawns & self.bm_black];
        for (i, &p) in [Pawn(WHITE), Pawn(BLACK)].iter().enumerate() {
            let nfiles = (0..8)
                .filter(|&q| 0b11111111 << (q * 8) & bm[i] > 0)
                .count() as i16;
            let npawns = bm[i].count_ones() as i16;
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
                .count() as i16;

            let x = 20 * double_pawns + 4 * isolated_pawns;
            pen += if p == Pawn(WHITE) { -x } else { x };
        }

        // passed pawn bonus
        for i in 0..8 {
            let file: u64 = 0b11111111 << (i * 8);
            let w = file & bm[0];
            let b = file & bm[1];
            if w > 0 && w > b {
                let k = 63 - w.leading_zeros();
                let q = (k % 8) as i16;
                pen += 2 * q * q;
            }
            if b > 0 && (w == 0 || b < w) {
                let k = b.trailing_zeros();
                let q = (7 - k % 8) as i16;
                pen -= 2 * q * q;
            }
        }

        pen
    }

    fn quiescence_fab(&mut self, alp: i16, beta: i16, last: &Move, rfab: bool) -> i16 {
        let colour = self.colour;

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
            self.update(&m);
            if !self.in_check(colour) {
                // legal move
                let score = -self.quiescence_fab(-beta, -alpha, &m, true);
                match bscore {
                    Some(bs) if score <= bs => (),
                    _ => {
                        if score >= beta {
                            self.backdate(&m);
                            return score;
                        }
                        bscore = Some(score);
                        alpha = max(alpha, score);
                    }
                }
            }
            self.backdate(&m);
        }
        if let Some(bs) = bscore {
            bs
        } else {
            self.eval(colour)
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
        let colour = self.colour;

        let kmove = if let Some(e) = self.ttable.get(&self.hash) {
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
            self.update(m);
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
            self.backdate(m);
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
                self.update(m);
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
                self.backdate(m);
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
