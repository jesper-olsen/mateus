use std::fmt;
use Piece::*;

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

impl Piece {
    pub const fn pval(&self, pos: usize) -> i16 {
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
            _ => 0,
        }
    }
}

pub const END_GAME_MATERIAL: i16 = abs_material(&ROOT_BOARD) / 3;

pub const fn abs_material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += board[i].pval(i).abs();
        i += 1;
    }
    val
}

pub const fn material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += board[i].pval(i);
        i += 1;
    }
    val
}

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

// Lasker position - test for transposition table - winning move Ka1-b1
pub const LASKER_FEN: &str = "8/k7/3p4/p2Pawn(WHITE)p2/Pawn(BLACK)Pawn(WHITE)Pawn(BLACK)/8/8/K7"; //, "w - -", "Kb1")

fn feni(i: usize) -> usize {
    let x = 7 - i % 8;
    let y = 7 - i / 8;
    x * 8 + y
}

pub fn fen2board(s: &str) -> [Piece; 64] {
    let mut a = [Nil; 64];
    let mut offset = 0i16;
    for (i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            offset += d as i16 - 1;
        } else if c == '/' {
            offset -= 1;
        } else {
            let k: usize = (i as i16 + offset).try_into().unwrap();
            let q = feni(k);
            a[q] = match c {
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
                _ => panic!("invalid fen"),
            }
        }
    }
    a
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Rook(WHITE) => write!(f, "R"),
            Rook(BLACK) => write!(f, "r"),
            Knight(WHITE) => write!(f, "N"),
            Knight(BLACK) => write!(f, "n"),
            Bishop(WHITE) => write!(f, "B"),
            Bishop(BLACK) => write!(f, "b"),
            Queen(WHITE) => write!(f, "Q"),
            Queen(BLACK) => write!(f, "q"),
            King(WHITE) => write!(f, "K"),
            King(BLACK) => write!(f, "k"),
            Pawn(WHITE) => write!(f, "P"),
            Pawn(BLACK) => write!(f, "p"),
            _ => write!(f, "."),
        }
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
