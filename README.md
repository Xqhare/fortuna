# Fortuna
A dependency-free, deterministic, panic-proof PRNG for any OS on any hardware.

Hobby project, probably not suitable for production use.

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
