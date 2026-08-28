#[doc = include_str!("../README.md")]
mod entropy_pool;

use crate::entropy_pool::EntropyPool;
pub use athena::rng_api::{RngApi, RngResult};
use std::ops::{Add, Sub};

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
    /// The more the better.
    /// The seed is the only real source of entropy.
    #[must_use]
    pub fn new(seed: Vec<u8>) -> Self {
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
        let ran_u8 = self.entropy_pool.get_random_byte();
        if is_valid_ascii(ran_u8) {
            Ok(char::from_u32(u32::from(ran_u8)).expect("Valid ASCII character"))
        } else {
            let try1 = ran_u8.saturating_add(self.entropy_pool.get_random_byte());
            if is_valid_ascii(try1) {
                return Ok(char::from_u32(u32::from(try1)).expect("Valid ASCII character"));
            }
            let try2 = ran_u8.saturating_sub(self.entropy_pool.get_random_byte());
            if is_valid_ascii(try2) {
                return Ok(char::from_u32(u32::from(try2)).expect("Valid ASCII character"));
            }
            let try3 = ran_u8.saturating_add(self.entropy_pool.get_random_byte());
            if is_valid_ascii(try3) {
                return Ok(char::from_u32(u32::from(try3)).expect("Valid ASCII character"));
            }
            let try4 = ran_u8.saturating_sub(self.entropy_pool.get_random_byte());
            if is_valid_ascii(try4) {
                return Ok(char::from_u32(u32::from(try4)).expect("Valid ASCII character"));
            }
            self.random_ascii_char()
        }
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
