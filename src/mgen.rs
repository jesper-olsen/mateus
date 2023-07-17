use crate::bitmaps::*;
use crate::hashkeys::phashkey;
use crate::val::*;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move {
    pub frm: usize,
    pub to: usize,
    pub castle: bool,
    pub transform: (Piece, Piece),
    pub val: i32,
    pub kill: (Piece, usize),
    pub hash: u64,
}

pub const NULL_MOVE: Move = Move {
    frm: 0,
    to: 0,
    castle: false,
    transform: (NIL, NIL),
    val: 0,
    kill: (NIL, 0),
    hash: 0,
};

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x1 = 7 - self.frm / 8;
        let y1 = self.frm % 8 + 1;
        let x2 = 7 - self.to / 8;
        let y2 = self.to % 8 + 1;
        let s = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        write!(f, "{}{} {}{}", s[x1], y1, s[x2], y2)
    }
}

// count pseudo legal moves - ignoring enpassant & castling
pub fn count_moves(board: &[Piece; 64], colour: bool, bm_white: u64, bm_black: u64) -> u32 {
    let (bm_own, bm_opp) = if colour == WHITE {
        (bm_white, bm_black)
    } else {
        (bm_black, bm_white)
    };
    let bm_board = bm_white | bm_black;

    board
        .iter()
        .enumerate()
        .filter(|(_, &p)| p != NIL && p.colour == colour)
        .map(|(frm, &p)| match p {
            N1 | N2 => (BM_KNIGHT_MOVES[frm] & !bm_own).count_ones(),
            K1 | K2 => (BM_KING_MOVES[frm] & !bm_own).count_ones(),
            P1 | P2 => count_pawn_moves(frm, bm_opp, bm_board, colour),
            R1 | R2 => count_ray_moves(frm, BM_ROOK_MOVES[frm], bm_board, bm_own),
            B1 | B2 => count_ray_moves(frm, BM_BISHOP_MOVES[frm], bm_board, bm_own),
            Q1 | Q2 => count_ray_moves(frm, BM_QUEEN_MOVES[frm], bm_board, bm_own),
            _ => 0,
        })
        .sum()
}

// +9  +1 -7
// +8   0 -8
// +7  -1 -9

fn count_pawn_moves(frm: usize, bm_opp: u64, bm_board: u64, colour: bool) -> u32 {
    // TODO  - calc all at the same time;
    let cidx = if colour { 0 } else { 1 };
    let cap = BM_PAWN_CAPTURES[cidx][frm] & bm_opp;
    let step1 = BM_PAWN_STEP1[cidx][frm] & !bm_board;
    let step2 = if colour { step1 << 1 } else { step1 >> 1 };
    let step2 = step2 & BM_PAWN_STEP2[cidx][frm] & !bm_board;
    (cap | step1 | step2).count_ones()
}

fn count_ray_moves(frm: usize, moves: u64, bm_board: u64, bm_own: u64) -> u32 {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    (moves & !bl & !bm_own).count_ones()
}

// true if !colour side can capture colour king
pub fn in_check(
    board: &[Piece; 64],
    colour: bool,
    bm_wking: u64,
    bm_bking: u64,
    bm_board: u64,
) -> bool {
    let (cidx, bm_king) = if colour == WHITE {
        (1, bm_wking)
    } else {
        (0, bm_bking)
    };

    board
        .iter()
        .enumerate()
        .filter(|(_, &p)| p != NIL && p.colour != colour)
        .map(|(frm, &p)| match p {
            N1 | N2 => (BM_KNIGHT_MOVES[frm] & bm_king) != 0,
            K1 | K2 => (BM_KING_MOVES[frm] & bm_king) != 0,
            P1 | P2 => BM_PAWN_CAPTURES[cidx][frm] & bm_king != 0,
            R1 | R2 => ray_check(frm, BM_ROOK_MOVES[frm], bm_board, bm_king),
            B1 | B2 => ray_check(frm, BM_BISHOP_MOVES[frm], bm_board, bm_king),
            Q1 | Q2 => ray_check(frm, BM_QUEEN_MOVES[frm], bm_board, bm_king),
            _ => unreachable!(),
        })
        .any(|x| x)
    // .find(|x| *x)
    // .is_some()
}

fn ray_check(frm: usize, moves: u64, bm_board: u64, bm_king: u64) -> bool {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    (moves & !bl & bm_king) != 0
}

pub fn moves(
    board: &[Piece; 64],
    colour: bool,
    end_game: bool,
    can_castle: &[bool; 4],
    last: Option<&Move>,
    bm_white: u64,
    bm_black: u64,
) -> Vec<Move> {
    let (bm_own, bm_opp) = if colour == WHITE {
        (bm_white, bm_black)
    } else {
        (bm_black, bm_white)
    };
    let bm_board = bm_white | bm_black;

    let last = if let Some(m) = last {
        m
    } else {
        // dummy
        &Move {
            frm: 0,
            to: 0,
            castle: false,
            transform: (NIL, NIL),
            kill: (NIL, 0),
            val: 0,
            hash: 0,
        }
    };

    let mut v = Vec::with_capacity(50);
    board
        .iter()
        .enumerate()
        .filter(|(_, &p)| p != NIL && p.colour == colour)
        .map(|(frm, &p)| match p {
            N1 | N2 => knight_moves(&mut v, board, frm, bm_own),
            K1 | K2 => king_moves(
                &mut v, board, frm, bm_own, bm_board, end_game, can_castle, colour,
            ),
            P1 | P2 => pawn_moves(&mut v, board, frm, last, bm_opp, bm_board, colour),
            R1 | R2 => ray_moves(&mut v, board, frm, BM_ROOK_MOVES[frm], bm_board, bm_own),
            B1 | B2 => ray_moves(&mut v, board, frm, BM_BISHOP_MOVES[frm], bm_board, bm_own),
            Q1 | Q2 => ray_moves(&mut v, board, frm, BM_QUEEN_MOVES[frm], bm_board, bm_own),
            _ => unreachable!(),
        })
        .for_each(drop);
    v
}

fn knight_moves(v: &mut Vec<Move>, board: &[Piece; 64], frm: usize, bm_own: u64) {
    v.extend(
        bm2vec(BM_KNIGHT_MOVES[frm] & !bm_own)
            .iter()
            .map(|&to| Move {
                frm,
                to,
                castle: false,
                transform: (NIL, NIL),
                kill: (board[to], to),
                val: pval(board[frm], to) - pval(board[frm], frm) - pval(board[to], to),
                hash: phashkey(board[frm], to)
                    ^ phashkey(board[frm], frm)
                    ^ phashkey(board[to], to),
            }),
    );
}

fn ray_moves(
    v: &mut Vec<Move>,
    board: &[Piece; 64],
    frm: usize,
    moves: u64,
    bm_board: u64,
    bm_own: u64,
) {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    v.extend(bm2vec(moves & !bl & !bm_own).iter().map(|&to| Move {
        frm,
        to,
        castle: false,
        transform: (NIL, NIL),
        kill: (board[to], to),
        val: pval(board[frm], to) - pval(board[frm], frm) - pval(board[to], to),
        hash: phashkey(board[frm], to) ^ phashkey(board[frm], frm) ^ phashkey(board[to], to),
    }));
}

fn pawn_moves(
    v: &mut Vec<Move>,
    board: &[Piece; 64],
    frm: usize,
    last: &Move,
    bm_opp: u64,
    bm_board: u64,
    colour: bool,
) {
    let cidx = if colour { 0 } else { 1 };
    let cap = BM_PAWN_CAPTURES[cidx][frm] & bm_opp;
    let step1: u64 = BM_PAWN_STEP1[cidx][frm] & !bm_board;
    let step2: u64 = if colour { step1 << 1 } else { step1 >> 1 };
    let step2: u64 = step2 & BM_PAWN_STEP2[cidx][frm] & !bm_board;
    let vto = bm2vec(cap | step1 | step2);

    v.extend(vto.iter().map(|&to| Move {
        frm,
        to,
        castle: false,
        transform: transform(to, colour),
        kill: (board[to], to),
        val: pval(board[frm], to) - pval(board[frm], frm) - pval(board[to], to),
        hash: phashkey(board[frm], to) ^ phashkey(board[frm], frm) ^ phashkey(board[to], to),
    }));

    // en passant
    if matches!(board[last.to], P2 | P1) && last.to.abs_diff(last.frm) == 2 {
        // square attacked if last move was a step-2 pawn move
        let idx = if colour { last.frm - 1 } else { last.frm + 1 };

        v.extend(
            bm2vec(BM_PAWN_CAPTURES[cidx][frm] & 1 << idx)
                .iter()
                .map(|&to| Move {
                    frm,
                    to,
                    castle: false,
                    transform: (NIL, NIL),
                    kill: (board[last.to], last.to),
                    val: pval(board[frm], to)
                        - pval(board[frm], frm)
                        - pval(board[last.to], last.to),
                    hash: phashkey(board[frm], to)
                        ^ phashkey(board[frm], frm)
                        ^ phashkey(board[to], to),
                }),
        );
    }
}

fn king_moves(
    v: &mut Vec<Move>,
    board: &[Piece; 64],
    frm: usize,
    bm_own_pieces: u64,
    bm_board: u64,
    end_game: bool,
    can_castle: &[bool; 4],
    colour: bool,
) {
    // change king valuation in end_game
    let p = match (colour, end_game) {
        (WHITE, false) => K1,
        (WHITE, true) => K2,
        (BLACK, false) => K2,
        (BLACK, true) => K1,
    };

    // castling
    // check squares between K & R unoccupied
    const WSHORT: u64 = 1 << 8 | 1 << 16;
    const WLONG: u64 = 1 << 32 | 1 << 40 | 1 << 48;
    const BSHORT: u64 = 1 << 15 | 1 << 23;
    const BLONG: u64 = 1 << 55 | 1 << 47 | 1 << 39;

    #[rustfmt::skip]
    let cc2 = [
        (can_castle[0] && frm == 24 && board[0] == R1 && bm_board & WSHORT == 0,
         K1, R1, 8, 0, 16,),
        (can_castle[1] && frm == 24 && board[56] == R1 && bm_board & WLONG == 0,
         K1, R1, 48, 56, 32,),
        (can_castle[2] && frm == 31 && board[7] == R2 && bm_board & BSHORT == 0,
         K2, R2, 15, 7, 23,),
        (can_castle[3] && frm == 31 && board[63] == R2 && bm_board & BLONG == 0,
         K2, R2, 55, 63, 39,),
    ];

    v.extend(
        bm2vec(BM_KING_MOVES[frm] & !bm_own_pieces)
            .iter()
            .map(|&to| Move {
                frm,
                to,
                castle: false,
                transform: (NIL, NIL),
                kill: (board[to], to),
                val: pval(p, to) - pval(p, frm) - pval(board[to], to),
                hash: phashkey(board[frm], to)
                    ^ phashkey(board[frm], frm)
                    ^ phashkey(board[to], to),
            })
            .chain(
                cc2.iter()
                    .filter(|(c, _, _, _, _, _)| *c)
                    .map(|(_, k, r, to, rfrm, rto)| Move {
                        frm,
                        to: *to,
                        castle: true,
                        transform: (NIL, NIL),
                        kill: (NIL, *to),
                        val: pval(p, *to) - pval(p, frm) + pval(*r, *rto) - pval(*r, *rfrm),
                        hash: phashkey(*k, *to)
                            ^ phashkey(*k, frm)
                            ^ phashkey(*r, *rto)
                            ^ phashkey(*r, *rfrm),
                    }),
            ),
    );
}

fn transform(to: usize, colour: bool) -> (Piece, Piece) {
    match (colour, to % 8) {
        (WHITE, 7) => (P1, Q1),
        (BLACK, 0) => (P2, Q2),
        _ => (NIL, NIL),
    }
}

// const fn board2bm(board: &[Piece; 64]) -> u64 {
//     let mut b: u64 = 0;
//     let mut i = 0;
//     while i < 64 {
//         match board[i] {
//             Piece {
//                 ptype: PType::Nil, ..
//             } => (),
//             _ => b |= 1 << i,
//         }
//         i += 1;
//     }
//     b
// }

pub const fn board2bm_colour(board: &[Piece; 64], colour: bool) -> u64 {
    let mut b: u64 = 0;
    let mut i = 0;
    while i < 64 {
        match (board[i], colour) {
            (R2 | N2 | B2 | K2 | Q2 | P2, BLACK) => b |= 1 << i,
            (R1 | N1 | B1 | K1 | Q1 | P1, WHITE) => b |= 1 << i,
            _ => (),
        }
        i += 1;
    }
    b
}

// pub fn board2bm_piece(board: &[Piece; 64], p: Piece) -> u64 {
//     let mut b: u64 = 0;
//     let mut i = 0;
//     while i < 64 {
//         if board[i] == p {
//             b |= 1 << i;
//         }
//         i += 1;
//     }
//     b
// }

pub const fn board2bm_white_pawns(board: &[Piece; 64]) -> u64 {
    let mut b: u64 = 0;
    let mut i = 0;
    while i < 64 {
        match board[i] {
            P1 => b |= 1 << i,
            _ => (),
        }
        i += 1;
    }
    b
}

pub const fn board2bm_black_pawns(board: &[Piece; 64]) -> u64 {
    let mut b: u64 = 0;
    let mut i = 0;
    while i < 64 {
        match board[i] {
            P2 => b |= 1 << i,
            _ => (),
        }
        i += 1;
    }
    b
}
