use alloc::vec::Vec;
use core::hash::{Hash, BuildHasher};
use super::hash::RandomState;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
    hash_builder: RandomState,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: (0..16).map(|_| Vec::new()).collect(),
            len: 0,
            hash_builder: RandomState::new(),
        }
    }

    fn hash(&self, key: &K) -> u64 {
        self.hash_builder.hash_one(key)
    }

    fn bucket_index(&self, key: &K) -> usize {
        (self.hash(key) % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, val: V) {
        let idx = self.bucket_index(&key);
        self.buckets[idx].push((key, val));
        self.len += 1;
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            buckets: &self.buckets,
            bucket_index: 0,
            entry_index: 0,
        }
    }
}

pub struct Iter<'a, K, V> {
    buckets: &'a [Vec<(K, V)>],
    bucket_index: usize,
    entry_index: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bucket_index == self.buckets.len() {
            return None;
        }
        let bucket = &self.buckets[self.bucket_index];
        if &self.entry_index == &bucket.len() {
            self.bucket_index += 1;
            self.entry_index = 0;
            return self.next();
        }
        let (key, val) = &bucket[self.entry_index];
        self.entry_index += 1;
        Some((key, val))
    }
}