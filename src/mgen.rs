use crate::bitmaps::*;
use crate::val::Piece::*;
use crate::val::*;
use std::fmt;

const fn pack_flags(castle: bool, en_passant: bool, transform: bool) -> (bool, bool, bool) {
    (castle, en_passant, transform)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move {
    flags: (bool, bool, bool),
    frmto: (u8, u8),
    pub val: i16,
    pub hash: u64,
}

impl Move {
    pub fn castle(&self) -> bool {
        self.flags.0
    }
    pub fn en_passant(&self) -> bool {
        self.flags.1
    }
    pub fn transform(&self) -> bool {
        self.flags.2
    }
    pub fn frm(&self) -> usize {
        self.frmto.0 as usize
    }
    pub fn to(&self) -> usize {
        self.frmto.1 as usize
    }
}

pub const NULL_MOVE: Move = Move {
    frmto: (0, 0),
    flags: pack_flags(false, false, false),
    val: 0,
    hash: 0,
};

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (frm, to) = self.frmto;
        let x1 = 7 - frm / 8;
        let y1 = frm % 8 + 1;
        let x2 = 7 - to / 8;
        let y2 = to % 8 + 1;
        let s = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        write!(f, "{}{} {}{}", s[x1 as usize], y1, s[x2 as usize], y2)
    }
}

// count pseudo legal moves - ignoring en passant & castling
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
        .map(|(frm, &p)| match p {
            Knight(c) if c == colour => (BM_KNIGHT_MOVES[frm] & !bm_own).count_ones(),
            King(c) if c == colour => (BM_KING_MOVES[frm] & !bm_own).count_ones(),
            Pawn(c) if c == colour => count_pawn_moves(frm, bm_opp, bm_board, colour),
            Rook(c) if c == colour => count_ray_moves(frm, BM_ROOK_MOVES[frm], bm_board, bm_own),
            Bishop(c) if c == colour => {
                count_ray_moves(frm, BM_BISHOP_MOVES[frm], bm_board, bm_own)
            }
            Queen(c) if c == colour => count_ray_moves(frm, BM_QUEEN_MOVES[frm], bm_board, bm_own),
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
    board
        .iter()
        .enumerate()
        .map(|(frm, &p)| match (colour, p) {
            (WHITE, Knight(BLACK)) => (BM_KNIGHT_MOVES[frm] & bm_wking) != 0,
            (BLACK, Knight(WHITE)) => (BM_KNIGHT_MOVES[frm] & bm_bking) != 0,
            (WHITE, King(BLACK)) => (BM_KING_MOVES[frm] & bm_wking) != 0,
            (BLACK, King(WHITE)) => (BM_KING_MOVES[frm] & bm_bking) != 0,
            (WHITE, Pawn(BLACK)) => BM_PAWN_CAPTURES[1][frm] & bm_wking != 0,
            (BLACK, Pawn(WHITE)) => BM_PAWN_CAPTURES[0][frm] & bm_bking != 0,
            (WHITE, Rook(BLACK)) => ray_check(frm, BM_ROOK_MOVES[frm], bm_board, bm_wking),
            (BLACK, Rook(WHITE)) => ray_check(frm, BM_ROOK_MOVES[frm], bm_board, bm_bking),
            (WHITE, Bishop(BLACK)) => ray_check(frm, BM_BISHOP_MOVES[frm], bm_board, bm_wking),
            (BLACK, Bishop(WHITE)) => ray_check(frm, BM_BISHOP_MOVES[frm], bm_board, bm_bking),
            (WHITE, Queen(BLACK)) => ray_check(frm, BM_QUEEN_MOVES[frm], bm_board, bm_wking),
            (BLACK, Queen(WHITE)) => ray_check(frm, BM_QUEEN_MOVES[frm], bm_board, bm_bking),
            _ => false,
        })
        .any(|x| x)
}

fn ray_check(frm: usize, moves: u64, bm_board: u64, bm_king: u64) -> bool {
    let bl: u64 = bm2vec(moves & bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    (moves & !bl & bm_king) != 0
}

struct Bitmaps {
    bm_board: u64,
    bm_own: u64,
    bm_opp: u64,
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

    let bitmaps = Bitmaps {
        bm_board,
        bm_own,
        bm_opp,
    };

    let last = if let Some(m) = last { m } else { &NULL_MOVE };

    let mut v = Vec::with_capacity(50);
    board.iter().enumerate().for_each(|(frm, &p)| match p {
        Knight(c) if c == colour => knight_moves(&mut v, board, frm, &bitmaps),
        King(c) if c == colour => king_moves(&mut v, board, frm, &bitmaps, end_game, can_castle),
        Pawn(c) if c == colour => pawn_moves(&mut v, board, frm, last, &bitmaps, colour),
        Rook(c) if c == colour => ray_moves(&mut v, board, frm, BM_ROOK_MOVES[frm], &bitmaps),
        Bishop(c) if c == colour => ray_moves(&mut v, board, frm, BM_BISHOP_MOVES[frm], &bitmaps),
        Queen(c) if c == colour => ray_moves(&mut v, board, frm, BM_QUEEN_MOVES[frm], &bitmaps),
        _ => (),
    });
    v
}

fn knight_moves(v: &mut Vec<Move>, board: &[Piece; 64], frm: usize, bitmaps: &Bitmaps) {
    v.extend(
        bm2vec(BM_KNIGHT_MOVES[frm] & !bitmaps.bm_own)
            .iter()
            .map(|&to| Move {
                frmto: (frm as u8, to as u8),
                flags: pack_flags(false, false, false),
                val: board[frm].val(to) - board[frm].val(frm) - board[to].val(to),
                hash: board[frm].hashkey(to) ^ board[frm].hashkey(frm) ^ board[to].hashkey(to),
            }),
    );
}

fn ray_moves(v: &mut Vec<Move>, board: &[Piece; 64], frm: usize, moves: u64, bitmaps: &Bitmaps) {
    let bl: u64 = bm2vec(moves & bitmaps.bm_board)
        .iter()
        .fold(0, |a, i| a | BM_BLOCKED[frm][*i]);
    v.extend(
        bm2vec(moves & !bl & !bitmaps.bm_own)
            .iter()
            .map(|&to| Move {
                frmto: (frm as u8, to as u8),
                flags: pack_flags(false, false, false),
                val: board[frm].val(to) - board[frm].val(frm) - board[to].val(to),
                hash: board[frm].hashkey(to) ^ board[frm].hashkey(frm) ^ board[to].hashkey(to),
            }),
    );
}

fn pawn_moves(
    v: &mut Vec<Move>,
    board: &[Piece; 64],
    frm: usize,
    last: &Move,
    bitmaps: &Bitmaps,
    colour: bool,
) {
    let cidx = if colour { 0 } else { 1 };
    let cap = BM_PAWN_CAPTURES[cidx][frm] & bitmaps.bm_opp;
    let step1: u64 = BM_PAWN_STEP1[cidx][frm] & !bitmaps.bm_board;
    let step2: u64 = if colour { step1 << 1 } else { step1 >> 1 };
    let step2: u64 = step2 & BM_PAWN_STEP2[cidx][frm] & !bitmaps.bm_board;
    let vto = bm2vec(cap | step1 | step2);

    v.extend(vto.iter().map(|&to| Move {
        frmto: (frm as u8, to as u8),
        flags: pack_flags(false, false, to % 8 == 7 || to % 8 == 0),
        val: board[frm].transform(to).val(to) - board[frm].val(frm) - board[to].val(to),
        hash: board[frm].transform(to).hashkey(to)
            ^ board[frm].hashkey(frm)
            ^ board[to].hashkey(to),
    }));

    // en passant
    if matches!(board[last.to()], Pawn(_)) && last.to().abs_diff(last.frm()) == 2 {
        // square attacked if last move was a step-2 pawn move
        let idx = if colour {
            last.frm() - 1
        } else {
            last.frm() + 1
        };

        v.extend(
            bm2vec(BM_PAWN_CAPTURES[cidx][frm] & 1 << idx)
                .iter()
                .map(|&to| Move {
                    frmto: (frm as u8, to as u8),
                    flags: pack_flags(false, true, false),
                    val: board[frm].val(to) - board[frm].val(frm) - board[last.to()].val(last.to()),
                    hash: board[frm].hashkey(to) ^ board[frm].hashkey(frm) ^ board[to].hashkey(to),
                }),
        );
    }
}

fn king_moves(
    v: &mut Vec<Move>,
    board: &[Piece; 64],
    frm: usize,
    bitmaps: &Bitmaps,
    end_game: bool,
    can_castle: &[bool; 4],
) {
    // change king valuation in end_game
    let p = match (board[frm], end_game) {
        (King(WHITE), false) => King(WHITE),
        (King(WHITE), true) => King(BLACK),
        (King(BLACK), false) => King(BLACK),
        (King(BLACK), true) => King(WHITE),
        _ => panic!(),
    };

    // castling
    // check squares between K & R unoccupied
    const WSHORT: u64 = 1 << 8 | 1 << 16;
    const WLONG: u64 = 1 << 32 | 1 << 40 | 1 << 48;
    const BSHORT: u64 = 1 << 15 | 1 << 23;
    const BLONG: u64 = 1 << 55 | 1 << 47 | 1 << 39;

    #[rustfmt::skip]
    let cc2 = [
        (can_castle[0] && frm == 24 && board[0] == Rook(WHITE) && bitmaps.bm_board & WSHORT == 0,
         King(WHITE), Rook(WHITE), 8, 0, 16,),
        (can_castle[1] && frm == 24 && board[56] == Rook(WHITE) && bitmaps.bm_board & WLONG == 0,
         King(WHITE), Rook(WHITE), 48, 56, 32,),
        (can_castle[2] && frm == 31 && board[7] == Rook(BLACK) && bitmaps.bm_board & BSHORT == 0,
         King(BLACK), Rook(BLACK), 15, 7, 23,),
        (can_castle[3] && frm == 31 && board[63] == Rook(BLACK) && bitmaps.bm_board & BLONG == 0,
         King(BLACK), Rook(BLACK), 55, 63, 39,),
    ];

    v.extend(
        bm2vec(BM_KING_MOVES[frm] & !bitmaps.bm_own)
            .iter()
            .map(|&to| Move {
                frmto: (frm as u8, to as u8),
                flags: pack_flags(false, false, false),
                //castle: false,
                //en_passant: false,
                //transform: false,
                val: p.val(to) - p.val(frm) - board[to].val(to),
                hash: board[frm].hashkey(to) ^ board[frm].hashkey(frm) ^ board[to].hashkey(to),
            })
            .chain(
                cc2.iter()
                    .filter(|(c, _, _, _, _, _)| *c)
                    .map(|(_, k, r, to, rfrm, rto)| Move {
                        frmto: (frm as u8, *to),
                        flags: pack_flags(true, false, false),
                        val: p.val(*to as usize) - p.val(frm) + r.val(*rto) - r.val(*rfrm),
                        hash: k.hashkey(*to as usize)
                            ^ k.hashkey(frm)
                            ^ r.hashkey(*rto)
                            ^ r.hashkey(*rfrm),
                    }),
            ),
    );
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

pub const fn board2bm(board: &[Piece; 64]) -> (u64, u64) {
    let (mut w, mut b): (u64, u64) = (0, 0);
    let mut i = 0;
    while i < 64 {
        match board[i] {
            Rook(BLACK) | Knight(BLACK) | Bishop(BLACK) | King(BLACK) | Queen(BLACK)
            | Pawn(BLACK) => b |= 1 << i,
            Rook(WHITE) | Knight(WHITE) | Bishop(WHITE) | King(WHITE) | Queen(WHITE)
            | Pawn(WHITE) => w |= 1 << i,
            _ => (),
        }
        i += 1;
    }
    (w, b)
}

pub const fn board2bm_pawns(board: &[Piece; 64]) -> u64 {
    let mut b: u64 = 0;
    let mut i = 0;
    while i < 64 {
        match board[i] {
            Pawn(_) => b |= 1 << i,
            _ => (),
        }
        i += 1;
    }
    b
}
