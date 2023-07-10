pub const BM_BLOCKED: [[u64; 64]; 64] = bm_queen_blockers();
pub const BM_QUEEN_MOVES: [u64; 64] = bm_queen_moves();
pub const BM_BISHOP_MOVES: [u64; 64] = bm_bishop_moves();
pub const BM_ROOK_MOVES: [u64; 64] = bm_rook_moves();
pub const BM_KNIGHT_MOVES: [u64; 64] = bm_knight_moves();
pub const BM_KING_MOVES: [u64; 64] = bm_king_moves();
pub const BM_PAWN_CAPTURES: [[u64; 64]; 2] = bm_pawn_captures();
pub const BM_PAWN_STEP1: [[u64; 64]; 2] = bm_pawn_step1();
pub const BM_PAWN_STEP2: [[u64; 64]; 2] = bm_pawn_step2();

const fn bm_queen_blockers() -> [[u64; 64]; 64] {
    let mut bm = [[0; 64]; 64];

    let mut frm = 0;
    while frm < 64 {
        let mut blocked = 0;
        while blocked < 64 {
            if frm != blocked {
                bm[frm][blocked] = bm_queen_moves_bb(frm, blocked);
            }
            blocked += 1;
        }
        frm += 1;
    }
    bm
}

// set_bit k: b |= 1<<k
// clear_bit k: b &= !(1<<k)
// is_set k: b & 1<<k

pub fn bm_display(b: u64) {
    println!();
    for y in (0..8).rev() {
        print!("{} ", y + 1);
        for x in 0..8 {
            let x = if b & 1 << ((7 - x) * 8 + y) != 0 {
                'x'
            } else {
                '.'
            };
            print!("{}", x);
        }
        println!();
    }
    println!("  ABCDEFGH");
}

const fn bm_queen_moves() -> [u64; 64] {
    let mut bm = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        bm[i] = BM_BISHOP_MOVES[i] | BM_ROOK_MOVES[i];
        i += 1;
    }
    bm
}

const fn bm_bishop_moves() -> [u64; 64] {
    let mut bm = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        bm[i] = bm_bishop_moves_from(i);
        i += 1;
    }
    bm
}

const fn min_usize(i1: usize, i2: usize) -> usize {
    if i1 <= i2 {
        i1
    } else {
        i2
    }
}

const fn bm_bishop_moves_from(frm: usize) -> u64 {
    let mut b = 0;

    // NW
    let mut i = frm + 9;
    while i < min_usize(64, frm + (8 - frm % 8) * 9) {
        b |= 1 << i;
        i += 9;
    }

    // NE
    let mut i = frm - 7 * min_usize(frm / 8, 7 - frm % 8);
    while i < frm {
        b |= 1 << i;
        i += 7;
    }

    // SW
    let mut i = frm + 7;
    while i < min_usize(64, frm + frm % 8 * 7 + 1) {
        b |= 1 << i;
        i += 7;
    }

    // SE
    let mut i = frm - 9 * min_usize(frm / 8, frm % 8);
    while i < frm {
        b |= 1 << i;
        i += 9;
    }

    b
}
const fn bm_queen_moves_bb(frm: usize, blocked: usize) -> u64 {
    let mut b = 0;

    let frm_x = frm / 8;
    let frm_y = frm % 8;
    let blocked_x = blocked / 8;
    let blocked_y = blocked % 8;

    if frm_x == blocked_x {
        if frm_y < blocked_y {
            // rook blocked N
            let mut i = blocked + 1;
            while i % 8 > 0 {
                b |= 1 << i;
                i += 1;
            }
        } else {
            // rook blocked S
            let mut i = blocked as isize - 1;
            while i % 8 < frm_y as isize && i >= 0 {
                b |= 1 << i;
                i -= 1;
            }
        }
    } else if frm_y == blocked_y {
        if frm_x < blocked_x {
            // rook blocked W
            let mut i = blocked + 8;
            while i < 64 {
                b |= 1 << i;
                i += 8;
            }
        } else {
            // rook blocked E
            let mut i = blocked as isize - 8;
            while i >= 0 {
                b |= 1 << i;
                i -= 8;
            }
        }
    } else if frm_x.abs_diff(blocked_x) == frm_y.abs_diff(blocked_y) {
        if frm_x > blocked_x {
            if frm_y < blocked_y {
                // bishop blocked NE
                let mut i = blocked as isize - 7;
                while i % 8 <= 7 && i >= 0 {
                    b |= 1 << i;
                    i -= 7;
                }
            } else {
                // bishop blocked SE
                let mut i = blocked as isize - 9;
                while i % 8 >= 0 && i >= 0 {
                    b |= 1 << i;
                    i -= 9;
                }
            }
        } else if frm_y % 8 < blocked_y % 8 {
            //bishop blocked NW
            let mut i = blocked + 9;
            while i % 8 > frm_y && i < 64 {
                b |= 1 << i;
                i += 9;
            }
        } else {
            //bishop blocked SW
            let mut i = blocked + 7;
            while i % 8 < frm_y && i < 64 {
                b |= 1 << i;
                i += 7;
            }
        }
    }
    b
}

const fn bm_pawn_step2() -> [[u64; 64]; 2] {
    let mut bm = [[0u64; 64]; 2];
    let mut i = 0;
    while i < 64 {
        bm[0][i] = bm_white_pawn_step2_from(i);
        bm[1][i] = bm_black_pawn_step2_from(i);
        i += 1;
    }
    bm
}

const fn bm_white_pawn_step2_from(frm: usize) -> u64 {
    if frm % 8 == 1 {
        1 << (frm + 2)
    } else {
        0
    }
}

const fn bm_black_pawn_step2_from(frm: usize) -> u64 {
    if frm % 8 == 6 {
        1 << (frm - 2)
    } else {
        0
    }
}

///
const fn bm_pawn_step1() -> [[u64; 64]; 2] {
    let mut bm = [[0u64; 64]; 2];
    let mut i = 0;
    while i < 64 {
        bm[0][i] = bm_white_pawn_step1_from(i);
        bm[1][i] = bm_black_pawn_step1_from(i);
        i += 1;
    }
    bm
}

const fn bm_white_pawn_step1_from(frm: usize) -> u64 {
    if frm % 8 != 7 {
        1 << (frm + 1)
    } else {
        0
    }
}

const fn bm_black_pawn_step1_from(frm: usize) -> u64 {
    if frm % 8 != 0 {
        1 << (frm - 1)
    } else {
        0
    }
}

const fn bm_pawn_captures() -> [[u64; 64]; 2] {
    let mut bm = [[0u64; 64]; 2];
    let mut i = 0;
    while i < 64 {
        bm[0][i] = bm_white_pawn_captures_from(i);
        bm[1][i] = bm_black_pawn_captures_from(i);
        i += 1;
    }
    bm
}

const fn bm_white_pawn_captures_from(frm: usize) -> u64 {
    let mut b = 0;
    if frm < 56 && (frm + 9) % 8 != 0 {
        b |= 1 << (frm + 9)
    }
    if frm > 7 && (frm - 7) % 8 != 0 {
        b |= 1 << (frm - 7)
    }
    b
}

const fn bm_black_pawn_captures_from(frm: usize) -> u64 {
    let mut b = 0;
    if frm % 8 != 0 && frm < 56 {
        b |= 1 << (frm + 7)
    }
    if frm % 8 != 0 && frm >= 9 {
        b |= 1 << (frm - 9)
    }
    b
}

const fn bm_rook_moves() -> [u64; 64] {
    let mut bm = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        bm[i] = bm_rook_moves_from(i);
        i += 1;
    }
    bm
}

const fn bm_rook_moves_from(frm: usize) -> u64 {
    let mut b = 0;

    // West
    let mut i = frm + 8;
    while i < 7 * 8 + 1 + frm % 8 {
        b |= 1 << i;
        i += 8;
    }

    // East
    let mut i = frm as isize - 8;
    while i > 0 {
        b |= 1 << i;
        i -= 8;
    }

    // North
    let mut i = frm + 1;
    while i < (frm / 8) * 8 + 8 {
        b |= 1 << i;
        i += 1;
    }

    // South
    let mut i = (frm / 8) * 8;
    while i < frm {
        b |= 1 << i;
        i += 1;
    }

    b
}

const fn bm_knight_moves() -> [u64; 64] {
    bm_set_squares(&[
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
        (2, 1),
        (1, 2),
    ])
}

const fn bm_king_moves() -> [u64; 64] {
    bm_set_squares(&[
        (1, 1),
        (0, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ])
}

const fn bm_set_squares(rmoves: &[(isize, isize)]) -> [u64; 64] {
    let mut bm: [u64; 64] = [0; 64];

    let mut i: isize = 0;
    while i < 64 {
        let mut j: usize = 0;
        while j < rmoves.len() {
            let (a, b) = rmoves[j];
            let a = a + i / 8;
            let b = b + i % 8;
            if a >= 0 && a <= 7 && b >= 0 && b <= 7 {
                bm[i as usize] |= 1 << (a * 8 + b);
            }
            j += 1;
        }
        i += 1;
    }

    bm
}

pub fn bm2vec(bm: u64) -> Vec<usize> {
    // 29.7 sec
    // let b=BM_KNIGHT_MOVES[i];
    // let mut l=Vec::<usize>::new();
    // for i in 0..64 {
    //     if b & 1<<i != 0 {
    //         l.push(i as usize);
    //     }
    // }

    // 23.1 sec
    // let b=BM_KNIGHT_MOVES[i];
    // let mut l=Vec::<usize>::new();
    // for i in b.trailing_zeros()..64-b.leading_zeros() {
    //     if b & 1<<i != 0 {
    //         l.push(i as usize);
    //     }
    // }

    //let mut b: u64 =BM_KNIGHT_MOVES[i];
    let mut b = bm;
    let mut l = Vec::<usize>::new();
    while b != 0 {
        let i = b.trailing_zeros(); // 20.5 sec
        b &= !(1 << i);
        l.push(i as usize);
    }
    l

    // let mut b: u64 =BM_KNIGHT_MOVES[i];
    // let mut l=Vec::<usize>::new();
    // while b!=0 {
    //     let i = 63-b.leading_zeros();  // 20.8 sec
    //    b &= !(1<<i);
    //     l.push(i as usize);
    // }
    //l

    //  let b=BM_KNIGHT_MOVES[i];           // 23.3 sec
    //  (b.trailing_zeros()..64-b.leading_zeros() )
    //      .filter(|i| b & 1<<i!=0)
    //      .map(|x| x as usize)
    // .collect::<Vec<usize>>()
}
