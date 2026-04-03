use fortuna::Fortuna;
use athena::rng_api::RngApi;

#[test]
fn usage_example() {
    let mut fortuna = Fortuna::new();

    let _random_u8: u8 = fortuna.random_u8().unwrap();
    let _random_u16: u16 = fortuna.random_u16().unwrap();
    let _random_u32: u32 = fortuna.random_u32().unwrap();
    let _random_u64: u64 = fortuna.random_u64().unwrap();

    let _random_i8: i8 = fortuna.random_i8().unwrap();
    let _random_i16: i16 = fortuna.random_i16().unwrap();
    let _random_i32: i32 = fortuna.random_i32().unwrap();
    let _random_i64: i64 = fortuna.random_i64().unwrap();

    let _random_f32: f32 = fortuna.random_f32().unwrap();
    let _random_f64: f64 = fortuna.random_f64().unwrap();

    let _lowercase_char: char = fortuna.random_latin_char(false).unwrap();
    let _uppercase_char: char = fortuna.random_latin_char(true).unwrap();

    let _ascii_char: char = fortuna.random_ascii_char().unwrap();

    let _random_bool: bool = fortuna.random_bool().unwrap();

    let _random_u_range: usize = fortuna.random_from_range(0, 100).unwrap();
    let _random_u64_range: u64 = fortuna.random_from_u64_range(0, 100).unwrap();

    let _random_i_range: isize = fortuna.random_from_i_range(-100, 100).unwrap();
    let _random_i64_range: i64 = fortuna.random_from_i64_range(-100, 100).unwrap();

    let _random_f32_range = fortuna.random_from_f32_range(-100.0, 100.0).unwrap();
    let _random_f64_range = fortuna.random_from_f64_range(-100.0, 100.0).unwrap();

    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _random_index: usize = fortuna.random_index(vector.len()).unwrap();

    let _random_with_ceiling: usize = fortuna.random_with_ceiling(100).unwrap();
    let _random_with_floor: usize = fortuna.random_with_floor(100).unwrap();
}

#[test]
fn new_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: u8 = fortuna.random_u8().unwrap();
  let _random_bool: bool = fortuna.random_bool().unwrap();
}

#[test]
fn create_size_restricted_example() {
  let mut fortuna = Fortuna::create_size_restricted(1_000);
  let _random_number: u8 = fortuna.random_u8().unwrap();
  let _random_bool: bool = fortuna.random_bool().unwrap();
}

#[test]
fn create_seeded_example() {
  let mut fortuna = Fortuna::create_seeded(vec![1, 2, 3, 4]);
  let _random_number: u8 = fortuna.random_u8().unwrap();
  let _random_bool: bool = fortuna.random_bool().unwrap();
}

#[test]
fn create_seeded_size_restricted_example() {
  let mut fortuna = Fortuna::create_seeded_size_restricted(vec![1, 2, 3, 4], 1_000);
  let _random_number: u8 = fortuna.random_u8().unwrap();
  let _random_bool: bool = fortuna.random_bool().unwrap();
}

#[test]
fn random_u8_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: u8 = fortuna.random_u8().unwrap();
}

#[test]
fn random_u16_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: u16 = fortuna.random_u16().unwrap();
}

#[test]
fn random_u32_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: u32 = fortuna.random_u32().unwrap();
}

#[test]
fn random_u64_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: u64 = fortuna.random_u64().unwrap();
}

#[test]
fn random_i8_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: i8 = fortuna.random_i8().unwrap();
}

#[test]
fn random_i16_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: i16 = fortuna.random_i16().unwrap();
}

#[test]
fn random_i32_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: i32 = fortuna.random_i32().unwrap();
}

#[test]
fn random_i64_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: i64 = fortuna.random_i64().unwrap();
}

#[test]
fn random_f32_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: f32 = fortuna.random_f32().unwrap();
}

#[test]
fn random_f64_example() {
  let mut fortuna = Fortuna::new();
  let _random_number: f64 = fortuna.random_f64().unwrap();
}

#[test]
fn random_latin_char_example() {
  let mut fortuna = Fortuna::new();
  let _random_char_uppercase: char = fortuna.random_latin_char(true).unwrap();
  let _random_char_lowercase: char = fortuna.random_latin_char(false).unwrap();
}

#[test]
fn random_ascii_char_example() {
  let mut fortuna = Fortuna::new();
  let _random_char: char = fortuna.random_ascii_char().unwrap();
}

#[test]
fn random_bool_example() {
  let mut fortuna = Fortuna::new();
  let _random_bool: bool = fortuna.random_bool().unwrap();
}

#[test]
fn random_from_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_range(0, 100).unwrap();
}

#[test]
fn random_from_u64_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_u64_range(0, 100).unwrap();
}

#[test]
fn random_from_f32_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_f32_range(0.1, 100.1).unwrap();
}

#[test]
fn random_from_f64_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_f64_range(-100.1, 100.1).unwrap();
}

#[test]
fn random_from_i32_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_i32_range(-100, 100).unwrap();
}

#[test]
fn random_from_i64_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_i64_range(-100, 100).unwrap();
}

#[test]
fn random_from_i_range_example() {
    let mut fortuna = Fortuna::new();
    let _chosen_element = fortuna.random_from_i_range(-100, 100).unwrap();
}

#[test]
fn random_index_example() {
    let mut fortuna = Fortuna::new();
    let collection = (0..100).collect::<Vec<usize>>();
    let random_index = fortuna.random_index(collection.len()).unwrap();
    let _ = collection[random_index];
}

#[test]
fn random_with_ceiling_example() {
    let mut fortuna = Fortuna::new();
    for n in 100000..100010  {
         let _answ = fortuna.random_with_ceiling(n).unwrap();
    }
}

#[test]
fn random_with_floor_example() {
    let mut fortuna = Fortuna::new();
    for n in 0..10  {
         let _answ = fortuna.random_with_floor(n).unwrap();
    }
}
