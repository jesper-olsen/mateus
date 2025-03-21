use crate::Piece;
use crate::bitmaps::*;
use crate::hashkeys_generated::WHITE_HASH;
use crate::val::*;
use crate::val::{Colour::*, Piece::*};
use crate::misc;
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
const CASTLE_BIT: u16 = 1 << 12;
const EN_PASSANT_BIT: u16 = 1 << 13;
const TRANSFORM_BIT: u16 = 1 << 14;
const TO_SHIFT: u16 = 6;
pub const CASTLE_W_SHORT: u8 = 0b0001;
pub const CASTLE_W_LONG: u8  = 0b0010;
pub const CASTLE_B_SHORT: u8 = 0b0100;
pub const CASTLE_B_LONG: u8  = 0b1000;
pub const FRM_MASK: u16 = 0b111111;
pub const TO_MASK: u16 = FRM_MASK << TO_SHIFT;

const fn pack_data(
    castle: bool,
    en_passant: bool,
    ptransform: Piece,
    frm: usize,
    to: usize,
) -> u16 {
    let (transform, tbits) = match ptransform {
        Rook(_) => (true, 1 << 15),
        Knight(_) => (true, 1 << 12),
        Bishop(_) => (true, 1 << 13),
        Queen(_) => (true, 0),
        _ => (false, 0),
    };
    ((castle as u16) << 12)
        | ((en_passant as u16) << 13)
        | ((transform as u16) << 14)
        | ((to << 6) | frm) as u16
        | tbits
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub data: u16,
    pub val: i16,
}

impl Move {
    pub fn new(castle: bool, en_passant: bool, frm: usize, to: usize) -> Self {
        // incomplete - needed by from_fen
        let data = pack_data(castle, en_passant, Nil, frm, to);
        Move { data, val: 0 }
    }
    #[inline]
    pub fn castle(&self) -> bool {
        self.data & CASTLE_BIT != 0 && !self.transform()
    }
    #[inline]
    pub fn en_passant(&self) -> bool {
        (self.data & EN_PASSANT_BIT) != 0 && !self.transform()
    }
    #[inline]
    pub fn ptransform(&self, colour: Colour) -> Piece {
        const MASK: u16 = 1 << 15 | 1 << 13 | 1 << 12;
        match self.data & MASK {
            0b10000000_00000000 => Rook(colour),
            0b00100000_00000000 => Bishop(colour),
            0b00010000_00000000 => Knight(colour),
            _ => Queen(colour),
        }
    }
    #[inline]
    pub fn transform(&self) -> bool {
        self.data & TRANSFORM_BIT != 0
    }
    #[inline]
    pub fn frm(&self) -> usize {
        (self.data & FRM_MASK) as usize
    }
    #[inline]
    pub fn to(&self) -> usize {
        ((self.data & TO_MASK) >> TO_SHIFT) as usize
    }
}

pub fn ext_frm(data: u16) -> u8 {
    (data & FRM_MASK) as u8
}

pub fn ext_to(data: u16) -> u8 {
    ((data & TO_MASK) >> TO_SHIFT) as u8
}

pub const NULL_MOVE: Move = Move {
    data: pack_data(false, false, Piece::Nil, 0, 0),
    val: 0,
};

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (frm, to) = (self.frm(), self.to());
        let t = if self.transform() {
            match self.ptransform(White) {
                Rook(_) => "=R",
                Knight(_) => "=N",
                Bishop(_) => "=B",
                _ => "=Q",
            }
        } else {
            ""
        };
        write!(f, "{}{}{t}", I2SQ[frm], I2SQ[to])
    }
}

pub struct Board {
    pub squares: [Piece; 64],
    pub colour: Colour,
    pub bitmaps: Bitmaps,
    pub can_castle: u8, // white short, long, black short, long
    pub end_game_material: i16,
    pub log_bms: Vec<(Bitmaps, Piece, u64, u8)>,
    pub move_log: Vec<Move>,
    pub material: i16,
    pub hash: u64,
    pub half_move_clock: usize, // since last irreversible move
    pub full_move_count: usize,
}

impl Default for Board {
    #[rustfmt::skip]
    fn default() -> Self {
        let squares =  [
                Rook(White),   Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Rook(Black), 
                Knight(White), Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Knight(Black), 
                Bishop(White), Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Bishop(Black), 
                King(White),   Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), King(Black), 
                Queen(White),  Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Queen(Black), 
                Bishop(White), Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Bishop(Black), 
                Knight(White), Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Knight(Black), 
                Rook(White),   Pawn(White), Nil, Nil, Nil, Nil, Pawn(Black), Rook(Black),
            ];
        let bitmaps = to_bitmaps(&squares);
        let end_game_material = abs_material(&squares) / 3;
        let material = material(&squares);
        let colour = White;
        let hash = calc_hash(&squares, colour);
        Board {
            squares,
            bitmaps,
            colour,
            can_castle: CASTLE_W_SHORT | CASTLE_W_LONG | CASTLE_B_SHORT | CASTLE_B_LONG,
            end_game_material,
            log_bms: vec![],
            move_log: Vec::new(),
            material,
            hash,
            half_move_clock: 0,
            full_move_count: 0,
        }
    }
}

impl Index<usize> for Board {
    type Output = Piece;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.squares[idx]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.squares[idx]
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Piece;
    type IntoIter = Iter<'a, Piece>;

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
    pub fn from_fen(s: &str) -> Self {
        let mut squares = [Nil; 64];
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

        let colour = if parts.len() > 1 {
            if parts[1].to_lowercase().starts_with('w') {
                Colour::White
            } else {
                Colour::Black
            }
        } else {
            Colour::White
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

        let mut move_log = Vec::new();
        if parts.len() > 3 {
            // en passant attack
            if let Some(sq) = misc::parse_chess_coord(parts[3]) {
                let sq = sq as isize;
                let o = if colour.is_white() { -1 } else { 1 };
                let to: usize = (sq + o).try_into().expect("must be positive");
                let frm: usize = (sq - o).try_into().expect("must be positive");
                let m = Move::new(false, true, frm, to);
                move_log.push(m);
            }
        }

        
        let half_move_clock = if parts.len() > 4 {
            // moves since last irreversible move
            if let Ok(value) = parts[4].parse::<usize>() {
                value
            } else {
                0
                }
        } else {
            0
            };

        let full_move_count = if parts.len() > 5 {
            // full-move count
            if let Ok(value) = parts[5].parse::<usize>() {
                value
            } else {
                0  // TODO - Err ?
            }
        } else {
            0
        };

        let bitmaps = to_bitmaps(&squares);
        let end_game_material = abs_material(&Board::default().squares); // TODO
        let hash = calc_hash(&squares, colour);
        let material = material(&squares);
        
        Board {
            squares,
            bitmaps,
            colour,
            can_castle,
            end_game_material,
            log_bms: vec![],
            move_log,
            material,
            hash,
            half_move_clock,
            full_move_count,
        }

        
    }

    pub fn eval(&self) -> i16 {
        let s = self.material + self.score_pawn_structure() + self.mobility();
        if self.colour.is_white() { s } else { -s }
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
            self.bitmaps.pawns & self.bitmaps.pieces[White as usize],
            self.bitmaps.pawns & self.bitmaps.pieces[Black as usize],
        ];
        for (i, &p) in [Pawn(White), Pawn(Black)].iter().enumerate() {
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
            pen += if p == Pawn(White) { -x } else { x };
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
        self.count_moves(White) as i16 - self.count_moves(Black) as i16
    }

    // true if !colour side can capture colour king
    pub fn in_check(&self, colour: Colour) -> bool {
        let bm_king = self.bitmaps.kings & self.bitmaps.pieces[colour as usize];
        let bm_board = self.bitmaps.pieces[Black as usize] | self.bitmaps.pieces[White as usize];
        self.squares.iter().enumerate().any(|(frm, &p)| match p {
            Knight(c) if c != colour => BM_KNIGHT_MOVES[frm] & bm_king != 0,
            King(c) if c != colour => BM_KING_MOVES[frm] & bm_king != 0,
            Pawn(c) if c != colour => BM_PAWN_CAPTURES[colour as usize][frm] & bm_king != 0,
            Rook(c) if c != colour => ray_check(frm, BM_ROOK_MOVES[frm], bm_board, bm_king),
            Bishop(c) if c != colour => ray_check(frm, BM_BISHOP_MOVES[frm], bm_board, bm_king),
            Queen(c) if c != colour => ray_check(frm, BM_QUEEN_MOVES[frm], bm_board, bm_king),
            _ => false,
        })
    }

    pub fn moves(
        &self,
        in_check: bool,
        end_game: bool,
        can_castle: u8,
        last: Option<&Move>,
    ) -> Vec<Move> {
        let colour = self.colour;
        let bitmaps = OBitmaps {
            bm_board: self.bitmaps.pieces[White as usize] | self.bitmaps.pieces[Black as usize],
            bm_own: self.bitmaps.pieces[colour as usize],
            bm_opp: self.bitmaps.pieces[colour.opposite() as usize],
        };

        let last = if let Some(m) = last { m } else { &NULL_MOVE };

        let mut v = Vec::with_capacity(50);
        self.squares
            .iter()
            .enumerate()
            .for_each(|(frm, &p)| match p {
                Knight(c) if c == colour => self.knight_moves(&mut v, frm, &bitmaps),
                King(c) if c == colour => {
                    self.king_moves(&mut v, frm, &bitmaps, end_game, can_castle, in_check)
                }
                Pawn(c) if c == colour => self.pawn_moves(&mut v, frm, last, &bitmaps, colour),
                Rook(c) if c == colour => self.ray_moves(&mut v, frm, BM_ROOK_MOVES[frm], &bitmaps),
                Bishop(c) if c == colour => {
                    self.ray_moves(&mut v, frm, BM_BISHOP_MOVES[frm], &bitmaps)
                }
                Queen(c) if c == colour => {
                    self.ray_moves(&mut v, frm, BM_QUEEN_MOVES[frm], &bitmaps)
                }
                _ => (),
            });
        v
    }

    fn knight_moves(&self, v: &mut Vec<Move>, frm: usize, bitmaps: &OBitmaps) {
        v.extend(
            bm2vec(BM_KNIGHT_MOVES[frm] & !bitmaps.bm_own)
                .iter()
                .map(|&to| Move {
                    data: pack_data(false, false, Nil, frm, to),
                    val: self.squares[frm].val(to)
                        - self.squares[frm].val(frm)
                        - self.squares[to].val(to),
                }),
        );
    }

    fn ray_moves(&self, v: &mut Vec<Move>, frm: usize, moves: u64, bitmaps: &OBitmaps) {
        let bl: u64 = bm2vec(moves & bitmaps.bm_board)
            .iter()
            .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
        v.extend(
            bm2vec(moves & !bl & !bitmaps.bm_own)
                .iter()
                .map(|&to| Move {
                    data: pack_data(false, false, Nil, frm, to),
                    val: self.squares[frm].val(to)
                        - self.squares[frm].val(frm)
                        - self.squares[to].val(to),
                }),
        );
    }

    fn pawn_moves(
        &self,
        v: &mut Vec<Move>,
        frm: usize,
        last: &Move,
        bitmaps: &OBitmaps,
        colour: Colour,
    ) {
        let cidx = if colour.is_white() { 0 } else { 1 };
        let cap = BM_PAWN_CAPTURES[cidx][frm] & bitmaps.bm_opp;
        let step1: u64 = BM_PAWN_STEP1[cidx][frm] & !bitmaps.bm_board;
        let step2: u64 = if colour.is_white() {
            step1 << 1
        } else {
            step1 >> 1
        };
        let step2: u64 = step2 & BM_PAWN_STEP2[cidx][frm] & !bitmaps.bm_board;
        let vto = bm2vec(cap | step1 | step2);

        v.extend(vto.iter().flat_map(|&to| {
            match to % 8 {
                0 | 7 => vec![
                    Move {
                        data: pack_data(false, false, Queen(colour), frm, to),
                        val: Piece::Queen(colour).val(to)
                            - self.squares[frm].val(frm)
                            - self.squares[to].val(to),
                    },
                    Move {
                        data: pack_data(false, false, Rook(colour), frm, to),
                        val: Piece::Rook(colour).val(to)
                            - self.squares[frm].val(frm)
                            - self.squares[to].val(to),
                    },
                    Move {
                        data: pack_data(false, false, Knight(colour), frm, to),
                        val: Piece::Knight(colour).val(to)
                            - self.squares[frm].val(frm)
                            - self.squares[to].val(to),
                    },
                    Move {
                        data: pack_data(false, false, Bishop(colour), frm, to),
                        val: Piece::Bishop(colour).val(to)
                            - self.squares[frm].val(frm)
                            - self.squares[to].val(to),
                    },
                ]
                .into_iter(),
                _ => vec![Move {
                    data: pack_data(false, false, Nil, frm, to),
                    val: self.squares[frm].val(to)
                        - self.squares[frm].val(frm)
                        - self.squares[to].val(to),
                }]
                .into_iter(),
            }
        }));

        // en passant
        if matches!(self.squares[last.to()], Pawn(_)) && last.to().abs_diff(last.frm()) == 2 {
            // square attacked if last move was a step-2 pawn move
            let idx = last.frm() as isize + if colour.is_white() { -1 } else { 1 };

            v.extend(
                bm2vec(BM_PAWN_CAPTURES[cidx][frm] & 1 << idx)
                    .iter()
                    .map(|&to| Move {
                        data: pack_data(false, true, Nil, frm, to),
                        val: self.squares[frm].val(to)
                            - self.squares[frm].val(frm)
                            - self.squares[last.to()].val(last.to()),
                    }),
            );
        }
    }

    fn king_moves(
        &self,
        v: &mut Vec<Move>,
        frm: usize,
        bitmaps: &OBitmaps,
        end_game: bool,
        can_castle: u8,
        in_check: bool,
    ) {
        // change king valuation in end_game
        let p = match (self.squares[frm], end_game) {
            (King(White), true) => King(Black),
            (King(Black), true) => King(White),
            (_, false) => self.squares[frm],
            _ => panic!(),
        };

        // castling
        // check squares between K & R unoccupied
        const WSHORT: u64 = 1 << 8 | 1 << 16;
        const WLONG: u64 = 1 << 32 | 1 << 40 | 1 << 48;
        const BSHORT: u64 = 1 << 15 | 1 << 23;
        const BLONG: u64 = 1 << 55 | 1 << 47 | 1 << 39;

        let cc2 = [
            (can_castle & CASTLE_W_SHORT!=0 && frm == 24 && !in_check && self.squares[0] == Rook(White) && bitmaps.bm_board & WSHORT == 0,
             Rook(White), 8, 0, 16,),
            (can_castle & CASTLE_W_LONG!=0 && frm == 24 && !in_check && self.squares[56] == Rook(White) && bitmaps.bm_board & WLONG == 0,
             Rook(White), 40, 56, 32,),
            (can_castle & CASTLE_B_SHORT!=0 && frm == 31 && !in_check && self.squares[7] == Rook(Black) && bitmaps.bm_board & BSHORT == 0,
             Rook(Black), 15, 7, 23,),
            (can_castle & CASTLE_B_LONG!=0 && frm == 31 && !in_check && self.squares[63] == Rook(Black) && bitmaps.bm_board & BLONG == 0,
             Rook(Black), 47, 63, 39,),
        ];

        v.extend(
            bm2vec(BM_KING_MOVES[frm] & !bitmaps.bm_own)
                .iter()
                .map(|&to| Move {
                    data: pack_data(false, false, Nil, frm, to),
                    //castle: false,
                    //en_passant: false,
                    //transform: false,
                    val: p.val(to) - p.val(frm) - self.squares[to].val(to),
                })
                .chain(
                    cc2.iter()
                        .filter(|(c, _, _, _, _)| *c)
                        .map(|(_, r, to, rfrm, rto)| Move {
                            data: pack_data(true, false, Nil, frm, *to as usize),
                            val: p.val(*to as usize) - p.val(frm) + r.val(*rto) - r.val(*rfrm),
                        }),
                ),
        );
    }

    // count pseudo legal moves - ignoring en passant & castling
    fn count_moves(&self, colour: Colour) -> u32 {
        let bm_board = self.bitmaps.pieces[White as usize] | self.bitmaps.pieces[Black as usize];
        let bm_own = self.bitmaps.pieces[colour as usize];
        let bm_opp = self.bitmaps.pieces[colour.opposite() as usize];

        self.squares
            .iter()
            .enumerate()
            .map(|(frm, &p)| match p {
                Knight(c) if c == colour => (BM_KNIGHT_MOVES[frm] & !bm_own).count_ones(),
                King(c) if c == colour => (BM_KING_MOVES[frm] & !bm_own).count_ones(),
                Pawn(c) if c == colour => count_pawn_moves(frm, bm_opp, bm_board, colour),
                Rook(c) if c == colour => {
                    count_ray_moves(frm, BM_ROOK_MOVES[frm], bm_board, bm_own)
                }
                Bishop(c) if c == colour => {
                    count_ray_moves(frm, BM_BISHOP_MOVES[frm], bm_board, bm_own)
                }
                Queen(c) if c == colour => {
                    count_ray_moves(frm, BM_QUEEN_MOVES[frm], bm_board, bm_own)
                }
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
    let cidx = if colour.is_white() { 0 } else { 1 };
    let cap = BM_PAWN_CAPTURES[cidx][frm] & bm_opp;
    let step1 = BM_PAWN_STEP1[cidx][frm] & !bm_board;
    let step2 = if colour.is_white() {
        step1 << 1
    } else {
        step1 >> 1
    };
    let step2 = step2 & BM_PAWN_STEP2[cidx][frm] & !bm_board;
    (cap | step1 | step2).count_ones()
}

fn count_ray_moves(frm: usize, moves: u64, bm_board: u64, bm_own: u64) -> u32 {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    (moves & !bl & !bm_own).count_ones()
}

fn ray_check(frm: usize, moves: u64, bm_board: u64, bm_king: u64) -> bool {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    (moves & !bl & bm_king) != 0
}

struct OBitmaps {
    bm_board: u64,
    bm_own: u64,
    bm_opp: u64,
}

const fn to_bitmaps(squares: &[Piece;64]) -> Bitmaps {
    let mut bm = Bitmaps {
        pieces: [0, 0],
        pawns: 0,
        kings: 0,
    };
    let mut i = 0;
    while i < 64 {
        match squares[i] {
            Rook(c) | Knight(c) | Bishop(c) | Queen(c) => bm.pieces[c as usize] |= 1 << i,
            Pawn(c) => {
                bm.pieces[c as usize] |= 1 << i;
                bm.pawns |= 1 << i
            }
            King(c) => {
                bm.pieces[c as usize] |= 1 << i;
                bm.kings |= 1 << i
            }
            Nil => (),
        }
        i += 1;
    }
    bm
}

pub const fn material(squares: &[Piece;64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < squares.len() {
        val += squares[i].val(i);
        i += 1;
    }
    val
}

pub const fn abs_material(squares: &[Piece;64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < squares.len() {
        val += squares[i].val(i).abs();
        i += 1;
    }
    val
}

    pub const fn calc_hash(squares: &[Piece;64], colour: Colour) -> u64 {
        let mut key = match colour {
            Colour::White => WHITE_HASH,
            Colour::Black => 0,
        };

        let mut i = 0;
        while i < 64 {
            match squares[i] {
                Piece::Nil => (),
                _ => key ^= squares[i].hashkey(i),
            };
            i += 1;
        }
        key
    }
