use crate::val::*;

// We use const_random! to create hash keys at compile time.
// Our hash keys are u64, which const_random supports.
// For arrays, however, const_random only supports u8 arrays,
// so we need to define our own macros to make that convenient...

macro_rules! add_elems {
    ($entry:ident $($tokens:tt)*) => {
        add_elems!{0, $entry $($tokens)*}
    };
    ($acc:expr, $entry:ident $token:tt $($tokens:tt)*) => {
        add_elems!{2*$acc, $entry $($tokens)*}
        add_elems!{2*$acc + 1, $entry $($tokens)*}
    };
    ($count:expr, $entry:ident) => {
        $entry[$count] = const_random::const_random!(u64);
    };
}

macro_rules! hashkey {
    () => {{
        let mut arr: [u64; 64] = [0; 64];
        add_elems!(arr ######);
        arr
    }}
}

const R1_HASH: [u64; 64] = hashkey!();
const R2_HASH: [u64; 64] = hashkey!();
const N1_HASH: [u64; 64] = hashkey!();
const N2_HASH: [u64; 64] = hashkey!();
const B1_HASH: [u64; 64] = hashkey!();
const B2_HASH: [u64; 64] = hashkey!();
const K1_HASH: [u64; 64] = hashkey!();
const K2_HASH: [u64; 64] = hashkey!();
const Q1_HASH: [u64; 64] = hashkey!();
const Q2_HASH: [u64; 64] = hashkey!();
const P1_HASH: [u64; 64] = hashkey!();
const P2_HASH: [u64; 64] = hashkey!();
const NIL_HASH: [u64; 64] = hashkey!();
pub const WHITE_HASH: u64 = const_random::const_random!(u64);

pub const fn phashkey(p: Piece, pos: usize) -> u64 {
    match p {
        R1 => R1_HASH[pos],
        R2 => R2_HASH[pos],
        N1 => N1_HASH[pos],
        N2 => N2_HASH[pos],
        B1 => B1_HASH[pos],
        B2 => B2_HASH[pos],
        K1 => K1_HASH[pos],
        K2 => K2_HASH[pos],
        Q1 => Q1_HASH[pos],
        Q2 => Q2_HASH[pos],
        P1 => P1_HASH[pos],
        P2 => P2_HASH[pos],
        _ => NIL_HASH[pos],
    }
}

pub const fn board2hash(board: &[Piece; 64], colour: bool) -> u64 {
    let mut key = match colour {
        WHITE => WHITE_HASH,
        BLACK => 0,
    };

    let mut i = 0;
    while i < 64 {
        match board[i] {
            NIL => (),
            _ => key ^= phashkey(board[i], i),
        };
        i += 1;
    }
    key
}
