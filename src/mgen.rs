use crate::bitmaps::*;
use crate::hashkeys_generated::WHITE_HASH;
use crate::misc::{self, sq2i};
use crate::val::*;
use crate::val::{BLACK, BPAWN, Colour, Piece, WHITE, WPAWN};
use std::collections::hash_map::{Entry, HashMap};
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Debug, Copy, Clone)]
pub struct Bitmaps {
    pub pieces: [u64; 2],
    pub pawns: u64,
    pub kings: u64,
}

// bitpacking - 1st 12 bits (6+6) for from/to, remaining 4 bits for castling and
// pawn transforms & enpassant. Castling, en passant & transform are mutually exclusive.
const TO_SHIFT: u16 = 6;
pub const CASTLE_W_SHORT: u8 = 0b0001;
pub const CASTLE_W_LONG: u8 = 0b0010;
pub const CASTLE_B_SHORT: u8 = 0b0100;
pub const CASTLE_B_LONG: u8 = 0b1000;
pub const FRM_MASK: u16 = 0b111111;
pub const TO_MASK: u16 = FRM_MASK << TO_SHIFT;
pub const PROMOTE_ROOK: u16 = 0b0010000_00000000;
pub const PROMOTE_KNIGHT: u16 = 0b0100000_00000000;
pub const PROMOTE_BISHOP: u16 = 0b0110000_00000000;
pub const PROMOTE_QUEEN: u16 = 0b1000000_00000000;
pub const PROMOTE_MASK: u16 = 0b1110000_00000000;

#[inline(always)]
const fn pack_data(promote: u16, frm: usize, to: usize) -> u16 {
    promote | ((to << 6) | frm) as u16
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub data: u16,
    pub val: i16,
}

impl Move {
    #[inline(always)]
    fn promote_kind(&self) -> u8 {
        match self.data & PROMOTE_MASK {
            PROMOTE_ROOK => ROOK,
            PROMOTE_BISHOP => BISHOP,
            PROMOTE_KNIGHT => KNIGHT,
            PROMOTE_QUEEN => QUEEN,
            _ => 0,
        }
    }

    pub fn promote_label(&self) -> &str {
        match self.data & PROMOTE_MASK {
            PROMOTE_ROOK => "=R",
            PROMOTE_BISHOP => "=B",
            PROMOTE_KNIGHT => "=N",
            PROMOTE_QUEEN => "=Q",
            _ => "",
        }
    }

    #[inline]
    pub fn is_promote(&self) -> bool {
        self.data & PROMOTE_MASK != 0
    }

    #[inline]
    pub fn frm(&self) -> u8 {
        (self.data & FRM_MASK) as u8
    }
    #[inline]
    pub fn to(&self) -> u8 {
        ((self.data & TO_MASK) >> TO_SHIFT) as u8
    }
}

pub fn ext_frm(data: u16) -> u8 {
    (data & FRM_MASK) as u8
}

pub fn ext_to(data: u16) -> u8 {
    ((data & TO_MASK) >> TO_SHIFT) as u8
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (frm, to) = (self.frm() as usize, self.to() as usize);
        write!(f, "{}{}{}", I2SQ[frm], I2SQ[to], self.promote_label())
    }
}

pub struct Board {
    squares: [Piece; 64],
    pub turn: Colour,
    pub can_castle: u8, // white short, long, black short, long
    pub material: i16,
    pub hash: u64,
    pub half_move_clock: usize, // since last irreversible move
    pub full_move_count: usize,
    pub rep: HashMap<u64, u8>,
    en_passant_sq: u8,
    bitmaps: Bitmaps,
    end_game_material: i16,
    log_bms: Vec<(Bitmaps, Piece, u64, u8, u8)>,
}

impl Default for Board {
    fn default() -> Self {
        Board::from_fen(ROOT_FEN).expect("Always good for ROOT_FEN")
    }
}

impl Index<usize> for Board {
    type Output = Piece;
    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.squares[idx]
    }
}

impl IndexMut<usize> for Board {
    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.squares[idx]
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Piece;
    type IntoIter = Iter<'a, Piece>;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in (0..8).rev() {
            write!(f, "{} ", y + 1)?;
            for x in 0..8 {
                write!(f, "{}", self.squares[(7 - x) * 8 + y])?;
            }
            writeln!(f)?;
        }
        write!(f, "  ABCDEFGH")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape codes for background and foreground colors
        let light_square_bg = "\x1b[48;5;229m"; // Light background
        let dark_square_bg = "\x1b[48;5;94m"; // Dark background
        //let light_square_bg = "\x1b[48;5;15m"; // White background
        //let dark_square_bg = "\x1b[48;5;8m";   // Gray background
        let black_fg = "\x1b[38;5;0m"; // Black foreground
        let white_fg = "\x1b[38;5;15m"; // White foreground
        let reset_colour = "\x1b[0m"; // Reset to default colour

        for y in (0..8).rev() {
            write!(f, "{} ", y + 1)?;
            for x in 0..8 {
                let i = (7 - x) * 8 + y;
                let ch = self.squares[i].to_unicode();
                let fg = if self.squares[i].is_white() {
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

impl Board {
    fn legal_move(&mut self, m: &Move) -> bool {
        // verify move does not expose own king
        self.update(m);
        let flag = self.in_check(self.turn.opposite());
        self.backdate(m);
        !flag
    }

    pub fn legal_moves(&mut self) -> Vec<Move> {
        let in_check = self.in_check(self.turn);
        let mut moves = self.moves(in_check, false);
        moves.retain(|m| self.legal_move(m));
        moves
    }

    /// true if move is castle right or left
    #[inline(always)]
    pub fn is_castle(&self, m: &Move) -> bool {
        match (m.frm(), m.to(), self.turn.is_white()) {
            (24, 8, true) => self.can_castle & CASTLE_W_SHORT != 0,
            (24, 40, true) => self.can_castle & CASTLE_W_LONG != 0,
            (31, 15, false) => self.can_castle & CASTLE_B_SHORT != 0,
            (31, 47, false) => self.can_castle & CASTLE_B_LONG != 0,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_en_passant(&self, m: &Move) -> bool {
        self.en_passant_sq > 0
            && m.to() == self.en_passant_sq
            && self.bitmaps.pawns & 1 << m.frm() != 0
    }

    #[inline(always)]
    pub fn rep_count(&self) -> u8 {
        if let Some(count) = self.rep.get(&self.hash) {
            *count
        } else {
            0
        }
    }

    #[inline(always)]
    fn rep_inc(&mut self) {
        //*self.rep.entry(self.hash).or_default() += 1;
        self.rep
            .entry(self.hash)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    #[inline(always)]
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

    pub fn from_fen(s: &str) -> Result<Board, String> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let squares = from_fen(parts[0]);

        let turn = if parts.len() > 1 {
            if parts[1].to_lowercase().starts_with('w') {
                Colour::white()
            } else {
                Colour::black()
            }
        } else {
            Colour::white()
        };

        let mut can_castle = 0;
        if parts.len() > 2 {
            for (c, q) in ['K', 'Q', 'k', 'q'].into_iter().zip([
                CASTLE_W_SHORT,
                CASTLE_W_LONG,
                CASTLE_B_SHORT,
                CASTLE_B_LONG,
            ]) {
                if parts[2].contains(c) {
                    can_castle |= q
                }
            }
        }

        let en_passant_sq = if parts.len() > 3 {
            if parts[3].starts_with('-') {
                0
            } else {
                match misc::parse_chess_coord(parts[3]) {
                    None => return Err("Bad en passant square in fen string".into()),
                    Some(val) => val,
                }
            }
        } else {
            0
        };

        let half_move_clock = if parts.len() > 4 {
            match parts[4].parse::<usize>() {
                Ok(val) => val,
                Err(_) => return Err("Expected an integer for half-move clock".into()),
            }
        } else {
            0
        };

        let full_move_count = if parts.len() > 5 {
            match parts[5].parse::<usize>() {
                Ok(val) => val - 1,
                Err(_) => return Err("Expected an integer for full-move count".into()),
            }
        } else {
            0
        };

        let bitmaps = to_bitmaps(&squares);

        let end_game_material = abs_material(&from_fen(ROOT_FEN)) / 3;
        let hash = calc_hash(&squares, turn);
        let material = material(&squares);
        let rep = HashMap::from([(hash, 1)]);

        Ok(Board {
            squares,
            bitmaps,
            turn,
            can_castle,
            end_game_material,
            log_bms: vec![],
            material,
            hash,
            en_passant_sq,
            half_move_clock,
            full_move_count,
            rep,
        })
    }

    pub fn to_fen(&self) -> String {
        let mut s = String::new();
        for y in (0..=7).rev() {
            let mut n = 0;
            for x in (0..=7).rev() {
                let idx = x * 8 + y;
                if self[idx] == EMPTY {
                    n += 1;
                } else {
                    if n > 0 {
                        s.push_str(format!("{}", n).as_str());
                        n = 0;
                    }
                    s.push(self[idx].to_ascii())
                }
            }
            if n > 0 {
                s.push_str(format!("{}", n).as_str())
            }

            if y != 0 {
                s.push('/')
            }
        }
        s.push_str(if self.turn.is_white() { " w" } else { " b" });
        s.push(' ');

        let castling_rights: String = ['K', 'Q', 'k', 'q']
            .into_iter()
            .zip([CASTLE_W_SHORT, CASTLE_W_LONG, CASTLE_B_SHORT, CASTLE_B_LONG])
            .filter(|(_, c)| self.can_castle & c != 0)
            .map(|(x, _)| x)
            .collect();
        s.push_str(if castling_rights.is_empty() {
            "-"
        } else {
            &castling_rights
        });

        // en passant sq
        s.push(' ');
        if self.en_passant_sq > 0 {
            s.push_str(I2SQ[self.en_passant_sq as usize])
        } else {
            s.push('-');
        }

        // reversible moves,move nr
        s.push_str(format!(" {} {}", self.half_moves() - 1, self.move_number()).as_str());
        s
    }

    pub fn move_number(&self) -> usize {
        self.full_move_count / 2 + 1
    }

    #[rustfmt::skip]
    pub fn to_csv(&self) -> Vec<u8> {
        const CSV_SIZE: usize = 2 * 6 * 64 + 1 + 4 + 64 + 1;
        let mut v = Vec::with_capacity(CSV_SIZE);
        for p in [
            WPAWN, WROOK, WKNIGHT, WBISHOP, WQUEEN, WKING,
            BPAWN, BROOK, BKNIGHT, BBISHOP, BQUEEN, BKING,
        ] {
            for pb in &self.squares {
                v.push((p == *pb) as u8);
            }
        }

        // turn & castling rights
        v.push(self.turn.is_white() as u8);
        v.extend(
            [CASTLE_W_SHORT, CASTLE_W_LONG, CASTLE_B_SHORT, CASTLE_B_LONG]
                .map(|c| (self.can_castle & c != 0) as u8),
        );

        // en passant
        if self.en_passant_sq>0 {
            for i in 0..64 {
                v.push((i == self.en_passant_sq) as u8);
            }
        } else {
            v.resize(v.len() + 64, 0);
        }

        v
    }

    pub fn half_moves(&self) -> usize {
        self.half_move_clock + self.rep.iter().map(|(_, &v)| v).sum::<u8>() as usize
    }

    pub fn update(&mut self, m: &Move) {
        self.log_bms.push((
            self.bitmaps,
            self[m.to() as usize],
            self.hash,
            self.can_castle,
            self.en_passant_sq,
        ));
        let hash;
        //self[m.to() as usize] = if m.castle() {
        self[m.to() as usize] = if self.is_castle(m) {
            self.en_passant_sq = 0;
            let (r_frm, r_to) = if m.to() <= 15 {
                (m.frm() - 24, m.frm() - 8) // short
            } else {
                (m.frm() + 32, m.frm() + 8) // long
            };

            self.bitmaps.kings |= 1 << m.to();
            self.bitmaps.kings ^= 1 << m.frm();
            self.bitmaps.pieces[self.turn.as_usize()] |= 1 << m.to();
            self.bitmaps.pieces[self.turn.as_usize()] ^= 1 << m.frm();
            self.bitmaps.pieces[self.turn.as_usize()] |= 1 << r_to;
            self.bitmaps.pieces[self.turn.as_usize()] ^= 1 << r_frm;

            match self[m.frm() as usize] {
                WKING => self.can_castle &= !(CASTLE_W_SHORT | CASTLE_W_LONG),
                BKING => self.can_castle &= !(CASTLE_B_SHORT | CASTLE_B_LONG),
                _ => {
                    println!("{}", self);
                    panic!("not castle {m}...")
                }
            }

            hash = self[m.frm() as usize].hashkey(m.to())
                ^ self[m.frm() as usize].hashkey(m.frm())
                ^ self[r_frm as usize].hashkey(r_to)
                ^ self[r_frm as usize].hashkey(r_frm);
            self[r_to as usize] = self.squares[r_frm as usize]; // move rook
            self[r_frm as usize] = EMPTY;
            self[m.frm() as usize]
        } else if m.is_promote() {
            self.en_passant_sq = 0;
            self.bitmaps.pieces[self.turn.as_usize()] |= 1 << m.to();
            self.bitmaps.pieces[self.turn.as_usize()] ^= 1 << m.frm();
            self.bitmaps.pawns ^= 1 << m.frm();
            if self[m.to() as usize] != EMPTY {
                // capture
                let c = self[m.to() as usize].colour();
                self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to();
            }

            let p = Piece::new(m.promote_kind(), self.turn);
            hash = p.hashkey(m.to())
                ^ self[m.frm() as usize].hashkey(m.frm())
                ^ self[m.to() as usize].hashkey(m.to());
            p
        //} else if m.en_passant() {
        } else if self.is_en_passant(m) {
            // +9  +1 -7
            // +8   0 -8
            // +7  -1 -9
            self.en_passant_sq = 0;
            let x = match m.to() > m.frm() {
                true => m.frm() + 8,  // west
                false => m.frm() - 8, // east
            };

            self.bitmaps.pieces[self.turn.as_usize()] |= 1 << m.to();
            self.bitmaps.pieces[self.turn.as_usize()] ^= 1 << m.frm();
            self.bitmaps.pieces[self.turn.opposite().as_usize()] ^= 1 << x;
            self.bitmaps.pawns |= 1 << m.to();
            self.bitmaps.pawns ^= 1 << m.frm();
            self.bitmaps.pawns ^= 1 << x;

            hash = self[m.frm() as usize].hashkey(m.to())
                ^ self[m.frm() as usize].hashkey(m.frm())
                ^ self[x as usize].hashkey(x);
            self[x as usize] = EMPTY;
            self[m.frm() as usize]
        } else {
            self.en_passant_sq = 0;
            self.bitmaps.pieces[self.turn.as_usize()] |= 1 << m.to();
            self.bitmaps.pieces[self.turn.as_usize()] ^= 1 << m.frm();
            let c = self[m.to() as usize].colour();
            match (self[m.frm() as usize].kind(), self[m.to() as usize].kind()) {
                (PAWN, PAWN) => {
                    self.bitmaps.pawns ^= 1 << m.frm();
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to();
                }
                (PAWN, ROOK | BISHOP | QUEEN | KNIGHT) => {
                    self.bitmaps.pawns |= 1 << m.to();
                    self.bitmaps.pawns ^= 1 << m.frm();
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to();
                }
                (PAWN, _) => {
                    if m.frm().abs_diff(m.to()) == 2 {
                        self.en_passant_sq = m.frm() + 2 * self.turn.as_u8() - 1;
                    }
                    self.bitmaps.pawns |= 1 << m.to();
                    self.bitmaps.pawns ^= 1 << m.frm();
                }
                (KING, PAWN) => {
                    self.bitmaps.pawns ^= 1 << m.to();
                    self.bitmaps.kings |= 1 << m.to();
                    self.bitmaps.kings ^= 1 << m.frm();
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to();
                }
                (KING, ROOK | BISHOP | QUEEN | KNIGHT) => {
                    self.bitmaps.kings |= 1 << m.to();
                    self.bitmaps.kings ^= 1 << m.frm();
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to();
                }
                (KING, _) => {
                    self.bitmaps.kings |= 1 << m.to();
                    self.bitmaps.kings ^= 1 << m.frm();
                }
                (_, PAWN) => {
                    self.bitmaps.pawns ^= 1 << m.to();
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to()
                }
                (_, ROOK | KNIGHT | QUEEN | BISHOP) => {
                    self.bitmaps.pieces[c.as_usize()] ^= 1 << m.to()
                }
                _ => (),
            }
            match self[m.frm() as usize] {
                WKING => self.can_castle &= !(CASTLE_W_SHORT | CASTLE_W_LONG),
                BKING => self.can_castle &= !(CASTLE_B_SHORT | CASTLE_B_LONG),
                WROOK if m.frm() == 0 => self.can_castle &= !CASTLE_W_SHORT,
                WROOK if m.frm() == 56 => self.can_castle &= !CASTLE_W_LONG,
                BROOK if m.frm() == 7 => self.can_castle &= !CASTLE_B_SHORT,
                BROOK if m.frm() == 63 => self.can_castle &= !CASTLE_B_LONG,
                _ => (),
            }

            hash = self[m.frm() as usize].hashkey(m.to())
                ^ self[m.frm() as usize].hashkey(m.frm())
                ^ self[m.to() as usize].hashkey(m.to());
            self[m.frm() as usize]
        };
        self[m.frm() as usize] = EMPTY;
        self.material += m.val;
        self.rep_inc();
        self.hash ^= hash ^ WHITE_HASH;
        // self.bitmaps = self.board.to_bitmaps();
        self.turn.flip();
    }

    pub fn backdate(&mut self, m: &Move) {
        let bms = self.log_bms.pop().unwrap();
        let capture;
        (
            self.bitmaps,
            capture,
            self.hash,
            self.can_castle,
            self.en_passant_sq,
        ) = bms;
        self.turn.flip();
        //self.hash ^= m.hash ^ WHITE_HASH;
        self.rep_dec();
        //if m.castle() {
        if self.is_castle(m) {
            let (frm, to) = if m.to() <= 15 {
                (m.frm() - 24, m.frm() - 8) // short
            } else {
                (m.frm() + 32, m.frm() + 8) // long
            };
            self[frm as usize] = self.squares[to as usize]; // move rook
            self[to as usize] = EMPTY;
        }
        self[m.frm() as usize] = if m.is_promote() {
            Piece::new(PAWN, self.turn)
        } else {
            self[m.to() as usize]
        };
        self[m.to() as usize] = capture;

        if self.is_en_passant(m) {
            //if m.en_passant() {
            let x = match m.to() > m.frm() {
                true => m.frm() + 8,  // west
                false => m.frm() - 8, // east
            };
            self[x as usize] = if self.squares[m.frm() as usize].is_white() {
                BPAWN
            } else {
                WPAWN
            };
        }

        self.material -= m.val;
    }

    pub fn eval(&self) -> i16 {
        let s = self.material + self.score_pawn_structure() + self.mobility();
        if self.turn.is_white() { s } else { -s }
        //s * (2 * (colour as i16) - 1)
    }

    pub const fn is_end_game(&self) -> bool {
        abs_material(&self.squares) < self.end_game_material
    }

    pub const fn to_bitmaps(&self) -> Bitmaps {
        to_bitmaps(&self.squares)
    }

    pub fn score_pawn_structure(&self) -> i16 {
        let mut pen: i16 = 0;
        let bm: [u64; 2] = [
            self.bitmaps.pawns & self.bitmaps.pieces[WHITE.as_usize()],
            self.bitmaps.pawns & self.bitmaps.pieces[BLACK.as_usize()],
        ];
        for (i, &p) in [WPAWN, BPAWN].iter().enumerate() {
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
            pen += if p == WPAWN { -x } else { x };
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

    pub fn mobility(&self) -> i16 {
        self.count_moves(WHITE) as i16 - self.count_moves(BLACK) as i16
    }

    // true if !colour side can capture colour king
    pub fn in_check(&self, colour: Colour) -> bool {
        let bm_king = self.bitmaps.kings & self.bitmaps.pieces[colour.as_usize()];
        let bm_board =
            self.bitmaps.pieces[BLACK.as_usize()] | self.bitmaps.pieces[WHITE.as_usize()];
        let opp = colour.opposite().as_usize();
        self.squares
            .iter()
            .enumerate()
            .filter(|(frm, _)| 1 << frm & self.bitmaps.pieces[opp] != 0)
            .any(|(frm, &p)| match p.kind() {
                KNIGHT => BM_KNIGHT_MOVES[frm] & bm_king != 0,
                KING => BM_KING_MOVES[frm] & bm_king != 0,
                PAWN => BM_PAWN_CAPTURES[opp][frm] & bm_king != 0,
                ROOK => ray_check(frm, BM_ROOK_MOVES[frm], bm_board, bm_king),
                BISHOP => ray_check(frm, BM_BISHOP_MOVES[frm], bm_board, bm_king),
                QUEEN => ray_check(frm, BM_QUEEN_MOVES[frm], bm_board, bm_king),
                _ => false,
            })
    }

    pub fn moves(&self, in_check: bool, end_game: bool) -> Vec<Move> {
        let mut v = Vec::with_capacity(50);
        self.squares
            .iter()
            .enumerate()
            .filter(|(frm, _)| 1 << frm & self.bitmaps.pieces[self.turn.as_usize()] != 0)
            .for_each(|(frm, &p)| match p.kind() {
                KNIGHT => self.knight_moves(&mut v, frm),
                KING => self.king_moves(&mut v, frm, end_game, in_check),
                PAWN => self.pawn_moves(&mut v, frm),
                ROOK => self.ray_moves(&mut v, frm, BM_ROOK_MOVES[frm]),
                BISHOP => self.ray_moves(&mut v, frm, BM_BISHOP_MOVES[frm]),
                QUEEN => self.ray_moves(&mut v, frm, BM_QUEEN_MOVES[frm]),
                _ => (),
            });
        v
    }

    /// delta value of moving a piece between two squares  and possibly capturing another piece
    #[inline(always)]
    const fn delta_val(&self, frm: u8, to: u8) -> i16 {
        self.squares[frm as usize].val(to)
            - self.squares[frm as usize].val(frm)
            - self.squares[to as usize].val(to)
    }

    fn knight_moves(&self, v: &mut Vec<Move>, frm: usize) {
        let mut b = BM_KNIGHT_MOVES[frm] & !self.bitmaps.pieces[self.turn.as_usize()];
        while b != 0 {
            let to = b.trailing_zeros() as u8;
            b &= !(1 << to);

            v.push(Move {
                data: pack_data(0, frm, to as usize),
                val: self.delta_val(frm as u8, to),
            })
        }
    }

    fn ray_moves(&self, v: &mut Vec<Move>, frm: usize, moves: u64) {
        let bm_board =
            self.bitmaps.pieces[WHITE.as_usize()] | self.bitmaps.pieces[BLACK.as_usize()];
        let bl = bm_blockers(frm as u8, moves & bm_board);

        let mut b = moves & !bl & !self.bitmaps.pieces[self.turn.as_usize()];
        while b != 0 {
            let to = b.trailing_zeros() as u8;
            b &= !(1 << to);
            v.push(Move {
                data: pack_data(0, frm, to as usize),
                val: self.delta_val(frm as u8, to),
            })
        }
    }

    fn pawn_moves(&self, v: &mut Vec<Move>, frm: usize) {
        let bm_board =
            self.bitmaps.pieces[WHITE.as_usize()] | self.bitmaps.pieces[BLACK.as_usize()];
        let cap = BM_PAWN_CAPTURES[self.turn.as_usize()][frm]
            & self.bitmaps.pieces[self.turn.opposite().as_usize()];
        let step1: u64 = BM_PAWN_STEP1[self.turn.as_usize()][frm] & !bm_board;
        let step2: u64 = if self.turn.is_white() {
            step1 << 1
        } else {
            step1 >> 1
        };
        let step2: u64 = step2 & BM_PAWN_STEP2[self.turn.as_usize()][frm] & !bm_board;

        let mut b = cap | step1 | step2;
        while b != 0 {
            let to = b.trailing_zeros() as usize;
            b &= !(1 << to);

            match to % 8 {
                0 | 7 => {
                    // promotion
                    let frm_val = self[frm].val(frm as u8);
                    let to_val = self[to].val(to as u8);
                    let officers = [
                        (PROMOTE_QUEEN, Piece::new(QUEEN, self.turn)),
                        (PROMOTE_ROOK, Piece::new(ROOK, self.turn)),
                        (PROMOTE_KNIGHT, Piece::new(KNIGHT, self.turn)),
                        (PROMOTE_BISHOP, Piece::new(BISHOP, self.turn)),
                    ];
                    for (pk, p) in officers {
                        v.push(Move {
                            data: pack_data(pk, frm, to),
                            val: p.val(to as u8) - frm_val - to_val,
                        })
                    }
                }
                _ => v.push(Move {
                    data: pack_data(0, frm, to),
                    val: self.delta_val(frm as u8, to as u8),
                }),
            }
        }

        if self.en_passant_sq > 0 {
            let lto = self.en_passant_sq as u8 + 2 * self.turn.opposite().as_u8() - 1;
            let mut b = BM_PAWN_CAPTURES[self.turn.as_usize()][frm] & 1 << self.en_passant_sq;
            while b != 0 {
                let to = b.trailing_zeros() as usize;
                b &= !(1 << to);

                v.push(Move {
                    data: pack_data(0, frm, to),
                    val: self[frm].val(to as u8)
                        - self[frm].val(frm as u8)
                        - self[lto as usize].val(lto),
                });
            }
        }
    }

    fn king_moves(&self, v: &mut Vec<Move>, frm: usize, end_game: bool, in_check: bool) {
        let bm_board =
            self.bitmaps.pieces[WHITE.as_usize()] | self.bitmaps.pieces[BLACK.as_usize()];
        // change king valuation in end_game
        let p = match (self[frm], end_game) {
            (WKING, true) => BKING,
            (BKING, true) => WKING,
            (_, false) => self[frm],
            _ => panic!(),
        };

        struct Castle {
            side: u8,
            block_mask: u64, // squares between K & R - must be unoccupied
            rook: Piece,
            rook_from: u8,
            rook_to: u8,
            king_to: u8,
        }

        const CASTLES: [[Castle; 2]; 2] = [
            [
                Castle {
                    side: CASTLE_B_SHORT,
                    block_mask: 1 << sq2i("f8") | 1 << sq2i("g8"),
                    rook: BROOK,
                    rook_from: sq2i("h8"),
                    rook_to: sq2i("f8"),
                    king_to: sq2i("g8"),
                },
                Castle {
                    side: CASTLE_B_LONG,
                    block_mask: 1 << sq2i("b8") | 1 << sq2i("c8") | 1 << sq2i("d8"),
                    rook: BROOK,
                    rook_from: sq2i("a8"),
                    rook_to: sq2i("d8"),
                    king_to: sq2i("c8"),
                },
            ],
            [
                Castle {
                    side: CASTLE_W_SHORT,
                    block_mask: 1 << sq2i("f1") | 1 << sq2i("g1"),
                    rook: WROOK,
                    rook_from: sq2i("h1"),
                    rook_to: sq2i("f1"),
                    king_to: sq2i("g1"),
                },
                Castle {
                    side: CASTLE_W_LONG,
                    block_mask: 1 << sq2i("d1") | 1 << sq2i("c1") | 1 << sq2i("b1"),
                    rook: WROOK,
                    rook_from: sq2i("a1"),
                    rook_to: sq2i("d1"),
                    king_to: sq2i("c1"),
                },
            ],
        ];

        if !in_check && self.bitmaps.kings & 1 << frm != 0 {
            for c in &CASTLES[self.turn.as_usize()] {
                if self.can_castle & c.side != 0 && bm_board & c.block_mask == 0 {
                    v.push(Move {
                        data: pack_data(0, frm, c.king_to as usize),
                        val: p.val(c.king_to as u8) - p.val(frm as u8) + c.rook.val(c.rook_to)
                            - c.rook.val(c.rook_from),
                    });
                }
            }
        }

        let mut b = BM_KING_MOVES[frm] & !self.bitmaps.pieces[self.turn.as_usize()];
        while b != 0 {
            let to = b.trailing_zeros() as u8;
            b &= !(1 << to);

            v.push(Move {
                data: pack_data(0, frm, to as usize),
                val: self.delta_val(frm as u8, to),
            })
        }
    }

    // count pseudo legal moves - ignoring en passant & castling
    fn count_moves(&self, colour: Colour) -> u32 {
        let bm_board =
            self.bitmaps.pieces[WHITE.as_usize()] | self.bitmaps.pieces[BLACK.as_usize()];
        let bm_own = self.bitmaps.pieces[colour.as_usize()];
        let bm_opp = self.bitmaps.pieces[colour.opposite().as_usize()];

        self.squares
            .iter()
            .enumerate()
            .filter(|(frm, _)| 1 << frm & self.bitmaps.pieces[colour.as_usize()] != 0)
            .map(|(frm, &p)| match p.kind() {
                KNIGHT => (BM_KNIGHT_MOVES[frm] & !bm_own).count_ones(),
                KING => (BM_KING_MOVES[frm] & !bm_own).count_ones(),
                PAWN => count_pawn_moves(frm, bm_opp, bm_board, colour),
                ROOK => count_ray_moves(frm, BM_ROOK_MOVES[frm], bm_board, bm_own),
                BISHOP => count_ray_moves(frm, BM_BISHOP_MOVES[frm], bm_board, bm_own),
                QUEEN => count_ray_moves(frm, BM_QUEEN_MOVES[frm], bm_board, bm_own),
                _ => 0,
            })
            .sum()
    }
}

// +9  +1 -7
// +8   0 -8
// +7  -1 -9

fn count_pawn_moves(frm: usize, bm_opp: u64, bm_board: u64, colour: Colour) -> u32 {
    // TODO  - calc all at the same time;
    let cap = BM_PAWN_CAPTURES[colour.as_usize()][frm] & bm_opp;
    let step1 = BM_PAWN_STEP1[colour.as_usize()][frm] & !bm_board;
    let step2 = if colour.is_white() {
        step1 << 1
    } else {
        step1 >> 1
    };
    let step2 = step2 & BM_PAWN_STEP2[colour.as_usize()][frm] & !bm_board;
    (cap | step1 | step2).count_ones()
}

fn count_ray_moves(frm: usize, moves: u64, bm_board: u64, bm_own: u64) -> u32 {
    (moves & !bm_own & !bm_blockers(frm as u8, moves & bm_board)).count_ones()
}

fn ray_check(frm: usize, moves: u64, bm_board: u64, bm_king: u64) -> bool {
    moves & bm_king & !bm_blockers(frm as u8, moves & bm_board) != 0
}

const fn to_bitmaps(squares: &[Piece]) -> Bitmaps {
    let mut bm = Bitmaps {
        pieces: [0, 0],
        pawns: 0,
        kings: 0,
    };
    let mut i = 0;
    while i < squares.len() {
        let c = squares[i].colour();
        match squares[i].kind() {
            ROOK | KNIGHT | BISHOP | QUEEN => bm.pieces[c.as_usize()] |= 1 << i,
            PAWN => {
                bm.pieces[c.as_usize()] |= 1 << i;
                bm.pawns |= 1 << i
            }
            KING => {
                bm.pieces[c.as_usize()] |= 1 << i;
                bm.kings |= 1 << i
            }
            _ => (),
        }
        i += 1;
    }
    bm
}

pub const fn material(squares: &[Piece]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < squares.len() {
        val += squares[i].val(i as u8);
        i += 1;
    }
    val
}

pub const fn abs_material(squares: &[Piece]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < squares.len() {
        val += squares[i].val(i as u8).abs();
        i += 1;
    }
    val
}

pub const fn calc_hash(squares: &[Piece], colour: Colour) -> u64 {
    let mut key = match colour {
        WHITE => WHITE_HASH,
        _ => 0,
    };

    let mut i = 0;
    while i < squares.len() {
        match squares[i] {
            EMPTY => (),
            _ => key ^= squares[i].hashkey(i as u8),
        };
        i += 1;
    }
    key
}

fn from_fen(s: &str) -> [Piece; 64] {
    let mut squares = [EMPTY; 64];
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
            squares[q] = Piece::from_ascii(c);
        }
    }
    squares
}

#[cfg(test)]
mod tests {
    use crate::benchmark::*;
    use crate::*;

    #[test]
    fn test_en_passant() -> Result<(), String> {
        let board = Board::from_fen(GUNDERSEN_FAUL[1].0)?;
        let mut game = Game::new(board);
        let moves = game.legal_moves();
        let (frm, to) = misc::str2move("g7g5").unwrap();
        let r = moves.iter().position(|m| (m.frm(), m.to()) == (frm, to));
        assert!(r.is_some());
        let m = moves[r.unwrap()];
        game.make_move(m);

        let moves = game.legal_moves();
        let (frm, to) = misc::str2move("h5g6").unwrap();
        let r = moves.iter().position(|m| (m.frm(), m.to()) == (frm, to));
        assert!(r.is_some());
        let m = moves[r.unwrap()];
        game.make_move(m);

        let moves = game.legal_moves();
        assert_eq!(moves.len(), 0);
        Ok(())
    }
}
