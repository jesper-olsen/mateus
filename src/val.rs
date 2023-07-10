use std::fmt;

// +9 +1 -7
// +8    -8
// +7 -1 -9

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
    Nil,
}
pub const WHITE: bool = true;
pub const BLACK: bool = false;

//#[rustfmt::skip]
//mod unformatted {
//}
//use unformatted::*;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Piece {
    pub ptype: PType,
    pub colour: bool,
}

#[rustfmt::skip]
pub const R1: Piece = Piece {ptype: PType::Rook, colour: WHITE};
pub const R2: Piece = Piece {
    ptype: PType::Rook,
    colour: BLACK,
};
pub const N1: Piece = Piece {
    ptype: PType::Knight,
    colour: WHITE,
};
pub const N2: Piece = Piece {
    ptype: PType::Knight,
    colour: BLACK,
};
pub const B1: Piece = Piece {
    ptype: PType::Bishop,
    colour: WHITE,
};
pub const B2: Piece = Piece {
    ptype: PType::Bishop,
    colour: BLACK,
};
pub const K1: Piece = Piece {
    ptype: PType::King,
    colour: WHITE,
};
pub const K2: Piece = Piece {
    ptype: PType::King,
    colour: BLACK,
};
pub const Q1: Piece = Piece {
    ptype: PType::Queen,
    colour: WHITE,
};
pub const Q2: Piece = Piece {
    ptype: PType::Queen,
    colour: BLACK,
};
pub const P1: Piece = Piece {
    ptype: PType::Pawn,
    colour: WHITE,
};
pub const P2: Piece = Piece {
    ptype: PType::Pawn,
    colour: BLACK,
};
pub const NIL: Piece = Piece {
    ptype: PType::Nil,
    colour: BLACK,
};

pub const fn pval(p: Piece, pos: usize) -> i32 {
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

pub const fn abs_material(board: &[Piece; 64]) -> i32 {
    let mut i = 0;
    let mut val: i32 = 0;
    while i < board.len() {
        val += pval(board[i], i).abs();
        i += 1;
    }
    val
}

pub const fn material(board: &[Piece; 64]) -> i32 {
    let mut i = 0;
    let mut val: i32 = 0;
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

#[rustfmt::skip]
pub const _TEST_BOARD: [Piece; 64] = [
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL, 
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL, 
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL, 
    K1,  P1, NIL, NIL, NIL, NIL, P2, K2, 
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL,
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL,
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL, 
    NIL, P1, NIL, NIL, NIL, NIL, P2, NIL,
];

// Lasker position - winning move Ka1-b1!!
// Test for transpotion table
#[rustfmt::skip]
pub const _TEST_BOARD2: [Piece; 64] = [
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 
    NIL, NIL, NIL, P1,  P2,  NIL, NIL, NIL,
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 
    NIL, NIL, NIL, P1,  P1,  P2,  NIL, NIL, 
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 
    K1,  NIL, NIL, P1,  P2,  NIL, K2,  NIL, 
];

pub const END_GAME_MATERIAL: i32 = abs_material(&ROOT_BOARD) / 3;

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
pub const KINGVAL1 : [i32;64] = [
   24,  24,  12,  6,  6,  12,  24,  24, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
    6,   0,  -6, -12, -12, -6,  0,  6, 
   12,  6,   0,  -6, -6,  0,   6,  12, 
   24,  12,  6,   0,  0,  6,   12,  24, 
   24,  24,  12,  6,  6,  12,  24,  24];
pub const KINGVAL2: [i32; 64] = array_mul(-1, KINGVAL1);

#[rustfmt::skip]
pub const PAWNVAL1 : [i32;64] = [
  100, 100, 101, 102, 104, 106, 108, 900, 
  100, 100, 102, 104, 106, 109, 112, 900, 
  100, 100, 104, 108, 112, 115, 118, 900, 
  100, 100, 107, 114, 121, 128, 135, 900, 
  100, 100, 106, 112, 118, 124, 132, 900, 
  100, 100, 104, 108, 112, 116, 120, 900, 
  100, 100, 102, 104, 106, 108, 112, 900, 
  100, 100, 101, 102, 104, 106, 108, 900];
pub const PAWNVAL2: [i32; 64] = array_reverse(array_mul(-1, PAWNVAL1));

#[rustfmt::skip]
pub const ROOKVAL1 : [i32;64] = [
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500, 
  500, 500, 500, 500, 500, 500, 522, 500];
pub const ROOKVAL2: [i32; 64] = array_reverse(array_mul(-1, ROOKVAL1));

#[rustfmt::skip]
pub const KNIGHTVAL1 : [i32;64] = [
  315, 315, 315, 315, 315, 315, 315, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 325, 325, 330, 330, 320, 315, 
  315, 320, 320, 320, 320, 320, 320, 315, 
  315, 315, 315, 315, 315, 315, 315, 315];
pub const KNIGHTVAL2: [i32; 64] = array_reverse(array_mul(-1, KNIGHTVAL1));

#[rustfmt::skip]
pub const BISHOPVAL1: [i32;64] = [
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350,
   339, 350, 350, 350, 350, 350, 350, 350];
pub const BISHOPVAL2: [i32; 64] = array_mul(-1, array_reverse(BISHOPVAL1));

pub const QUEENVAL1: [i32; 64] = [900; 64];
pub const QUEENVAL2: [i32; 64] = [-900; 64];

const fn array_mul<const N: usize>(factor: i32, mut a: [i32; N]) -> [i32; N] {
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
