use athena::{
    Array, Object, XffValue,
    rng_api::{RngError, RngResult},
    xff,
};

use crate::entropy_pool::generator::shuffle_pool;

mod generator;
mod pools;

/// `EntropyPool` is a struct that contains a pool of pseudo random bytes.
/// The pool will regenerate itself if it is empty.
///
/// Panic save.
///
/// Constructed with a seed vector via [`EntropyPool::new`].
pub struct EntropyPool {
    pool: [u8; 256],
    current_pool: u8,
    pool_index: u8,
    seeded_iteration: u32,
    seed: Vec<u8>,
}

impl EntropyPool {
    /// Creates a new `EntropyPool`.
    ///
    /// Please supply a seed vector with at least one element.
    /// The more the better.
    pub fn new<V: Into<Vec<u8>>>(seed: V) -> Self {
        let seed = seed.into();
        let pool_num = if let Some(first) = seed.first() {
            *first
        } else {
            0
        };
        let seed = if seed.is_empty() { vec![0] } else { seed };
        println!("pool_num: {}", pool_num);
        println!("seed_len: {}", seed.len());
        println!("is_seed_empty: {}", seed.is_empty());
        let pool = shuffle_pool(pool_num, &seed);
        Self {
            pool,
            current_pool: pool_num,
            pool_index: 0,
            seeded_iteration: 0,
            seed,
        }
    }

    /// Converts the `EntropyPool` to a `XffValue`.
    ///
    /// Useful for serialization if the state of the `EntropyPool` is needed or wanted for re-initialization.
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("pool_index", self.pool_index);
        obj.insert("current_pool", self.current_pool);
        obj.insert("seeded_iteration", self.seeded_iteration);
        let mut seed = Array::new();
        for s in &self.seed {
            seed.push(*s);
        }
        obj.insert("seed", seed);
        xff!(obj)
    }

    /// Converts the `XffValue` to a `EntropyPool`.
    ///
    /// Useful for deserialization if the state of the `EntropyPool` is needed or wanted for re-initialization.
    ///
    /// Will error if the `XffValue` is not a valid `EntropyPool`.
    ///
    /// Use [`EntropyPool::to_xff`] to convert a `EntropyPool` to an `XffValue`.
    pub fn from_xff(xff: &XffValue) -> RngResult<Self> {
        if let Some(obj) = xff.as_object()
            && let Some(pool_index) = obj.get("pool_index")
            && let Some(pool_index) = pool_index.as_number()
            && let Some(pool_index) = pool_index.into_usize()
            && let Some(current_pool) = obj.get("current_pool")
            && let Some(current_pool) = current_pool.as_number()
            && let Some(current_pool) = current_pool.into_usize()
            && let Some(seeded_iteration) = obj.get("seeded_iteration")
            && let Some(seeded_iteration) = seeded_iteration.as_number()
            && let Some(seeded_iteration) = seeded_iteration.into_usize()
            && let Some(seed) = obj.get("seed")
            && let Some(seed) = seed.as_array()
        {
            let pool_index = pool_index as u8;
            let seeded_iteration = seeded_iteration as u32;
            let current_pool = current_pool as u8;
            let seed: Vec<u8> = seed
                .iter()
                .map(|s| s.as_number().unwrap().into_usize().unwrap() as u8)
                .collect();
            let seed = if seed.is_empty() { vec![0] } else { seed };
            let pool = shuffle_pool(current_pool, &seed);
            Ok(EntropyPool {
                pool,
                current_pool,
                pool_index,
                seeded_iteration,
                seed,
            })
        } else {
            Err(RngError::Generic(format!(
                "Cannot create EntropyPool from passed XffValue: '{:?}'",
                xff
            )))
        }
    }

    /// Returns a byte from the pool.
    /// If the pool is empty, it will regenerate the pool.
    pub fn get_random_byte(&mut self) -> u8 {
        let out = self.pool[self.pool_index as usize];
        self.pool_index += 1;
        if self.pool_index == 255 {
            self.regenerate_pool();
        }
        out
    }

    /// Regenerates the pool.
    fn regenerate_pool(&mut self) {
        self.pool_index = 0;
        let next_seeds = self
            .seeded_iteration
            .overflowing_add(self.current_pool as u32)
            .0
            .to_le_bytes();
        self.seeded_iteration = self.seeded_iteration.wrapping_add(1);
        let next_pool = self.current_pool.wrapping_add(self.seeded_iteration as u8) % 255;
        self.current_pool = next_pool;
        if self.seed.len() < 256 {
            self.seed.extend(next_seeds);
        } else {
            if self.seeded_iteration < 512 && self.seeded_iteration % 128 == 0 {
                self.seed.extend(next_seeds);
            }
        }
        self.pool = shuffle_pool(next_pool, &self.seed);
    }
}
