#[doc = include_str!("../README.md")]
mod entropy_pool;

use crate::entropy_pool::EntropyPool;
use athena::XffValue;
pub use athena::rng_api::{RngApi, RngError, RngResult};
use std::ops::{Add, Sub};

/// Recommended Seed lengths for all use cases
pub mod seed_lengths {
    /// The minimum seed length.
    ///
    /// Choosing a value lower than this will lead to errors being returned.
    ///
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const MIN: usize = 1;
    /// Any seed should be at least this long.
    ///
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const RECOMMENDED_MIN: usize = 32;
    /// Using this seed length is recommended if performance is preferred over perceived randomness.
    ///
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const PERFORMANCE: usize = 64;
    /// Using this seed length is recommended if a balance between performance and perceived randomness is desired.
    ///
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const BALANCE: usize = 128;
    /// Using this seed length is recommended if perceived randomness is preferred over performance.
    ///
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const RANDOM: usize = 256;
    /// Using this seed length is recommended if the seed consists only or mostly of ASCII characters.
    ///
    /// Consider using [`helpers::ascii_seed_normaliser`] to normalise the seed to a larger range of bytes.
    /// Also consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    pub const FROM_ASCII_RECOMMENDED_MAX: usize = 64;
    /// The maximum seed length it is recommend to initialize `Fortuna` with.
    /// Consider using [`helpers::has_recommended_entropy`] to determine if the seed is suitable for `Fortuna`.
    ///
    /// Choosing a value higher than this has diminishing returns on the perceived randomness
    /// while needlessly increasing the time it takes to initialize `Fortuna` and lowering performance of pool re-generations.
    pub const RECOMMENDED_MAX: usize = 256;
}

/// A collection of helper functions
pub mod helpers {
    use std::collections::HashSet;

    use crate::seed_lengths::{RECOMMENDED_MAX, RECOMMENDED_MIN};

    /// `Fortuna` works best when the seed spans all byte values 0..255.
    /// Seeds entered by the user (by keyboard) are most likely to be 32..126 (ascii printable characters)
    ///
    /// This function normalises the seed to cover a larger range of bytes.
    ///
    /// # Arguments
    ///
    /// * `seed` - A vector of characters.
    ///
    /// # Note
    ///
    /// The input vector should be at most `seed_lengths::FROM_ASCII_RECOMMENDED_MAX` in length.
    ///
    /// The seed returned by this function will contain non-printable characters, displaying it will
    /// need special care.
    ///
    /// The returned seed will be of length: `seed.len() * 3 + 32`.
    ///
    /// # Example
    ///
    /// ```
    /// use fortuna::helpers;
    /// use std::collections::HashSet;
    ///
    /// let seed: Vec<char> = "ababcadadcdcdfgjkljgfh".chars().collect();
    /// assert!(!helpers::has_recommended_entropy(seed.clone().into_iter().map(|c| c as u8).collect::<Vec<u8>>()));
    /// let normalised = helpers::ascii_seed_normaliser(seed.clone());
    /// assert!(helpers::has_recommended_entropy(normalised.clone()));
    ///
    /// let seed_unique = HashSet::<u8>::from_iter(seed.iter().map(|c| *c as u8)).len();
    /// let normalised_unique = HashSet::<u8>::from_iter(normalised).len();
    /// assert!(seed_unique < normalised_unique);
    /// ```
    pub fn ascii_seed_normaliser<V: Into<Vec<char>>>(seed: V) -> Vec<u8> {
        let seed = seed
            .into()
            .into_iter()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();
        let missing_range: Vec<u8> = (128..160).collect();
        let mut out = Vec::with_capacity(seed.len().saturating_mul(3).saturating_add(32));
        for v in seed.iter() {
            out.push(*v % 32);
            out.push(*&v.saturating_add(128));
            out.push(*v);
        }
        for v in missing_range.iter() {
            out.insert(out[*v as usize % out.len()] as usize % out.len(), *v);
        }
        out
    }
    /// Convenience function to check if a seed is suitable for `Fortuna`.
    ///
    /// # Arguments
    ///
    /// * `seed` - A vector of bytes.
    ///
    /// # Returns
    ///
    /// * `true` if the seed is suitable for `Fortuna`.
    /// * `false` otherwise.
    ///
    /// # Note
    ///
    /// This function uses the recommended seed lengths found in the [`seed_lengths`] module as
    /// well as the amount of unique byte values present in the seed.
    ///
    /// Of course you can still use a seed that fails these tests, but it does limit the available
    /// entropy severely.
    ///
    /// # Examples
    /// ```rust
    /// use fortuna::helpers;
    ///
    /// let no_entropy_seed: Vec<u8> = "ababcadadcdcdfgjkljgfh".chars().map(|c| c as u8).collect();
    /// assert!(!helpers::has_recommended_entropy(no_entropy_seed));
    /// let low_entropy_seed: Vec<u8> = "In academic writing, the average sentence length is typically between 15-20 words.".chars().map(|c| c as u8).collect();
    /// assert!(helpers::has_recommended_entropy(low_entropy_seed));
    /// let high_entropy_seed: Vec<u8> = "KD6avuOwlHhfSSOAWxCRk8BO1dTwPEDWQb8TkswSM5VbHMUKYCW9SmM9u1cdUkQ2".chars().map(|c| c as u8).collect();
    /// assert!(helpers::has_recommended_entropy(high_entropy_seed));
    /// ```
    pub fn has_recommended_entropy<V: Into<Vec<u8>>>(seed: V) -> bool {
        let seed = seed.into();
        let seed_len = seed.len();
        let set_len = HashSet::<u8>::from_iter(seed).len();
        if seed_len >= RECOMMENDED_MIN
            && seed_len <= RECOMMENDED_MAX
            && set_len >= RECOMMENDED_MIN.saturating_div(2)
            && set_len >= seed_len.saturating_div(3)
        {
            true
        } else {
            false
        }
    }

    #[test]
    fn entropy() {
        let low_entropy_seed: Vec<u8> =
            "In academic writing, the average sentence length is typically between 15-20 words."
                .chars()
                .map(|c| c as u8)
                .collect();
        assert!(crate::helpers::has_recommended_entropy(low_entropy_seed));
        let high_entropy_seed: Vec<u8> =
            "KD6avuOwlHhfSSOAWxCRk8BO1dTwPEDWQb8TkswSM5VbHMUKYCW9SmM9u1cdUkQ2"
                .chars()
                .map(|c| c as u8)
                .collect();
        assert!(crate::helpers::has_recommended_entropy(high_entropy_seed));
        let no_entropy_seed: Vec<u8> = "ababababacdadjangabwgw".chars().map(|c| c as u8).collect();
        assert!(!crate::helpers::has_recommended_entropy(no_entropy_seed));
    }
    #[test]
    fn normaliser() {
        let no_entropy_seed: Vec<char> = "ababababacdadjangabwgw".chars().collect();
        assert!(!crate::helpers::has_recommended_entropy(
            no_entropy_seed
                .iter()
                .map(|c| *c as u8)
                .collect::<Vec<u8>>()
        ));
        let normalised = crate::helpers::ascii_seed_normaliser(no_entropy_seed);
        assert!(crate::helpers::has_recommended_entropy(normalised));
    }
    #[test]
    fn normaliser_increases_expected_range() {
        let seed = "KD6avuOwlHhfSSOAWxCRk8BO1dTwPEDWQb8TkswSM5VbHMUKYCW9SmM9u1cdUkQ2"
            .chars()
            .collect::<Vec<char>>();
        let unique_chars = seed
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<char>>();
        let normalised = crate::helpers::ascii_seed_normaliser(seed);
        let normalised_unique_chars = normalised
            .iter()
            .collect::<std::collections::HashSet<&u8>>();
        assert!(unique_chars.len() < normalised_unique_chars.len());
        assert!(
            u8::MAX as usize >= normalised.len(),
            "Length of normalised seed should be less than u8::MAX for performance, got {}",
            normalised.len()
        );
    }
}

/// `Fortuna` is a struct that contains a pool of pseudo-random bytes.
/// The entropy pool will regenerate itself if it is empty.
///
/// `Fortuna` is designed to be created once and reused.
///
/// `Fortuna` does not panic and guarantees to always return `Ok`.
pub struct Fortuna {
    entropy_pool: EntropyPool,
}

impl Fortuna {
    /// Creates a new `Fortuna` instance.
    ///
    /// `Fortuna` contains a pool of entropy. Generating the pool can be resource intensive, so make
    /// sure to use as few instances as possible.
    /// The pool will regenerate itself if it is empty.
    ///
    /// Please supply a seed vector with at least one element.
    /// In general: The more the better.
    /// The seed is the only real source of entropy.
    ///
    /// Recommended seed lengths are provided in the [`seed_lengths`] module.
    ///
    /// To increase the perceived randomness, ensure that as many byte values from 0x00 to 0xFF are used in your seed as possible.
    #[must_use]
    pub fn new<V: Into<Vec<u8>>>(seed: V) -> Self {
        Self {
            entropy_pool: EntropyPool::new(seed),
        }
    }

    /// Call with the start and end of the range (both `u32`).
    /// The range is inclusive on both ends.
    pub fn random_from_u32_range(&mut self, start: u32, end: u32) -> RngResult<u32> {
        if start < end {
            let range_size = (end).saturating_sub(start).saturating_add(1);
            let rng = self.random_u32().unwrap_or(0);
            let random_index = rng % range_size;
            Ok(start.saturating_add(random_index))
        } else if start == end {
            Ok(start)
        } else {
            self.random_from_u32_range(end, start)
        }
    }

    /// Converts the `EntropyPool` to a `XffValue`.
    ///
    /// Useful for serialization if the state of the `EntropyPool` is needed or wanted for re-initialization.
    pub fn to_xff(&self) -> XffValue {
        self.entropy_pool.to_xff()
    }

    /// Converts the `XffValue` to a `EntropyPool`.
    ///
    /// Use [`EntropyPool::to_xff`] to convert a `EntropyPool` to an `XffValue`.
    pub fn from_xff(xff: &XffValue) -> RngResult<Self> {
        EntropyPool::from_xff(xff).map(|entropy_pool| Self { entropy_pool })
    }
}

impl RngApi for Fortuna {
    fn random_u8(&mut self) -> RngResult<u8> {
        Ok(self.entropy_pool.get_random_byte())
    }

    fn random_u16(&mut self) -> RngResult<u16> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(u16::from_le_bytes(rng))
    }

    fn random_u32(&mut self) -> RngResult<u32> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(u32::from_le_bytes(rng))
    }

    fn random_u64(&mut self) -> RngResult<u64> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(u64::from_le_bytes(rng))
    }

    fn random_usize(&mut self) -> RngResult<usize> {
        let mut rng = [0u8; std::mem::size_of::<usize>()];
        for byte in &mut rng {
            *byte = self.entropy_pool.get_random_byte();
        }
        Ok(usize::from_le_bytes(rng))
    }

    fn random_i8(&mut self) -> RngResult<i8> {
        let rng = [self.entropy_pool.get_random_byte()];
        Ok(i8::from_le_bytes(rng))
    }

    fn random_i16(&mut self) -> RngResult<i16> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(i16::from_le_bytes(rng))
    }

    fn random_i32(&mut self) -> RngResult<i32> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(i32::from_le_bytes(rng))
    }

    fn random_i64(&mut self) -> RngResult<i64> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        Ok(i64::from_le_bytes(rng))
    }

    fn random_f32(&mut self) -> RngResult<f32> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        let out = f32::from_le_bytes(rng);
        if out.is_nan() {
            self.random_f32()
        } else {
            Ok(out)
        }
    }

    fn random_f64(&mut self) -> RngResult<f64> {
        let rng = [
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
            self.entropy_pool.get_random_byte(),
        ];
        let out = f64::from_le_bytes(rng);
        if out.is_nan() {
            self.random_f64()
        } else {
            Ok(out)
        }
    }

    fn random_bytes(&mut self, len: usize) -> RngResult<Vec<u8>> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.random_u8()?);
        }
        Ok(bytes)
    }

    fn random_string(&mut self, len: usize) -> RngResult<String> {
        let mut s = String::with_capacity(len);
        for _ in 0..len {
            s.push(self.random_ascii_char()?);
        }
        Ok(s)
    }

    fn random_latin_char(&mut self, uppercase: bool) -> RngResult<char> {
        let chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let chosen_char = chars[self.random_index(chars.len())?];
        if uppercase {
            Ok(chosen_char.to_ascii_uppercase())
        } else {
            Ok(chosen_char)
        }
    }

    fn random_ascii_char(&mut self) -> RngResult<char> {
        let val = self.random_from_range_inclusive(32, 126)?;
        Ok(val as u8 as char)
    }

    fn random_bool(&mut self) -> RngResult<bool> {
        let rng = self.entropy_pool.get_random_byte();
        Ok(!rng.is_multiple_of(2))
    }

    fn random_from_range_inclusive(&mut self, min: usize, max: usize) -> RngResult<usize> {
        if min < max {
            let range_size = (max).saturating_sub(min).saturating_add(1);
            let rng = self.random_usize()?;
            let random_index = rng % range_size;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_range_inclusive(max, min)
        }
    }

    fn random_from_range(&mut self, min: usize, max: usize) -> RngResult<usize> {
        if min < max {
            let range_size = (max).saturating_sub(min);
            let rng = self.random_usize()?;
            let random_index = rng % range_size;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_range(max, min)
        }
    }

    fn random_from_u64_range(&mut self, min: u64, max: u64) -> RngResult<u64> {
        if min < max {
            let range_size = max - min;
            let rng = self.random_u64()?;
            let random_index = rng % range_size;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_u64_range(max, min)
        }
    }

    fn random_from_i_range(&mut self, min: isize, max: isize) -> RngResult<isize> {
        if min < max {
            let range_size = (max as i128 - min as i128) as usize;
            let rng = self.random_usize()?;
            let random_index = (rng % range_size) as isize;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_i_range(max, min)
        }
    }

    fn random_from_i64_range(&mut self, min: i64, max: i64) -> RngResult<i64> {
        if min < max {
            let range_size = (max as i128 - min as i128) as u64;
            let rng = self.random_u64()?;
            let random_index = (rng % range_size) as i64;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_i64_range(max, min)
        }
    }

    fn random_from_i32_range(&mut self, min: i32, max: i32) -> RngResult<i32> {
        if min < max {
            let range_size = (max as i64 - min as i64) as u32;
            let rng = self.random_u32()?;
            let random_index = (rng % range_size) as i32;
            Ok(min.saturating_add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_i32_range(max, min)
        }
    }

    fn random_from_f32_range(&mut self, min: f32, max: f32) -> RngResult<f32> {
        if min < max {
            let range_size = max.sub(min);
            let rng = self.random_f32()?;
            let random_index = rng.abs() % range_size;
            Ok(min.add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_f32_range(max, min)
        }
    }

    fn random_from_f64_range(&mut self, min: f64, max: f64) -> RngResult<f64> {
        if min < max {
            let range_size = max.sub(min);
            let rng = self.random_f64()?;
            let random_index = rng.abs() % range_size;
            Ok(min.add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            self.random_from_f64_range(max, min)
        }
    }

    fn random_index(&mut self, collection_length: usize) -> RngResult<usize> {
        if collection_length >= 1 {
            self.random_with_ceiling(collection_length.saturating_sub(1))
        } else {
            Ok(collection_length)
        }
    }

    fn random_with_ceiling(&mut self, max: usize) -> RngResult<usize> {
        self.random_from_range_inclusive(usize::MIN, max)
    }

    fn random_with_floor(&mut self, min: usize) -> RngResult<usize> {
        self.random_from_range_inclusive(min, usize::MAX)
    }
}

fn is_valid_ascii(input: u8) -> bool {
    (32..=127).contains(&input)
}
