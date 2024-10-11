mod generator;
mod cpu_features;

use generator::generate_entropy_pool;

pub struct EntropyPool {
    pool: Vec<u8>,
    restricted_pool: bool,
    initial_pool_size: usize,
    salted: bool,
    salt: Option<Vec<u8>>,
    with_entropy: bool,
    entropy: Vec<u8>,
}

impl EntropyPool {
    pub fn new() -> Self {
        let pool = Self::generate_pool();
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
            salted: false,
            salt: None,
            with_entropy: false,
            entropy: Vec::new(),
        }
    }

    pub fn get_random_byte(&mut self) -> u8 {
        if self.pool.is_empty() {
            self.regenerate_pool();
        }
        *self.pool.last().unwrap()
    }

    pub fn create_size_restricted(initial_pool_size: usize)-> Self {
        let pool = {
let large_pool = generate_entropy_pool(false, None, false, Vec::new())
        };
        Self {
            initial_pool_size,
            pool,
            restricted_pool: true,
            salted: false,
            salt: None,
            with_entropy: false,
            entropy: Vec::new(),
        }
    }

    pub fn create_salted(salt: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(true, Some(salt), false, Vec::new());
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
        }
    }

    pub fn create_with_entropy(entropy: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(false, None, true, entropy);
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
        }
    }

    pub fn create_salted_with_entropy(salt: Vec<u8>, entropy: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(true, Some(salt), true, entropy);
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
        }
    }

    pub fn create_custom(initial_pool_size: usize, restricted_pool_size: bool, custom_salt: Vec<u8>, custom_entropy: Vec<u8>) -> Self {
        let pool = generate_entropy_pool();
    }

    fn regenerate_pool(&mut self) {
        if self.restricted_pool {
            let large_pool = Self::generate_pool();
            // take the first initial_pool_size elements
            self.pool = large_pool[0..self.initial_pool_size].to_vec();
        } else {
            self.pool = Self::generate_pool();
        }
    }

    fn generate_pool() -> Vec<u8> {
        generate_entropy_pool(false, None, false, Vec::new())
    }

    fn generate_restricted_pool(initial_pool_size: usize) -> Vec<u8> {
        let large_pool = Self::generate_pool();
        large_pool[0..initial_pool_size].to_vec()
    }
}
