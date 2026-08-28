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
pub fn shuffle_pool(pool_num: u8, seed: &[u8]) -> [u8; 255] {
    let mut pool = get_pool(pool_num).to_vec();
    debug_assert!(pool.len() == 255, "Pool must be 255 bytes long");
    for seed_byte in seed.iter() {
        let idx1 = if *seed_byte == 255 {
            1
        } else {
            *seed_byte as usize
        };
        pool.swap(
            idx1,
            255usize.saturating_sub(*seed_byte as usize).clamp(0, 254),
        );
    }
    match pool.try_into() {
        Ok(pool) => pool,
        Err(_) => unreachable!("Pool didn't stay 255 bytes long by doing only swaps?!"),
    }
}
