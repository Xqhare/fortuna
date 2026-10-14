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
            pool_index: 0,
            seeded_iteration: 0,
            seed,
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
        self.seeded_iteration = self.seeded_iteration.wrapping_add(1);
        let next_pool = self.seed[self.seeded_iteration as usize % self.seed.len()];
        if self.seed.len() < 256 {
            self.seed
                .push(next_pool.wrapping_add((self.seeded_iteration % 255) as u8));
        }
        self.pool = shuffle_pool(next_pool, &self.seed);
    }
}
