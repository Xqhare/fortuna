use crate::entropy_pool::pools::get_pool;

/// Shuffles the pool based on the seed
///
/// # Arguments
/// * `pool_num` - The pool number to shuffle
/// * `seed` - The seed to use
///
/// # Returns
/// The shuffled pool
///
/// # Notes
/// Does one allocation, then only swaps the bytes
///
/// # Panics
/// Panics if the seed is empty
pub fn shuffle_pool(pool_num: u8, seed: &[u8]) -> [u8; 256] {
    let mut pool = get_pool(pool_num).to_vec();
    debug_assert!(pool.len() == 256, "Pool must be 256 bytes long");
    debug_assert!(!seed.is_empty(), "Seed must be at least 1 byte long");
    for (idx, seed_byte) in seed.iter().enumerate() {
        let idx1 = *seed_byte as usize;
        let idx2 = {
            if idx == 0 {
                if seed.is_empty() {
                    panic!("Seed must be at least 1 byte long");
                } else if seed.len() == 1 {
                    pool[seed[0] as usize] as usize
                } else {
                    pool[pool.len().saturating_sub(*seed_byte as usize) % pool.len()] as usize
                }
            } else {
                if idx % 2 == 0 {
                    pool[seed[idx.saturating_sub(1)] as usize] as usize
                } else {
                    pool[seed[idx.saturating_add(1) % seed.len()] as usize] as usize
                }
            }
        };
        pool.swap(idx1, idx2);
    }
    match pool.try_into() {
        Ok(pool) => pool,
        Err(_) => unreachable!("Pool didn't stay 256 bytes long by doing only swaps?!"),
    }
}
