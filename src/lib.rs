
mod entropy_pool;

use crate::entropy_pool::EntropyPool;
use std::ops::{Add, Sub};

pub struct Fortuna {
    entropy_pool: EntropyPool,
}

impl Fortuna {
    pub fn new() -> Self {
        Self {
            entropy_pool: EntropyPool::new(),
        }
    }

    pub fn create_size_restricted(initial_pool_size: usize) -> Self {
        Self {
            entropy_pool: EntropyPool::create_size_restricted(initial_pool_size),
        }
    }

    /// Generates a pseudorandom `u8`. 
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns:
    /// Returns `Ok(u8)` otherwise.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::prelude::random_u8;
    ///
    /// fn main() {
    ///   let random_number: u8 = random_u8();
    ///   println!("Generated random u8: {}", random_number);
    /// }
    /// ```
    pub fn random_u8(&mut self) -> u8 {
        self.entropy_pool.get_random_byte()
    }

    /// Generates a pseudorandom `u16`
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(u16)` with the random `u16` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_u16;
        ///
        /// fn main() {
        ///   let random_number: u16 = random_u16();
        ///   println!("Generated random u16: {}", random_number);
        /// }
        /// ```
        pub fn random_u16(&mut self) -> u16 {
            let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
            u16::from_le_bytes(rng)
        }

        
        /// Generates a pseudorandom `u32`
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(u32)` with the random `u32` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_u32;
        ///
        /// fn main() {
        ///   let random_number: u32 = random_u32().unwrap();
        ///   println!("Generated random u32: {}", random_number);
        /// }
        /// ```
        pub fn random_u32(&mut self) -> u32 {
            let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
            u32::from_le_bytes(rng)
        }

        /// Generates a pseudorandom `u64`
        ///
        /// Please note that this function needs a 64bit system for obvious reasons.
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(u64)` with the random `u64` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_u64;
        ///
        /// fn main() {
        ///   let random_number: u64 = random_u64().unwrap();
        ///   println!("Generated random u64: {}", random_number);
        /// }
        /// ```
        pub fn random_u64(&mut self) -> u64 {
            let rng = [
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),];
            u64::from_le_bytes(rng)
        }

        /// Generates a pseudorandom `i8`
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(i8)` with the random `i8` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_i8;
        ///
        /// fn main() {
        ///   let random_number: i8 = random_i8().unwrap();
        ///   println!("Generated random i8: {}", random_number);
        /// }
        /// ```
        pub fn random_i8(&mut self) -> i8 {
            let rng = [self.entropy_pool.get_random_byte()];
            i8::from_le_bytes(rng)
        }

        pub fn random_i16(&mut self) -> i16 {
            let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
            i16::from_le_bytes(rng)
        }

        /// Generates a pseudorandom `i32` 
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(i32)` with the random `i32` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_i32;
        ///
        /// fn main() {
        ///   let random_number: i32 = random_i32().unwrap();
        ///   println!("Generated random i32: {}", random_number);
        /// }
        /// ```
        pub fn random_i32(&mut self) -> i32 {
            let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
            i32::from_le_bytes(rng)
        }

        pub fn random_i64(&mut self) -> i64 {
            let rng = [
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),
                self.entropy_pool.get_random_byte(),];
            i64::from_le_bytes(rng)
        }

        /// Generates a pseudorandom `f32`
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(f32)` with the random `f32` number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_f32;
        ///
        /// fn main() {
        ///   let random_number: f32 = random_f32().unwrap();
        ///   println!("Generated random f32: {}", random_number);
        /// }
        /// ```
        pub fn random_f32(&mut self) -> f32 {
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
                out
            }
        }

        pub fn random_f64(&mut self) -> f64 {
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
                out
            }
        }

        /// Generates a random latin character, can be upper or lower case.
        ///
        /// ## Arguments
        ///
        /// * `uppercase` - `true` for upper case, `false` for lower case
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_latin_char;
        ///
        /// fn main() {
        ///   let random_char: char = random_latin_char(true).unwrap();
        ///   println!("Generated random char: {}", random_char);
        /// }
        /// ```
        pub fn random_latin_char(&mut self, uppercase: bool) -> char {
            let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
            let chosen_char = chars[self.random_index(chars.len())];
            if uppercase {
                chosen_char.to_ascii_uppercase()
            } else {
                chosen_char
            }
        }

        /// Generates a random boolean.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_bool;
        ///
        /// fn main() {
        ///   let random_bool: bool = random_bool().unwrap();
        ///   println!("Generated random bool: {}", random_bool);
        /// }
        /// ```
        pub fn random_bool(&mut self) -> bool {
            let rng = self.entropy_pool.get_random_byte();
            if rng % 2 == 0 {
                return false;
            } else {
                return true;
            }
        }
        
        /// Call with the start and end of the range (both `usize`).
        /// The range is inclusive on both ends.
        /// 
        /// Uses a 32bit seeded rng, for 64bit seeded rng please use `random_from_u64range`.
        ///
        /// ## Panics
        /// If the start is greater than the end.
        ///
        /// ## Returns
        /// Will return `Ok(usize)` wrapping a number inside the given range.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_from_range;
        ///
        /// fn main() {
        ///     let chosen_element = random_from_range(0, 100).unwrap();
        ///     println!("Chosen element {chosen_element}, in range 0-100");
        /// }
        /// ```
        pub fn random_from_range(&mut self, start: usize, end: usize) -> usize {
            if start < end {
                let range_size = (end).saturating_sub(start).saturating_add(1);
                let rng = self.random_u32();
                let random_index = rng as usize % range_size;
                start.saturating_add(random_index)
            } else if start == end {
                start
            } else {
                self.random_from_range(end, start)
            }
        }

        /// Call with the start and end of the range (both `u64`).
        /// The range is inclusive on both ends.
        /// 
        /// Please note that this function needs a 64bit system for obvious reasons.
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// Will return `Ok(u64)` wrapping a number inside the given range.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_from_u64range;
        ///
        /// fn main() {
        ///     let chosen_element = random_from_u64range(0, 100).unwrap();
        ///     println!("Chosen element {chosen_element}, in range 0-100");
        /// }
        /// ```
        pub fn random_from_u64range(&mut self, start: u64, end: u64) -> u64 {
            if start < end {
                let range_size = (end - start).saturating_add(1);
                let rng = self.random_u64();
                let random_index = rng % range_size;
                start.saturating_add(random_index)
            } else if start == end {
                start
            } else {
                self.random_from_u64range(end, start)
            }
        }

        /// Call with the start and end of the range (both `f32`).
        /// The range is inclusive on start, and never quite reaches end.
        /// 
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// Will return `Ok(f32)` wrapping a number inside the given range.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_from_f32range;
        ///
        /// fn main() {
        ///     let chosen_element = random_from_f32range(0.1, 100.1).unwrap();
        ///     println!("Chosen element {chosen_element}, in range 0.1-100.1");
        /// }
        /// ```
        pub fn random_from_f32range(&mut self, start: f32, end: f32) -> f32 {
            if start < end {
                // I still believe this to have an off by one error, however it is infinity small
                // because of f32.
                // As further reading did not help in the slightes but confirm that floating point
                // numbers are weird I will have to live with it. It seems to grow towards end, and
                // never reaching it. I now suspect maths shinanigans.
                let range_size = end.sub(start);//.add(1.0);
                let rng = self.random_f32();
                    if rng.is_sign_positive() {
                        let random_index = rng % range_size;
                        start.add(random_index)
                    } else {
                        let random_index = (rng * -1.0) % range_size;
                        start.add(random_index)
                    }
            } else if start == end {
                start
            } else {
                self.random_from_f32range(end, start)
            }
        }

        /// Call with the start and end of the range (both `i32`).
        /// The range is inclusive on both ends.
        /// 
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// Will return `Ok(i32)` wrapping a number inside the given range.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_from_i32range;
        ///
        /// fn main() {
        ///     let chosen_element = random_from_i32range(-100, 100).unwrap();
        ///     println!("Chosen element {chosen_element}, in range -100, 100");
        /// }
        /// ```
        pub fn random_from_i32range(&mut self, start: i32, end: i32) -> i32 {
            if start < end {
                let range_size = end.sub(start).add(1);
                let rng = self.random_i32();
                if rng.is_positive() {
                    let random_index = rng % range_size;
                    start.add(random_index)
                } else {
                    let random_index = -rng % range_size;
                    start.add(random_index)
                }
            } else if start == end {
                start
            } else {
                self.random_from_i32range(end, start)
            }
        }

        /// Takes in the length of a collection, like a vector, and returns a valid, random, index for
        /// it.
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(usize)` containing the index.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_index;
        ///
        /// fn main() {
        ///    let collection = (0..100).collect::<Vec<usize>>();
        ///    let random_index = random_index(collection.len()).unwrap();
        ///    println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
        /// }
        /// ```
        pub fn random_index(&mut self, collection_length: usize) -> usize {
            if collection_length >= 1 {
                self.random_with_ceiling(collection_length.saturating_sub(1))
            } else {
                // collection has 1 element
                collection_length
            }
        }

        /// Computes a random number between 0 and the `ceiling` argument.
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(usize)` containing the number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_with_ceiling;
        ///
        /// fn main() {
        ///    for n in 100000..200000  {
        ///         let answ = random_with_ceiling(n);
        ///         println!("The random number between 0 and {} is: {}", n, answ.unwrap());
        ///    }
        /// }
        /// ```
        pub fn random_with_ceiling(&mut self, ceiling: usize) -> usize {
            let min_usize = usize::MIN;
            self.random_from_range(min_usize, ceiling)
        }

        /// Computes a random number between `usize::MAX` and the `floor` argument.
        ///
        /// ## Errors
        /// All `Error`'s are `std::io::Error` types.
        /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
        /// Or if the program cannot access `/dev/urandom`. This is most likely.
        ///
        /// ## Returns
        /// `Ok(usize)` containing the number.
        ///
        /// ## Example:
        /// ```
        /// use fortuna::prelude::random_with_floor;
        ///
        /// fn main() {
        ///    for n in 0..100000  {
        ///         let answ = random_with_floor(n);
        ///         let max_usize = usize::MAX;
        ///         println!("The random number between {} and {} is: {}", max_usize, n, answ.unwrap());
        ///    }
        /// }
        /// ```
        pub fn random_with_floor(&mut self, floor: usize) -> usize {
            let max_usize = usize::MAX;
            self.random_from_range(floor, max_usize)
        }
    }

