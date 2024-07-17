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

#[rustfmt::skip]
const R1: Piece = Piece::Rook(WHITE);
const N1: Piece = Piece::Knight(WHITE);
const B1: Piece = Piece::Bishop(WHITE);
const K1: Piece = Piece::King(WHITE);
const Q1: Piece = Piece::Queen(WHITE);
const P1: Piece = Piece::Pawn(WHITE);
const R2: Piece = Piece::Rook(BLACK);
const N2: Piece = Piece::Knight(BLACK);
const B2: Piece = Piece::Bishop(BLACK);
const K2: Piece = Piece::King(BLACK);
const Q2: Piece = Piece::Queen(BLACK);
const P2: Piece = Piece::Pawn(BLACK);
const NIL: Piece = Piece::Nil;

pub const fn pval(p: Piece, pos: usize) -> i16 {
    match p {
        R1 => ROOKVAL1[pos],
        R2 => ROOKVAL2[pos],
        N1 => KNIGHTVAL1[pos],
        N2 => KNIGHTVAL2[pos],
        B1 => BISHOPVAL1[pos],
        B2 => BISHOPVAL2[pos],
        K1 => KINGVAL1[pos],
        K2 => KINGVAL2[pos],
        Q1 => QUEENVAL1[pos],
        Q2 => QUEENVAL2[pos],
        P1 => PAWNVAL1[pos],
        P2 => PAWNVAL2[pos],
        _ => 0,
    }
}

pub const END_GAME_MATERIAL: i16 = abs_material(&ROOT_BOARD) / 3;

pub const fn abs_material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += pval(board[i], i).abs();
        i += 1;
    }
    val
}

pub const fn material(board: &[Piece; 64]) -> i16 {
    let mut i = 0;
    let mut val: i16 = 0;
    while i < board.len() {
        val += pval(board[i], i);
        i += 1;
    }
    val
}

#[rustfmt::skip]
pub const ROOT_BOARD: [Piece; 64] = [
    R1, P1, NIL, NIL, NIL, NIL, P2, R2, 
    N1, P1, NIL, NIL, NIL, NIL, P2, N2, 
    B1, P1, NIL, NIL, NIL, NIL, P2, B2, 
    K1, P1, NIL, NIL, NIL, NIL, P2, K2, 
    Q1, P1, NIL, NIL, NIL, NIL, P2, Q2, 
    B1, P1, NIL, NIL, NIL, NIL, P2, B2, 
    N1, P1, NIL, NIL, NIL, NIL, P2, N2, 
    R1, P1, NIL, NIL, NIL, NIL, P2, R2,
];

pub const ROOT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

// Lasker position - test for transposition table - winning move Ka1-b1
pub const LASKER_FEN: &str = "8/k7/3p4/p2P1p2/P2P1P2/8/8/K7"; //, "w - -", "Kb1")

fn feni(i: usize) -> usize {
    let x = 7 - i % 8;
    let y = 7 - i / 8;
    x * 8 + y
}

pub fn fen2board(s: &str) -> [Piece; 64] {
    let mut a = [NIL; 64];
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
                'r' => R2,
                'n' => N2,
                'b' => B2,
                'q' => Q2,
                'k' => K2,
                'p' => P2,
                'R' => R1,
                'N' => N1,
                'B' => B1,
                'Q' => Q1,
                'K' => K1,
                'P' => P1,
                _ => panic!("invalid fen"),
            }
        }
    }
    a
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            R1 => write!(f, "R"),
            R2 => write!(f, "r"),
            N1 => write!(f, "N"),
            N2 => write!(f, "n"),
            B1 => write!(f, "B"),
            B2 => write!(f, "b"),
            Q1 => write!(f, "Q"),
            Q2 => write!(f, "q"),
            K1 => write!(f, "K"),
            K2 => write!(f, "k"),
            P1 => write!(f, "P"),
            P2 => write!(f, "p"),
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
