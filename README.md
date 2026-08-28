# Fortuna
A dependency-free, deterministic, panic-proof PRNG for any OS on any hardware.

Hobby project, probably not suitable for production use.

If you are looking for a true CSPRNG check out my project [Tyche](https://github.com/xqhare/tyche).

## Features

- Performant
- Deterministic
- Panic-proof / no errors
- No dependencies

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
use fortuna::{Fortuna, RngApi};
fn main() {
    let mut fortuna = Fortuna::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let random_u8: u8 = fortuna.random_u8().unwrap();
    let random_u16: u16 = fortuna.random_u16().unwrap();
    let random_u32: u32 = fortuna.random_u32().unwrap();
    let random_u64: u64 = fortuna.random_u64().unwrap();

    let random_i8: i8 = fortuna.random_i8().unwrap();
    let random_i16: i16 = fortuna.random_i16().unwrap();
    let random_i32: i32 = fortuna.random_i32().unwrap();
    let random_i64: i64 = fortuna.random_i64().unwrap();

    let random_f32: f32 = fortuna.random_f32().unwrap();
    let random_f64: f64 = fortuna.random_f64().unwrap();

    let lowercase_char: char = fortuna.random_latin_char(false).unwrap();
    let uppercase_char: char = fortuna.random_latin_char(true).unwrap();

    let ascii_char: char = fortuna.random_ascii_char().unwrap();

    let random_bool: bool = fortuna.random_bool().unwrap();

    let random_u_range: usize = fortuna.random_from_range(0, 100).unwrap();
    let random_u32_range: u32 = fortuna.random_from_u32_range(0, 100).unwrap();
    let random_u64_range: u64 = fortuna.random_from_u64_range(0, 100).unwrap();

    let random_i_range: isize = fortuna.random_from_i_range(-100, 100).unwrap();
    let random_i32_range: i32 = fortuna.random_from_i32_range(-100, 100).unwrap();
    let random_i64_range: i64 = fortuna.random_from_i64_range(-100, 100).unwrap();

    let random_f32_range = fortuna.random_from_f32_range(-100.0, 100.0).unwrap();
    let random_f64_range = fortuna.random_from_f64_range(-100.0, 100.0).unwrap();

    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let random_index: usize = fortuna.random_index(vector.len()).unwrap();

    let random_with_ceiling: usize = fortuna.random_with_ceiling(100).unwrap();
    let random_with_floor: usize = fortuna.random_with_floor(100).unwrap();

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

    println!("Generated random ascii char: {}", ascii_char);

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

Fortuna relies on the seed it is given on startup as its only source of entropy.

The seed is used to pull one of 256 pre generated tables of random numbers.
The table is then shuffled by the seed and then used to return the random number.

Repeating sequences of numbers are possible but should remain unlikely.

Because of the way fortuna's tables are generated, each table contains 256 unique numbers, namely 0-255.
This means that repeating numbers are almost impossible. Only at the extreme edge of a table could a sequence of the same number be found.
There can never be a sequence of the same number thrice.

There are many more limitations to the way Fortuna generates random numbers.
But for general use, this should be enough.
