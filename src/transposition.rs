use crate::mgen::{self, Move};
use std::collections::HashMap;

//const TABLE_SIZE: usize = 1 << 20; // Example: 2^20 = 1,048,576 entries
//const MASK: usize = TABLE_SIZE - 1;
#[derive(Debug, Copy, Clone)]
pub struct TEntry {
    pub depth: u16,
    pub score: i16,
    data: u16, // frm, to, bound: 2x6 bits + 3 bits
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
}

pub struct Transpositions(HashMap<u64, TEntry>);

impl Default for Transpositions {
    fn default() -> Transpositions {
        Transpositions(HashMap::new())
    }
}

impl Transpositions {
    pub fn store(&mut self, key: u64, depth: u16, score: i16, alpha: i16, beta: i16, m: &Move) {
        // TODO - implement more efficient hashing function
        let bound = if score <= alpha {
            0 // Upper bound
        } else if score >= beta {
            TEntry::LOWER_BIT
        } else {
            TEntry::EXACT_BIT
        };
        let data = (m.data & (mgen::FRM_MASK | mgen::TO_MASK)) | bound;
        let e = TEntry { depth, score, data };
        self.0
            .entry(key)
            .and_modify(|x| {
                if x.depth < e.depth {
                    *x = e;
                }
            })
            .or_insert(e);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn probe(&self, key: &u64) -> Option<&TEntry> {
        self.0.get(key)
    }
}
