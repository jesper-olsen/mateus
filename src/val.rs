use Piece::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Piece {
    Rook(bool),
    Knight(bool),
    Bishop(bool),
    Queen(bool),
    King(bool),
    Pawn(bool),
    Nil,
}
pub const WHITE: bool = true;
pub const BLACK: bool = false;

pub static PIECES: [Piece; 12] = [
    Pawn(WHITE),
    Rook(WHITE),
    Knight(WHITE),
    Bishop(WHITE),
    Queen(WHITE),
    King(WHITE),
    Pawn(BLACK),
    Rook(BLACK),
    Knight(BLACK),
    Bishop(BLACK),
    Queen(BLACK),
    King(BLACK),
];

impl Piece {
    pub const fn val(&self, pos: usize) -> i16 {
        match self {
            Rook(WHITE) => ROOKVAL1[pos],
            Rook(BLACK) => ROOKVAL2[pos],
            Knight(WHITE) => KNIGHTVAL1[pos],
            Knight(BLACK) => KNIGHTVAL2[pos],
            Bishop(WHITE) => BISHOPVAL1[pos],
            Bishop(BLACK) => BISHOPVAL2[pos],
            King(WHITE) => KINGVAL1[pos],
            King(BLACK) => KINGVAL2[pos],
            Queen(WHITE) => QUEENVAL1[pos],
            Queen(BLACK) => QUEENVAL2[pos],
            Pawn(WHITE) => PAWNVAL1[pos],
            Pawn(BLACK) => PAWNVAL2[pos],
            Nil => 0,
        }
    }

    pub const fn from_ascii(c: char) -> Piece {
        match c {
            'r' => Rook(BLACK),
            'n' => Knight(BLACK),
            'b' => Bishop(BLACK),
            'q' => Queen(BLACK),
            'k' => King(BLACK),
            'p' => Pawn(BLACK),
            'R' => Rook(WHITE),
            'N' => Knight(WHITE),
            'B' => Bishop(WHITE),
            'Q' => Queen(WHITE),
            'K' => King(WHITE),
            'P' => Pawn(WHITE),
            ' ' => Nil,
            _ => panic!("can not convert to Piece"),
        }
    }

    pub const fn to_ascii(&self) -> char {
        match self {
            Rook(WHITE) => 'R',
            Rook(BLACK) => 'r',
            Knight(WHITE) => 'N',
            Knight(BLACK) => 'n',
            Bishop(WHITE) => 'B',
            Bishop(BLACK) => 'b',
            Queen(WHITE) => 'Q',
            Queen(BLACK) => 'q',
            King(WHITE) => 'K',
            King(BLACK) => 'k',
            Pawn(WHITE) => 'P',
            Pawn(BLACK) => 'p',
            Nil => '.',
        }
    }

    pub const fn to_unicode(&self) -> char {
        match self {
            Rook(WHITE) => '\u{2656}',
            Knight(WHITE) => '\u{2658}',
            Bishop(WHITE) => '\u{2657}',
            Queen(WHITE) => '\u{2655}',
            King(WHITE) => '\u{2654}',
            Pawn(WHITE) => '\u{2659}',
            Rook(BLACK) => '\u{265C}',
            Knight(BLACK) => '\u{265E}',
            Bishop(BLACK) => '\u{265D}',
            Queen(BLACK) => '\u{265B}',
            King(BLACK) => '\u{265A}',
            Pawn(BLACK) => '\u{265F}',
            Nil => ' ',
        }
    }

    pub const fn is_white(&self) -> bool {
        matches!(
            self,
            Rook(WHITE) | Knight(WHITE) | Bishop(WHITE) | Queen(WHITE) | King(WHITE) | Pawn(WHITE)
        )
    }
}

pub const END_GAME_MATERIAL: i16 = abs_material(&ROOT_BOARD) / 3;

pub const fn abs_material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += board[i].val(i).abs();
        i += 1;
    }
    val
}

pub const fn material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += board[i].val(i);
        i += 1;
    }
    val
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

#[rustfmt::skip]
pub const ROOT_BOARD: [Piece; 64] = [
    Rook(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Rook(BLACK), 
    Knight(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Knight(BLACK), 
    Bishop(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Bishop(BLACK), 
    King(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), King(BLACK), 
    Queen(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Queen(BLACK), 
    Bishop(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Bishop(BLACK), 
    Knight(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Knight(BLACK), 
    Rook(WHITE), Pawn(WHITE), Nil, Nil, Nil, Nil, Pawn(BLACK), Rook(BLACK),
];

pub const ROOT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ascii())
    }
}

#[rustfmt::skip]
pub const KINGVAL1 : [i16;64] = [
   24,  24,  12,  6,  6,  12,  24,  24, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   24,  24,  12,  6,  6,  12,  24,  24];
pub const KINGVAL2: [i16; 64] = array_mul(-1, KINGVAL1);

#[rustfmt::skip]
pub const PAWNVAL1 : [i16;64] = [
  100, 100, 101, 102, 104, 106, 108, 900, 
  100, 100, 102, 104, 106, 109, 112, 900, 
  100, 100, 104, 108, 112, 115, 118, 900, 
  100, 100, 107, 114, 121, 128, 135, 900, 
  100, 100, 106, 112, 118, 124, 132, 900, 
  100, 100, 104, 108, 112, 116, 120, 900, 
  100, 100, 102, 104, 106, 108, 112, 900, 
  100, 100, 101, 102, 104, 106, 108, 900];
pub const PAWNVAL2: [i16; 64] = array_reverse(array_mul(-1, PAWNVAL1));

#[rustfmt::skip]
pub const ROOKVAL1 : [i16;64] = [
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500];
pub const ROOKVAL2: [i16; 64] = array_reverse(array_mul(-1, ROOKVAL1));

#[rustfmt::skip]
pub const KNIGHTVAL1 : [i16;64] = [
  315, 315, 315, 315, 315, 315, 315, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 315, 315, 315, 315, 315, 315, 315];
pub const KNIGHTVAL2: [i16; 64] = array_reverse(array_mul(-1, KNIGHTVAL1));

#[rustfmt::skip]
pub const BISHOPVAL1: [i16;64] = [
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350];
pub const BISHOPVAL2: [i16; 64] = array_mul(-1, array_reverse(BISHOPVAL1));

pub const QUEENVAL1: [i16; 64] = [900; 64];
pub const QUEENVAL2: [i16; 64] = [-900; 64];

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
