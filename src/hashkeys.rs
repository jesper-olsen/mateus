use crate::hashkeys_generated::*;
use crate::val::*;

// We use const_random! to create hash keys at compile time.
// Our hash keys are u64, which const_random supports.

// macro_rules! add_elems {
//     ($entry:ident $($tokens:tt)*) => {
//         add_elems!{0, $entry $($tokens)*}
//     };
//     ($acc:expr, $entry:ident $token:tt $($tokens:tt)*) => {
//         add_elems!{2*$acc, $entry $($tokens)*}
//         add_elems!{2*$acc + 1, $entry $($tokens)*}
//     };
//     ($count:expr, $entry:ident) => {
//         $entry[$count] = const_random::const_random!(u64);
//     };
// }

// macro_rules! hashkey {
//     () => {{
//         let mut arr: [u64; 64] = [0; 64];
//         add_elems!(arr ######);
//         arr
//     }}
// }

// const R1_HASH: [u64; 64] = hashkey!();
// const R2_HASH: [u64; 64] = hashkey!();
// const N1_HASH: [u64; 64] = hashkey!();
// const N2_HASH: [u64; 64] = hashkey!();
// const B1_HASH: [u64; 64] = hashkey!();
// const B2_HASH: [u64; 64] = hashkey!();
// const K1_HASH: [u64; 64] = hashkey!();
// const K2_HASH: [u64; 64] = hashkey!();
// const Q1_HASH: [u64; 64] = hashkey!();
// const Q2_HASH: [u64; 64] = hashkey!();
// const P1_HASH: [u64; 64] = hashkey!();
// const P2_HASH: [u64; 64] = hashkey!();
// const NIL_HASH: [u64; 64] = hashkey!();
// pub const WHITE_HASH: u64 = const_random::const_random!(u64);

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
}

pub const fn board2hash(board: &[Piece; 64], colour: Colour) -> u64 {
    let mut key = match colour {
        Colour::White => WHITE_HASH,
        Colour::Black => 0,
    };

    let mut i = 0;
    while i < 64 {
        match board[i] {
            Piece::Nil => (),
            _ => key ^= board[i].hashkey(i),
        };
        i += 1;
    }
    key
}
