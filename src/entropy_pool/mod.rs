
pub struct EntropyPool {
    pool: Vec<u8>,
    restricted_pool_size: bool,
    initial_pool_size: usize,
}

impl EntropyPool {
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            restricted_pool_size: false,
            initial_pool_size: 0,
        }
    }

    pub fn get_random_byte(&mut self) -> u8 {
        if self.pool.is_empty() {
            self.regenerate_pool();
        }
        *self.pool.last().unwrap()
    }

    fn regenerate_pool(&mut self) {
        self.pool = Vec::new();
    }

    fn generate_pool() -> Vec<u8> {
        todo!();
    }
}
