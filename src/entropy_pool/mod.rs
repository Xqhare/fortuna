mod cpu_features;
mod generator;

#[cfg(test)]
mod tests;

use generator::generate_entropy_pool;

/// `EntropyPool` is a struct that contains a pool of pseudo random bytes.
/// The pool will regenerate itself if it is empty.
pub struct EntropyPool {
    pool: Vec<u8>,
    pool_index: usize,
    restricted_pool: bool,
    initial_pool_size: usize,
}

impl EntropyPool {
    /// Creates a new `EntropyPool`.
    pub fn new() -> Self {
        let pool = generate_entropy_pool();
        Self {
            initial_pool_size: pool.len(),
            pool,
            pool_index: 0,
            restricted_pool: false,
        }
    }

    /// Returns a byte from the pool.
    /// If the pool is empty, it will regenerate the pool.
    pub fn get_random_byte(&mut self) -> u8 {
        if self.initial_pool_size == self.pool_index + 1 {
            self.regenerate_pool();
        }
        let out = self.pool[self.pool_index];
        self.pool_index += 1;
        out
    }

    /// Creates a new `EntropyPool` with a restricted pool size.
    pub fn create_size_restricted(initial_pool_size: usize) -> Self {
        let pool = generate_restricted_pool(initial_pool_size);
        Self {
            initial_pool_size,
            pool,
            pool_index: 0,
            restricted_pool: true,
        }
    }

    fn regenerate_pool(&mut self) {
        if self.restricted_pool {
            self.pool = generate_restricted_pool(self.initial_pool_size);
            self.pool_index = 0;
        } else {
            self.pool = generate_entropy_pool();
            self.initial_pool_size = self.pool.len();
            self.pool_index = 0;
        }
    }
}

fn generate_restricted_pool(initial_pool_size: usize) -> Vec<u8> {
    let large_pool = generate_entropy_pool();
    if large_pool.len() < initial_pool_size {
        let mut enlarged_pool = large_pool;
        while enlarged_pool.len() < initial_pool_size {
            enlarged_pool.append(&mut generate_entropy_pool());
        }
        enlarged_pool[0..initial_pool_size].to_vec()
    } else {
        large_pool[0..initial_pool_size].to_vec()
    }
}
