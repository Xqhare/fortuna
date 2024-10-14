/*!
# Fortuna
A dependency-free, deterministic, panic-proof PRNG for any OS on any hardware.

If you are looking for a true CSPRNG check out my project [Tyche](https://github.com/xqhare/tyche).

It is recommended to use Fortuna on x86_64, riscv64 or aarch64 CPU architectures. On all other architectures, Fortuna will not be able to read the CPU features and fall back to a pre-generated entropy source instead.

## Features

- Performant
- Deterministic
- Panic-proof / no errors
- No dependencies
- Customizable memory footprint

## Naming
Fortuna is named after the ancient roman goddess of fortune. Her Greek equivalent is Tyche and lends her name to my own CSPRNG project.

## Usage
First add `fortuna` to your `Cargo.toml`:

```toml
[dependencies]
fortuna = { git = "https://github.com/Xqhare/fortuna" }
```

Then run `cargo update` to pull the latest version.

### Example
As `Fortuna` is quite small in scope, every function is listed in the example below.

```rust
use fortuna::Fortuna;

fn main() {
    let mut fortuna = Fortuna::new();

    let random_u8: u8 = fortuna.random_u8();
    let random_u16: u16 = fortuna.random_u16();
    let random_u32: u32 = fortuna.random_u32();
    let random_u64: u64 = fortuna.random_u64();

    let random_i8: i8 = fortuna.random_i8();
    let random_i16: i16 = fortuna.random_i16();
    let random_i32: i32 = fortuna.random_i32();
    let random_i64: i64 = fortuna.random_i64();

    let random_f32: f32 = fortuna.random_f32();
    let random_f64: f64 = fortuna.random_f64();

    let lowercase_char: char = fortuna.random_latin_char(false);
    let uppercase_char: char = fortuna.random_latin_char(true);

    let random_bool: bool = fortuna.random_bool();

    let random_u_range: usize = fortuna.random_from_range(0, 100);
    let random_u32_range: u32 = fortuna.random_from_u32_range(0, 100);
    let random_u64_range: u64 = fortuna.random_from_u64_range(0, 100);

    let random_i_range: isize = fortuna.random_from_i_range(-100, 100);
    let random_i32_range: i32 = fortuna.random_from_i32_range(-100, 100);
    let random_i64_range: i64 = fortuna.random_from_i64_range(-100, 100);

    let random_f32_range = fortuna.random_from_f32_range(-100.0, 100.0);
    let random_f64_range = fortuna.random_from_f64_range(-100.0, 100.0);

    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let random_index: usize = fortuna.random_index(vector.len());

    let random_with_ceiling: usize = fortuna.random_with_ceiling(100);
    let random_with_floor: usize = fortuna.random_with_floor(100);

    println!("Generated random u8: {}", random_u8);
    println!("Generated random u16: {}", random_u16);
    println!("Generated random u32: {}", random_u32);
    println!("Generated random u64: {}", random_u64);

    println!("Generated random i8: {}", random_i8);
    println!("Generated random i16: {}", random_i16);
    println!("Generated random i32: {}", random_i32);
    println!("Generated random i64: {}", random_i64);

    println!("Generated random f32: {}", random_f32);
    println!("Generated random f64: {}", random_f64);

    println!("Generated random lowercase char: {}", lowercase_char);
    println!("Generated random uppercase char: {}", uppercase_char);

    println!("Generated random bool: {}", random_bool);

    println!("Generated random u range: {}", random_u_range);
    println!("Generated random u32 range: {}", random_u32_range);
    println!("Generated random u64 range: {}", random_u64_range);

    println!("Generated random i range: {}", random_i_range);
    println!("Generated random i32 range: {}", random_i32_range);
    println!("Generated random i64 range: {}", random_i64_range);

    println!("Generated random f32 range: {}", random_f32_range);
    println!("Generated random f64 range: {}", random_f64_range);

    println!("Generated random index: {}", random_index);

    println!("Generated random with ceiling: {}", random_with_ceiling);
    println!("Generated random with floor: {}", random_with_floor);
}
```

## How Fortuna generates random numbers
Fortuna relies on the system it is executed on to provide entropy.

Fortuna is deterministic, so it will always return the same number for the same inputs. Because of this, some inputs have been chosen to never return the same number (e.g. System Time).
This means that if the complete state of the system is known it is possible to predict the next number.

Other inputs will always return the same number, like CPU features or the amount of files in the root directory.

Most, if not all variation is provided by the measured time spend building the pool, or parts of it.

Some inputs are combined into one entropy pool. The others are also pooled together to form a second pool. The second pool is then used to scramble the entropy pool itself.

The entropy pool is finite in size, but should be around 100k bytes of entropy.

The entropy pool will be refilled as needed.

The largest sources of entropy are system time and code execution times.

### Entropy sources
Fortuna uses the following entropy sources:
- System time
- File system properties (if available)
    - Main disk device ID and inode number
    - Directory nest depth to root
    - Current working directory properties
- Code execution times
- CPU features

*/
mod entropy_pool;

use crate::entropy_pool::EntropyPool;
use std::ops::{Add, Sub};


/// `Fortuna` is a struct that contains a pool of pseudo-random bytes.
/// The entropy pool will regenerate itself if it is empty.
///
/// `Fortuna` is designed to be created once and reused.
///
/// `Fortuna` does not panic.
///
/// Generating or regenerating the entropy pool is an expensive operation, so it is recommended to
/// keep the pool size on the larger side if you are constructing `Fortuna` by using `create_size_restricted`.
///
/// ## Example
/// ```
/// use fortuna::Fortuna;
///
/// fn main() {
///   let mut fortuna = Fortuna::new();
///
///   let random_u8: u8 = fortuna.random_u8();
///   let random_u16: u16 = fortuna.random_u16();
///   let random_u32: u32 = fortuna.random_u32();
///   let random_u64: u64 = fortuna.random_u64();
///
///   let random_i8: i8 = fortuna.random_i8();
///   let random_i16: i16 = fortuna.random_i16();
///   let random_i32: i32 = fortuna.random_i32();
///   let random_i64: i64 = fortuna.random_i64();
///
///   let random_f32: f32 = fortuna.random_f32();
///   let random_f64: f64 = fortuna.random_f64();
///
///   let lowercase_char: char = fortuna.random_latin_char(false);
///   let uppercase_char: char = fortuna.random_latin_char(true);
///
///   let random_bool: bool = fortuna.random_bool();
///
///   let random_u_range: usize = fortuna.random_from_range(0, 100);
///   let random_u32_range: u32 = fortuna.random_from_u32_range(0, 100);
///   let random_u64_range: u64 = fortuna.random_from_u64_range(0, 100);
///   
///   let random_i_range: isize = fortuna.random_from_i_range(-100, 100);
///   let random_i32_range: i32 = fortuna.random_from_i32_range(-100, 100);
///   let random_i64_range: i64 = fortuna.random_from_i64_range(-100, 100);
///
///   let random_f32_range = fortuna.random_from_f32_range(-100.0, 100.0);
///   let random_f64_range = fortuna.random_from_f64_range(-100.0, 100.0);
///
///   let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///   let random_index: usize = fortuna.random_index(vector.len());
///
///   let random_with_ceiling: usize = fortuna.random_with_ceiling(100);
///   let random_with_floor: usize = fortuna.random_with_floor(100);
///
///   println!("Generated random u8: {}", random_u8);
///   println!("Generated random u16: {}", random_u16);
///   println!("Generated random u32: {}", random_u32);
///   println!("Generated random u64: {}", random_u64);
///
///   println!("Generated random i8: {}", random_i8);
///   println!("Generated random i16: {}", random_i16);
///   println!("Generated random i32: {}", random_i32);
///   println!("Generated random i64: {}", random_i64);
///
///   println!("Generated random f32: {}", random_f32);
///   println!("Generated random f64: {}", random_f64);
///
///   println!("Generated random lowercase char: {}", lowercase_char);
///   println!("Generated random uppercase char: {}", uppercase_char);
///
///   println!("Generated random bool: {}", random_bool);
///
///   println!("Generated random u range: {}", random_u_range);
///   println!("Generated random u32 range: {}", random_u32_range);
///   println!("Generated random u64 range: {}", random_u64_range);
///
///   println!("Generated random i range: {}", random_i_range);
///   println!("Generated random i32 range: {}", random_i32_range);
///   println!("Generated random i64 range: {}", random_i64_range);
///
///   println!("Generated random f32 range: {}", random_f32_range);
///   println!("Generated random f64 range: {}", random_f64_range);
///
///   println!("Generated random index: {}", random_index);
///
///   println!("Generated random with ceiling: {}", random_with_ceiling);
///   println!("Generated random with floor: {}", random_with_floor);
/// }
/// ```
pub struct Fortuna {
    entropy_pool: EntropyPool,
}

impl Fortuna {

    /// Creates a new `Fortuna` instance.
    ///
    /// `Fortuna` contains a pool of entropy. Generating the pool is resource intensive, so make
    /// sure to use as few instances as possible.
    /// The pool will regenerate itself if it is empty.
    ///
    /// Use `create_size_restricted` to create a new instance with a size restricted pool.
    ///
    /// ## Example
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: u8 = fortuna.random_u8();
    ///   let random_bool: bool = fortuna.random_bool();
    ///   println!("Generated random u8: {}", random_number);
    ///   println!("Generated random bool: {}", random_bool);
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            entropy_pool: EntropyPool::new(),
        }
    }

    /// Creates a new `Fortuna` instance with a restricted pool size.
    /// This is useful if you are only generating a small number of random values, or operating
    /// with little memory.
    /// Generating entropy is resource intensive, so the pool size should be chosen to minimize
    /// regenerations.
    ///
    /// `Fortuna` contains a pool of entropy. Generating the pool is resource intensive, so make
    /// sure to use as few instances as possible.
    /// The pool will regenerate itself if it is empty.
    ///
    /// ## Parameters:
    /// - `initial_pool_size`: The initial size of the pool.
    ///
    /// ## Example
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::create_size_restricted(1_000);
    ///   let random_number: u8 = fortuna.random_u8();
    ///   let random_bool: bool = fortuna.random_bool();
    ///   println!("Generated random u8: {}", random_number);
    ///   println!("Generated random bool: {}", random_bool);
    /// }
    /// ```    
    pub fn create_size_restricted(initial_pool_size: usize) -> Self {
        Self {
            entropy_pool: EntropyPool::create_size_restricted(initial_pool_size),
        }
    }

    /// Generates a pseudo-random `u8`. 
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: u8 = fortuna.random_u8();
    ///   println!("Generated random u8: {}", random_number);
    /// }
    /// ```
    pub fn random_u8(&mut self) -> u8 {
        self.entropy_pool.get_random_byte()
    }

    /// Generates a pseudo-random `u16`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: u16 = fortuna.random_u16();
    ///   println!("Generated random u16: {}", random_number);
    /// }
    /// ```
    pub fn random_u16(&mut self) -> u16 {
        let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
        u16::from_le_bytes(rng)
    }

    
    /// Generates a pseudo-random `u32`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: u32 = fortuna.random_u32();
    ///   println!("Generated random u32: {}", random_number);
    /// }
    /// ```
    pub fn random_u32(&mut self) -> u32 {
        let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
        u32::from_le_bytes(rng)
    }

    /// Generates a pseudo-random `u64`
    ///
    /// This function needs a 64bit system for obvious reasons.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: u64 = fortuna.random_u64();
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

    /// Generates a pseudo-random `i8`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: i8 = fortuna.random_i8();
    ///   println!("Generated random i8: {}", random_number);
    /// }
    /// ```
    pub fn random_i8(&mut self) -> i8 {
        let rng = [self.entropy_pool.get_random_byte()];
        i8::from_le_bytes(rng)
    }

    /// Generates a pseudo-random `i16`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: i16 = fortuna.random_i16();
    ///   println!("Generated random i16: {}", random_number);
    /// }
    /// ```
    pub fn random_i16(&mut self) -> i16 {
        let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
        i16::from_le_bytes(rng)
    }

    /// Generates a pseudo-random `i32` 
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: i32 = fortuna.random_i32();
    ///   println!("Generated random i32: {}", random_number);
    /// }
    /// ```
    pub fn random_i32(&mut self) -> i32 {
        let rng = [self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte(), self.entropy_pool.get_random_byte()];
        i32::from_le_bytes(rng)
    }

    /// Generates a pseudo-random `i64`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: i64 = fortuna.random_i64();
    ///   println!("Generated random i64: {}", random_number);
    /// }
    /// ```
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

    /// Generates a pseudo-random `f32`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: f32 = fortuna.random_f32();
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

    /// Generates a pseudo-random `f64`
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_number: f64 = fortuna.random_f64();
    ///   println!("Generated random f64: {}", random_number);
    /// }
    /// ```
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

    /// Generates a pseudo-random latin character.
    ///
    /// ## Arguments
    ///
    /// * `uppercase` - `true` for upper case, `false` for lower case
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_char_uppercase: char = fortuna.random_latin_char(true);
    ///   println!("Generated random uppercase char: {}", random_char_uppercase);
    ///   let random_char_lowercase: char = fortuna.random_latin_char(false);
    ///   println!("Generated random lowercase char: {}", random_char_lowercase);
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

    /// Generates a pseudo-random boolean.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///   let mut fortuna = Fortuna::new();
    ///   let random_bool: bool = fortuna.random_bool();
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
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_range(0, 100);
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
    
    /// Call with the start and end of the range (both `u32`).
    /// The range is inclusive on both ends.
    /// 
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_u32_range(0, 100);
    ///     println!("Chosen element {chosen_element}, in range 0-100");
    /// }
    /// ```
    pub fn random_from_u32_range(&mut self, start: u32, end: u32) -> u32 {
        if start < end {
            let range_size = (end).saturating_sub(start).saturating_add(1);
            let rng = self.random_u32();
            let random_index = rng % range_size;
            start.saturating_add(random_index)
        } else if start == end {
            start
        } else {
            self.random_from_u32_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `u64`).
    /// The range is inclusive on both ends.
    /// 
    /// This function needs a 64bit system for obvious reasons.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_u64_range(0, 100);
    ///     println!("Chosen element {chosen_element}, in range 0-100");
    /// }
    /// ```
    pub fn random_from_u64_range(&mut self, start: u64, end: u64) -> u64 {
        if start < end {
            let range_size = (end - start).saturating_add(1);
            let rng = self.random_u64();
            let random_index = rng % range_size;
            start.saturating_add(random_index)
        } else if start == end {
            start
        } else {
            self.random_from_u64_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `f32`).
    /// The range is inclusive on start, and never quite reaches end.
    /// 
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_f32_range(0.1, 100.1);
    ///     println!("Chosen element {chosen_element}, in range 0.1-100.1");
    /// }
    /// ```
    pub fn random_from_f32_range(&mut self, start: f32, end: f32) -> f32 {
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
            self.random_from_f32_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `f64`).
    /// The range is inclusive on start, and never quite reaches end.
    ///
    /// This function needs a 64bit system for obvious reasons.
    /// 
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_f64_range(-100.1, 100.1);
    ///     println!("Chosen element {chosen_element}, in range -100.1 - 100.1");
    /// }
    /// ```
    pub fn random_from_f64_range(&mut self, start: f64, end: f64) -> f64 {
        if start < end {
            let range_size = end.sub(start);//.add(1.0);
            let rng = self.random_f64();
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
            self.random_from_f64_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `i32`).
    /// The range is inclusive on both ends.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_i32_range(-100, 100);
    ///     println!("Chosen element {chosen_element}, in range -100, 100");
    /// }
    /// ```
    pub fn random_from_i32_range(&mut self, start: i32, end: i32) -> i32 {
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
            self.random_from_i32_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `i64`).
    /// The range is inclusive on both ends.
    ///
    /// This function needs a 64bit system for obvious reasons.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_i64_range(-100, 100);
    ///     println!("Chosen element {chosen_element}, in range -100, 100");
    /// }
    /// ```
    pub fn random_from_i64_range(&mut self, start: i64, end: i64) -> i64 {
        if start < end {
            let range_size = end.sub(start).add(1);
            let rng = self.random_i64();
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
            self.random_from_i64_range(end, start)
        }
    }

    /// Call with the start and end of the range (both `isize`).
    /// The range is inclusive on both ends.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let chosen_element = fortuna.random_from_i_range(-100, 100);
    ///     println!("Chosen element {chosen_element}, in range -100, 100");
    /// }
    /// ```
    pub fn random_from_i_range(&mut self, start: isize, end: isize) -> isize {
        if start < end {
            let range_size = end.sub(start).add(1);
            let rng = self.random_i32() as isize;
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
            self.random_from_i_range(end, start)
        }
    }

    /// Takes in the length of a collection, like a vector, and returns a valid, random, index for
    /// it.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     let collection = (0..100).collect::<Vec<usize>>();
    ///     let random_index = fortuna.random_index(collection.len());
    ///     println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
    /// }
    /// ```
    pub fn random_index(&mut self, collection_length: usize) -> usize {
        if collection_length >= 1 {
            self.random_with_ceiling(collection_length.saturating_sub(1))
        } else {
            // collection has 0 elements ... return 0 to keep no errors
            collection_length
        }
    }

    /// Computes a random number between 0 and the `ceiling` argument.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     for n in 100000..200000  {
    ///          let answ = fortuna.random_with_ceiling(n);
    ///          println!("The random number between 0 and {} is: {}", n, answ);
    ///     }
    /// }
    /// ```
    pub fn random_with_ceiling(&mut self, ceiling: usize) -> usize {
        self.random_from_range(usize::MIN, ceiling)
    }

    /// Computes a random number between `usize::MAX` and the `floor` argument.
    ///
    /// ## Example:
    /// ```
    /// use fortuna::Fortuna;
    ///
    /// fn main() {
    ///     let mut fortuna = Fortuna::new();
    ///     for n in 0..100000  {
    ///          let answ = fortuna.random_with_floor(n);
    ///          let max_usize = usize::MAX;
    ///          println!("The random number between {} and {} is: {}", max_usize, n, answ);
    ///     }
    /// }
    /// ```
    pub fn random_with_floor(&mut self, floor: usize) -> usize {
        self.random_from_range(floor, usize::MAX)
    }
}

