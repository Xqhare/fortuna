mod generator;
mod cpu_features;

#[cfg(test)]
mod tests;

use generator::generate_entropy_pool;

/// `EntropyPool` is a struct that contains a pool of pseudo random bytes.
/// The pool will regenerate itself if it is empty.
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
    /// Creates a new `EntropyPool`.
    pub fn new() -> Self {
        let pool = Self::generate_pool(false, None, false, Vec::new());
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

    /// Returns a byte from the pool.
    /// If the pool is empty, it will regenerate the pool.
    pub fn get_random_byte(&mut self) -> u8 {
        if self.pool.is_empty() {
            self.regenerate_pool();
        }
        self.pool.remove(self.pool.len() - 1)
    }

    /// Creates a new `EntropyPool` with a restricted pool size.
    pub fn create_size_restricted(initial_pool_size: usize)-> Self {
        let pool = Self::generate_restricted_pool(initial_pool_size, false, None, false, Vec::new());
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

    /// Creates a new `EntropyPool` with a salt.
    pub fn create_salted(salt: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(true, Some(salt.clone()), false, Vec::new());
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
            salted: true,
            salt: Some(salt),
            with_entropy: false,
            entropy: Vec::new(),
        }
    }

    /// Creates a new `EntropyPool` with an entropy.
    pub fn create_with_entropy(entropy: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(false, None, true, entropy.clone());
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
            salted: false,
            salt: None,
            with_entropy: true,
            entropy,
        }
    }

    /// Creates a new `EntropyPool` with a salt and an entropy.
    pub fn create_salted_with_entropy(salt: Vec<u8>, entropy: Vec<u8>) -> Self {
        let pool = generate_entropy_pool(true, Some(salt.clone()), true, entropy.clone());
        Self {
            initial_pool_size: pool.len(),
            pool,
            restricted_pool: false,
            salted: true,
            salt: Some(salt),
            with_entropy: true,
            entropy,
        }
    }

    /// Creates a custom `EntropyPool` with a pool size of `initial_pool_size`, a salt of `custom_salt`, and an entropy of `custom_entropy`.
    /// Please note, if `restricted_pool` is true, the pool will be restricted to
    /// `initial_pool_size` bytes. If `restricted_pool` is false, the pool will be unrestricted and
    /// the value passed to `initial_pool_size` will be ignored.
    pub fn create_custom(initial_pool_size: usize, restricted_pool: bool, custom_salt: Vec<u8>, custom_entropy: Vec<u8>) -> Self {
        let pool = Self::generate_restricted_pool(initial_pool_size, true, Some(custom_salt.clone()), true, custom_entropy.clone());
        Self {
            initial_pool_size,
            pool,
            restricted_pool,
            salted: true,
            salt: Some(custom_salt),
            with_entropy: true,
            entropy: custom_entropy,
        }
    }

    fn regenerate_pool(&mut self) {
        if self.restricted_pool {
            self.pool = Self::generate_restricted_pool(self.initial_pool_size, self.salted, self.salt.clone(), self.with_entropy, self.entropy.clone());
        } else {
            self.pool = self.generate_pool_from_self();
        }
    }

    fn generate_pool_from_self(&self) -> Vec<u8> {
        Self::generate_pool(self.salted, self.salt.clone(), self.with_entropy, self.entropy.clone())
    }

    fn generate_pool(with_salt: bool, custom_salt: Option<Vec<u8>>, with_entropy: bool, custom_entropy: Vec<u8>) -> Vec<u8> {
        generate_entropy_pool(with_salt, custom_salt, with_entropy, custom_entropy)
    }

    fn generate_restricted_pool(initial_pool_size: usize, with_salt: bool, custom_salt: Option<Vec<u8>>, with_entropy: bool, custom_entropy: Vec<u8>) -> Vec<u8> {
        let large_pool = Self::generate_pool(with_salt, custom_salt, with_entropy, custom_entropy);
        large_pool[0..initial_pool_size].to_vec()
    }
}
