use crate::mgen::{self, Move};
use static_assertions::const_assert;

// Ensure usize is at least 64-bit at compile time
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u64>());

// Table size - examples
// 2 ^  0 =          1
// 2 ^  1 =          2
// 2 ^  2 =          4
// 2 ^  3 =          8
// 2 ^  4 =         16
// 2 ^  5 =         32
// 2 ^  6 =         64
// 2 ^  7 =        128
// 2 ^  8 =        256
// 2 ^ 20 =    1048576 =   1M
// 2 ^ 21 =    2097152 =   2M
// 2 ^ 22 =    4194304 =   4M
// 2 ^ 23 =    8388608 =   8M
// 2 ^ 24 =   16777216 =  16M
// 2 ^ 25 =   33554432 =  34M
// 2 ^ 26 =   67108864 =  64M
// 2 ^ 27 =  134217728 = 128M
// 2 ^ 28 =  268435456 = 256M
// 2 ^ 29 =  536870912 = 512M
// 2 ^ 30 = 1073741824 =   1G

const N_INDEX_BITS: usize = 24;
const TABLE_SIZE: usize = 1 << N_INDEX_BITS;
const MASK: usize = TABLE_SIZE - 1;

const N_REMINDER_BITS: usize = 64 - N_INDEX_BITS;
const N_REMINDER_BYTES: usize = (N_REMINDER_BITS + 7) / 8;
const N_ENTRY_BYTES: usize = N_REMINDER_BYTES + 5;

// N_INDEX_BITS is 24 => TABLE_SIZE = 16M
//                       N_REMINDER_BITS = 40
//                       N_REMINDER_BYTES = 5
//                       N_ENTRY_BYTES = 10
// Entry: N_REMINDER_BYTES (5) + Depth (1) + score (2) + move data (2 bytes / 14 bits)

const fn reminder_to_slice(hash_key: u64, array: &mut [u8]) {
    let reminder = hash_key >> N_INDEX_BITS;
    let bytes = reminder.to_le_bytes();

    let mut i = 0;
    while i < N_REMINDER_BYTES {
        array[i] = bytes[i];
        i += 1;
    }
}

const fn reminder_from_slice(array: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    let mut i = 0;
    while i < N_REMINDER_BYTES {
        bytes[i] = array[i];
        i += 1;
    }
    u64::from_le_bytes(bytes)
}

const fn i16_to_slice(value: i16, array: &mut [u8]) {
    let bytes = value.to_le_bytes();
    array[0] = bytes[0];
    array[1] = bytes[1];
}

const fn u16_to_slice(value: u16, array: &mut [u8]) {
    let bytes = value.to_le_bytes();
    array[0] = bytes[0];
    array[1] = bytes[1];
}

const fn u16_from_slice(array: &[u8]) -> u16 {
    u16::from_le_bytes([array[0], array[1]])
}

const fn i16_from_slice(array: &[u8]) -> i16 {
    i16::from_le_bytes([array[0], array[1]])
}

#[derive(Debug, Copy, Clone)]
pub struct TEntry {
    rmdata: [u8; N_ENTRY_BYTES],
}

impl Default for TEntry {
    #[inline(always)]
    fn default() -> TEntry {
        TEntry {
            rmdata: [0; N_ENTRY_BYTES],
        }
    }
}

impl TEntry {
    // Score is either exact, a lower bound or an upper bound
    const EXACT_BIT: u16 = 1 << 12;
    const LOWER_BIT: u16 = 1 << 13;
    #[inline(always)]
    pub fn exact_bound(&self) -> bool {
        let data = u16_from_slice(&self.rmdata[N_REMINDER_BYTES + 3..]);
        data & TEntry::EXACT_BIT != 0
    }
    #[inline(always)]
    pub fn lower_bound(&self) -> bool {
        let data = u16_from_slice(&self.rmdata[N_REMINDER_BYTES + 3..]);
        data & TEntry::LOWER_BIT != 0
    }

    #[inline(always)]
    pub fn frmto(&self) -> (u8, u8) {
        let data = u16_from_slice(&self.rmdata[N_REMINDER_BYTES + 3..]);
        (mgen::ext_frm(data), mgen::ext_to(data))
    }

    #[inline(always)]
    pub fn frmto2(&self) -> (u8, u8) {
        let data = u16_from_slice(&self.rmdata[N_REMINDER_BYTES + 3..]);
        (mgen::ext_frm(data), mgen::ext_to(data))
    }

    // #[inline(always)]
    // pub fn depth(&self) -> u8 {
    //     self.depth
    // }

    #[inline(always)]
    pub fn depth(&self) -> u8 {
        self.rmdata[N_REMINDER_BYTES]
    }

    // #[inline(always)]
    // pub fn score(&self) -> i16 {
    //     self.score
    // }

    #[inline(always)]
    pub fn score(&self) -> i16 {
        i16_from_slice(&self.rmdata[N_REMINDER_BYTES + 1..])
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
    pub fn store(&mut self, key: u64, depth: u8, score: i16, alpha: i16, beta: i16, m: Move) {
        let bound = if score <= alpha {
            0 // Upper bound
        } else if score >= beta {
            TEntry::LOWER_BIT
        } else {
            TEntry::EXACT_BIT
        };
        let move_data = (m.data & (mgen::FRM_MASK | mgen::TO_MASK)) | bound;
        let e = &mut self.0[index(key)];
        reminder_to_slice(key, &mut e.rmdata);
        e.rmdata[N_REMINDER_BYTES] = depth;
        i16_to_slice(score, &mut e.rmdata[N_REMINDER_BYTES + 1..]);
        u16_to_slice(move_data, &mut e.rmdata[N_REMINDER_BYTES + 3..]);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.fill(TEntry::default());
    }

    pub fn probe(&self, key: u64) -> Option<&TEntry> {
        let entry = &self.0[index(key)];
        let reminder = key >> N_INDEX_BITS;
        if reminder == reminder_from_slice(&entry.rmdata) {
            Some(entry)
        } else {
            None
        }
    }
}
