/// Bitset struct
pub struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    pub fn new_all_true(size: usize) -> Self {
        let num_words = (size + 63).div_ceil(64);
        Self {
            words: vec![!0u64; num_words], // !0u64 sets all 64 bits to 1
        }
    }

    pub fn new_all_false(size: usize) -> Self {
        let num_words = (size + 63).div_ceil(64);
        Self {
            words: vec![0u64; num_words],
        }
    }

    /// Mark an index as `false` (dead)
    pub fn mark_dead(&mut self, idx: usize) {
        self.words[idx / 64] &= !(1u64 << (idx % 64));
    }

    /// Mark an index as `true` (alive)
    pub fn mark_keep(&mut self, idx: usize) {
        self.words[idx / 64] |= 1u64 << (idx % 64);
    }

    /// Read whether index is `true` (keep)
    pub fn is_keep(&self, idx: usize) -> bool {
        (self.words[idx / 64] & (1u64 << (idx % 64))) != 0
    }

    pub fn clear_true(&mut self) {
        self.words.fill(!0u64);
    }

    pub fn clear_false(&mut self) {
        self.words.fill(0u64);
    }
}
