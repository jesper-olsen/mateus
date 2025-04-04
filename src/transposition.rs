use crate::mgen::{self, Move};
use static_assertions::const_assert;

// Ensure usize is at least 64-bit at compile time
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u64>());

// Table size - examples
// 2 ^ 20 =    1048576 =   1M
// 2 ^ 21 =    2097152 =   2M
// 2 ^ 22 =    4194304 =   4M
// 2 ^ 23 =    8388608 =   8M
// 2 ^ 24 =   16777216 =  16M
// 2 ^ 25 =   33554432 =  34M
// 2 ^ 26 =   67108864 =  67M
// 2 ^ 27 =  134217728 = 134M
// 2 ^ 28 =  268435456 = 268M
// 2 ^ 29 =  536870912 = 537M
// 2 ^ 30 = 1073741824 =   1G

const TABLE_SIZE: usize = 1 << 23;
const MASK: usize = TABLE_SIZE - 1;
#[derive(Debug, Copy, Clone)]
pub struct TEntry {
    key: u64,
    depth: u16,
    score: i16,
    data: u16, // frm, to, bound: 2x6 + 2 = 14 bits
               // 2x16 + 14 = 46
               // 64-46 = 18
}

impl Default for TEntry {
    #[inline(always)]
    fn default() -> TEntry {
        TEntry {
            key: 0,
            depth: 0,
            score: 0,
            data: 0,
        }
    }
}

impl TEntry {
    // Score is either exact, a lower bound or an upper bound
    const EXACT_BIT: u16 = 1 << 12;
    const LOWER_BIT: u16 = 1 << 13;
    #[inline(always)]
    pub fn exact_bound(&self) -> bool {
        self.data & TEntry::EXACT_BIT != 0
    }
    #[inline(always)]
    pub fn lower_bound(&self) -> bool {
        self.data & TEntry::LOWER_BIT != 0
    }

    #[inline(always)]
    pub fn frmto(&self) -> (u8, u8) {
        (mgen::ext_frm(self.data), mgen::ext_to(self.data))
    }

    #[inline(always)]
    pub fn depth(&self) -> u16 {
        self.depth
    }

    #[inline(always)]
    pub fn score(&self) -> i16 {
        self.score
    }
}

pub struct Transpositions(Vec<TEntry>);

impl Default for Transpositions {
    fn default() -> Transpositions {
        Transpositions(vec![TEntry::default(); TABLE_SIZE])
    }
}

#[inline(always)]
fn index(key: u64) -> usize {
    key as usize & MASK
}

impl Transpositions {
    pub fn store(&mut self, key: u64, depth: u16, score: i16, alpha: i16, beta: i16, m: &Move) {
        let bound = if score <= alpha {
            0 // Upper bound
        } else if score >= beta {
            TEntry::LOWER_BIT
        } else {
            TEntry::EXACT_BIT
        };
        let data = (m.data & (mgen::FRM_MASK | mgen::TO_MASK)) | bound;
        let e = TEntry {
            key,
            depth,
            score,
            data,
        };

        self.0[index(key)] = e
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.fill(TEntry::default());
    }

    pub fn probe(&self, key: u64) -> Option<&TEntry> {
        let entry = &self.0[index(key)];
        if entry.key == key { Some(entry) } else { None }
    }
}
