use core::hash::{Hasher, BuildHasher};
use arceos_api::modules::axhal::misc::random;

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// FNV-1a 算法实现的 Hasher (By GPT-6.2)
pub struct RandomHasher {
    state: u64,
}

impl RandomHasher {
    pub fn new(seed: u64) -> Self {
        Self {
            state: FNV_OFFSET_BASIS ^ seed,
        }
    }
}

impl Hasher for RandomHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(Clone)]
pub struct RandomState {
    seed: u64,
}

impl RandomState {
    pub fn new() -> Self {
        Self {
            seed: random() as u64,
        }
    }
}

impl Default for RandomState {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = RandomHasher;

    fn build_hasher(&self) -> Self::Hasher {
        RandomHasher::new(self.seed)
    }
}