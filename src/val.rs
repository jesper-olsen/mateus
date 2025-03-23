use crate::hashkeys_generated::*;
use Piece::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Colour {
    Black,
    White,
}
use Colour::*;

impl Colour {
    pub const fn opposite(&self) -> Colour {
        match *self {
            White => Black,
            Black => White,
        }
    }

    pub const fn flip(&mut self) {
        *self = match *self {
            White => Black,
            Black => White,
        }
    }

    pub const fn is_white(&self) -> bool {
        match *self {
            White => true,
            Black => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Piece {
    Rook(Colour),
    Knight(Colour),
    Bishop(Colour),
    Queen(Colour),
    King(Colour),
    Pawn(Colour),
    Nil,
}
impl Piece {
    pub const fn hashkey(&self, pos: usize) -> u64 {
        match self {
            Piece::Rook(c) => R_HASH[*c as usize][pos],
            Piece::Knight(c) => N_HASH[*c as usize][pos],
            Piece::Bishop(c) => B_HASH[*c as usize][pos],
            Piece::King(c) => K_HASH[*c as usize][pos],
            Piece::Queen(c) => Q_HASH[*c as usize][pos],
            Piece::Pawn(c) => P_HASH[*c as usize][pos],
            Piece::Nil => NIL_HASH[pos],
        }
    }

    pub const fn val(&self, pos: usize) -> i16 {
        match self {
            Rook(c) => ROOKVAL[*c as usize][pos],
            Knight(c) => KNIGHTVAL[*c as usize][pos],
            Bishop(c) => BISHOPVAL[*c as usize][pos],
            King(c) => KINGVAL[*c as usize][pos],
            Queen(c) => QUEENVAL[*c as usize][pos],
            Pawn(c) => PAWNVAL[*c as usize][pos],
            Nil => 0,
        }
    }

    pub const fn from_ascii(c: char) -> Piece {
        let colour = if c.is_ascii_uppercase() { White } else { Black };
        match c.to_ascii_lowercase() {
            'r' => Rook(colour),
            'n' => Knight(colour),
            'b' => Bishop(colour),
            'q' => Queen(colour),
            'k' => King(colour),
            'p' => Pawn(colour),
            ' ' => Nil,
            _ => panic!("can not convert to Piece"),
        }
    }

    pub const fn to_ascii(&self) -> char {
        let p = match self {
            Rook(_) => 'r',
            Knight(_) => 'n',
            Bishop(_) => 'b',
            Queen(_) => 'q',
            King(_) => 'k',
            Pawn(_) => 'p',
            Nil => '.',
        };
        if self.is_white() {
            p.to_ascii_uppercase()
        } else {
            p
        }
    }

    pub const fn to_unicode(&self) -> char {
        match self {
            Rook(White) => '\u{2656}',
            Knight(White) => '\u{2658}',
            Bishop(White) => '\u{2657}',
            Queen(White) => '\u{2655}',
            King(White) => '\u{2654}',
            Pawn(White) => '\u{2659}',
            Rook(Black) => '\u{265C}',
            Knight(Black) => '\u{265E}',
            Bishop(Black) => '\u{265D}',
            Queen(Black) => '\u{265B}',
            King(Black) => '\u{265A}',
            Pawn(Black) => '\u{265F}',
            Nil => ' ',
        }
    }

    pub const fn is_white(&self) -> bool {
        matches!(
            self,
            Rook(White) | Knight(White) | Bishop(White) | Queen(White) | King(White) | Pawn(White)
        )
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

pub const ROOT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

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
const KINGVAL: [[i16; 64]; 2] = [KINGVAL_B, KINGVAL_W];

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
const PAWNVAL: [[i16; 64]; 2] = [PAWNVAL_B, PAWNVAL_W];

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
const ROOKVAL: [[i16; 64]; 2] = [ROOKVAL_B, ROOKVAL_W];

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
const KNIGHTVAL: [[i16; 64]; 2] = [KNIGHTVAL_B, KNIGHTVAL_W];

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
const BISHOPVAL: [[i16; 64]; 2] = [BISHOPVAL_B, BISHOPVAL_W];

const QUEENVAL_W: [i16; 64] = [900; 64];
const QUEENVAL_B: [i16; 64] = [-900; 64];
const QUEENVAL: [[i16; 64]; 2] = [QUEENVAL_B, QUEENVAL_W];

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
