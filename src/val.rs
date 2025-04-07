use crate::hashkeys_generated::*;
use std::fmt;

pub const WHITE: Colour = Colour::white();
pub const BLACK: Colour = Colour::black();

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Colour(u8);

impl Colour {
    #[inline(always)]
    pub const fn white() -> Colour {
        Colour(1)
    }

    #[inline(always)]
    pub const fn black() -> Colour {
        Colour(0)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub const fn as_i8(&self) -> i8 {
        self.0 as i8
    }

    #[inline(always)]
    pub const fn as_isize(&self) -> isize {
        self.0 as isize
    }

    #[inline(always)]
    pub const fn opposite(&self) -> Colour {
        Colour(self.0 ^ 1)
    }

    #[inline(always)]
    pub const fn flip(&mut self) {
        self.0 ^= 1
    }

    #[inline(always)]
    pub const fn is_white(&self) -> bool {
        self.0 == 1
    }
}

pub const HASH: [[[u64; 64]; 2]; 8] = [
    [R1_HASH, R2_HASH],
    [N1_HASH, N2_HASH],
    [B1_HASH, B2_HASH],
    [Q1_HASH, Q2_HASH],
    [K1_HASH, K2_HASH],
    [P1_HASH, P2_HASH],
    [NIL_HASH, NIL_HASH],
    [NIL_HASH, NIL_HASH],
];

const VAL: [[[i16; 64]; 2]; 8] = [
    [ROOKVAL_B, ROOKVAL_W],
    [KNIGHTVAL_B, KNIGHTVAL_W],
    [BISHOPVAL_B, BISHOPVAL_W],
    [QUEENVAL_B, QUEENVAL_W],
    [KINGVAL_B, KINGVAL_W],
    [PAWNVAL_B, PAWNVAL_W],
    [[0; 64], [0; 64]],
    [[0; 64], [0; 64]],
];

const W: u8 = 0b00000001;
pub const ROOK: u8 = 0b00000010;
pub const KNIGHT: u8 = 0b00000100;
pub const BISHOP: u8 = 0b00001000;
pub const QUEEN: u8 = 0b00010000;
pub const KING: u8 = 0b00100000;
pub const PAWN: u8 = 0b01000000;
pub const WROOK: Piece = Piece(ROOK | W);
pub const WKNIGHT: Piece = Piece(KNIGHT | W);
pub const WBISHOP: Piece = Piece(BISHOP | W);
pub const WQUEEN: Piece = Piece(QUEEN | W);
pub const WKING: Piece = Piece(KING | W);
pub const WPAWN: Piece = Piece(PAWN | W);
pub const BROOK: Piece = Piece(ROOK);
pub const BKNIGHT: Piece = Piece(KNIGHT);
pub const BBISHOP: Piece = Piece(BISHOP);
pub const BQUEEN: Piece = Piece(QUEEN);
pub const BKING: Piece = Piece(KING);
pub const BPAWN: Piece = Piece(PAWN);
pub const EMPTY: Piece = Piece(0);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Piece(u8);

impl Piece {
    #[inline(always)]
    pub const fn is_officer(&self) -> bool {
        self.0 & (ROOK | KNIGHT | BISHOP | QUEEN) != 0
    }

    #[inline(always)]
    pub const fn colour(&self) -> Colour {
        // black: 0, white: 1
        Colour(self.0 & 1)
    }

    #[inline(always)]
    pub const fn is_white(&self) -> bool {
        self.0 & W == W
    }

    #[inline(always)]
    pub const fn kind(&self) -> u8 {
        // kind mask:  bits 1-6
        self.0 & 0b01111110
    }

    #[inline(always)]
    pub const fn index(&self) -> usize {
        (self.kind().trailing_zeros() - 1) as usize
    }

    #[inline(always)]
    pub const fn hashkey(&self, pos: u8) -> u64 {
        HASH[self.index()][self.colour().as_usize()][pos as usize]
    }

    #[inline(always)]
    pub const fn val(&self, pos: u8) -> i16 {
        VAL[self.index()][self.colour().as_usize()][pos as usize]
    }

    #[inline(always)]
    pub const fn new(kind: u8, colour: Colour) -> Self {
        Piece(kind | colour.as_u8())
    }

    pub const fn from_ascii(c: char) -> Piece {
        let colour = if c.is_ascii_uppercase() { WHITE } else { BLACK };
        match c.to_ascii_lowercase() {
            'r' => Piece::new(ROOK, colour),
            'n' => Piece::new(KNIGHT, colour),
            'b' => Piece::new(BISHOP, colour),
            'q' => Piece::new(QUEEN, colour),
            'k' => Piece::new(KING, colour),
            'p' => Piece::new(PAWN, colour),
            ' ' => EMPTY,
            _ => panic!("can not convert to Piece"),
        }
    }

    pub const fn to_ascii(&self) -> char {
        let p = match self.kind() {
            ROOK => 'r',
            KNIGHT => 'n',
            BISHOP => 'b',
            QUEEN => 'q',
            KING => 'k',
            PAWN => 'p',
            _ => '.',
        };
        if self.is_white() {
            p.to_ascii_uppercase()
        } else {
            p
        }
    }

    pub const fn to_unicode(&self) -> char {
        match (self.kind(), self.colour()) {
            (ROOK, WHITE) => '\u{2656}',
            (KNIGHT, WHITE) => '\u{2658}',
            (BISHOP, WHITE) => '\u{2657}',
            (QUEEN, WHITE) => '\u{2655}',
            (KING, WHITE) => '\u{2654}',
            (PAWN, WHITE) => '\u{2659}',
            (ROOK, BLACK) => '\u{265C}',
            (KNIGHT, BLACK) => '\u{265E}',
            (BISHOP, BLACK) => '\u{265D}',
            (QUEEN, BLACK) => '\u{265B}',
            (KING, BLACK) => '\u{265A}',
            (PAWN, BLACK) => '\u{265F}',
            _ => ' ',
        }
    }
}

#[rustfmt::skip]
pub const I2SQ: [&str;64] = ["h1", "h2", "h3", "h4","h5","h6","h7","h8",
                             "g1", "g2", "g3", "g4","g5","g6","g7","g8",
                             "f1", "f2", "f3", "f4","f5","f6","f7","f8",
                             "e1", "e2", "e3", "e4","e5","e6","e7","e8",
                             "d1", "d2", "d3", "d4","d5","d6","d7","d8",
                             "c1", "c2", "c3", "c4","c5","c6","c7","c8",
                             "b1", "b2", "b3", "b4","b5","b6","b7","b8",
                             "a1", "a2", "a3", "a4","a5","a6","a7","a8"];

pub const ROOT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ascii())
    }
}

#[rustfmt::skip]
const KINGVAL_W : [i16;64] = [
   24,  24,  12,  6,  6,  12,  24,  24, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   24,  24,  12,  6,  6,  12,  24,  24];
const KINGVAL_B: [i16; 64] = array_mul(-1, KINGVAL_W);

#[rustfmt::skip]
const PAWNVAL_W : [i16;64] = [
  100, 100, 101, 102, 104, 106, 108, 900, 
  100, 100, 102, 104, 106, 109, 112, 900, 
  100, 100, 104, 108, 112, 115, 118, 900, 
  100, 100, 107, 114, 121, 128, 135, 900, 
  100, 100, 106, 112, 118, 124, 132, 900, 
  100, 100, 104, 108, 112, 116, 120, 900, 
  100, 100, 102, 104, 106, 108, 112, 900, 
  100, 100, 101, 102, 104, 106, 108, 900];
const PAWNVAL_B: [i16; 64] = array_reverse(array_mul(-1, PAWNVAL_W));

#[rustfmt::skip]
const ROOKVAL_W : [i16;64] = [
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500];
const ROOKVAL_B: [i16; 64] = array_reverse(array_mul(-1, ROOKVAL_W));

#[rustfmt::skip]
const KNIGHTVAL_W : [i16;64] = [
  315, 315, 315, 315, 315, 315, 315, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 315, 315, 315, 315, 315, 315, 315];
const KNIGHTVAL_B: [i16; 64] = array_reverse(array_mul(-1, KNIGHTVAL_W));

#[rustfmt::skip]
const BISHOPVAL_W: [i16;64] = [
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350];
const BISHOPVAL_B: [i16; 64] = array_mul(-1, array_reverse(BISHOPVAL_W));

const QUEENVAL_W: [i16; 64] = [900; 64];
const QUEENVAL_B: [i16; 64] = [-900; 64];

const fn array_mul<const N: usize>(factor: i16, mut a: [i16; N]) -> [i16; N] {
    let mut i = 0;
    while i < N {
        a[i] *= factor;
        i += 1;
    }
    a
}

const fn array_reverse<T: Copy, const N: usize>(mut a: [T; N]) -> [T; N] {
    let mut i = 0;
    while i < N / 2 {
        let from_end = N - i - 1;
        (a[i], a[from_end]) = (a[from_end], a[i]);
        i += 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_index() {
        for (i, k) in [ROOK, KNIGHT, BISHOP, QUEEN, KING, PAWN]
            .into_iter()
            .enumerate()
        {
            for c in [WHITE, BLACK] {
                let p = Piece::new(k, c);
                assert_eq!(p.index(), i)
            }
        }
    }

    #[test]
    fn test_kind() {
        assert!(WROOK.is_officer());
        assert!(WKNIGHT.is_officer());
        assert!(WBISHOP.is_officer());
        assert!(WQUEEN.is_officer());
        assert!(!WKING.is_officer());
        assert!(!WPAWN.is_officer());
        assert!(BROOK.is_officer());
        assert!(BKNIGHT.is_officer());
        assert!(BBISHOP.is_officer());
        assert!(BQUEEN.is_officer());
        assert!(!BKING.is_officer());
        assert!(!BPAWN.is_officer());
    }
}
