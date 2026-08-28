use athena::rng_api::RngApi;
use std::collections::HashMap;

use fortuna::Fortuna;

#[test]
fn defaults() {
    let mut fortuna = Fortuna::new(vec![1, 2, 3, 4]);

    let mut random_u8_vec = Vec::new();
    let mut random_u16_vec = Vec::new();
    let mut random_u32_vec = Vec::new();
    let mut random_u64_vec = Vec::new();

    let mut random_i8_vec = Vec::new();
    let mut random_i16_vec = Vec::new();
    let mut random_i32_vec = Vec::new();
    let mut random_i64_vec = Vec::new();

    let mut random_f32_vec = Vec::new();
    let mut random_f64_vec = Vec::new();

    let mut lowercase_char_vec = Vec::new();
    let mut uppercase_char_vec = Vec::new();

    let mut ascii_char_vec = Vec::new();

    let mut random_bool_vec = Vec::new();

    let mut random_u_range_vec = Vec::new();
    let mut random_u32_range_vec = Vec::new();
    let mut random_u64_range_vec = Vec::new();

    let mut random_i_range_vec = Vec::new();
    let mut random_i32_range_vec = Vec::new();
    let mut random_i64_range_vec = Vec::new();

    let mut random_f32_range_vec = Vec::new();
    let mut random_f64_range_vec = Vec::new();

    let mut random_index_vec = Vec::new();

    let mut random_ceiling_vec = Vec::new();
    let mut random_floor_vec = Vec::new();

    for n in 0..10_000 {
        random_u8_vec.push(fortuna.random_u8().unwrap());
        random_u16_vec.push(fortuna.random_u16().unwrap());
        random_u32_vec.push(fortuna.random_u32().unwrap());
        random_u64_vec.push(fortuna.random_u64().unwrap());

        random_i8_vec.push(fortuna.random_i8().unwrap());
        random_i16_vec.push(fortuna.random_i16().unwrap());
        random_i32_vec.push(fortuna.random_i32().unwrap());
        random_i64_vec.push(fortuna.random_i64().unwrap());

        random_f32_vec.push(fortuna.random_f32().unwrap());
        random_f64_vec.push(fortuna.random_f64().unwrap());

        lowercase_char_vec.push(fortuna.random_latin_char(false).unwrap());
        uppercase_char_vec.push(fortuna.random_latin_char(true).unwrap());

        ascii_char_vec.push(fortuna.random_ascii_char().unwrap());

        random_bool_vec.push(fortuna.random_bool().unwrap());

        random_u_range_vec.push(fortuna.random_from_range(0, n).unwrap());
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64).unwrap());

        random_i_range_vec.push(
            fortuna
                .random_from_i_range(-(n as isize), n as isize)
                .unwrap(),
        );
        random_i32_range_vec.push(
            fortuna
                .random_from_i32_range(-(n as i32), n as i32)
                .unwrap(),
        );
        random_i64_range_vec.push(
            fortuna
                .random_from_i64_range(-(n as i64), n as i64)
                .unwrap(),
        );

        random_f32_range_vec.push(
            fortuna
                .random_from_f32_range(-(n as f32), n as f32)
                .unwrap(),
        );
        random_f64_range_vec.push(
            fortuna
                .random_from_f64_range(-(n as f64), n as f64)
                .unwrap(),
        );

        random_index_vec.push(fortuna.random_index(n).unwrap());

        random_ceiling_vec.push(fortuna.random_with_ceiling(n).unwrap());
        random_floor_vec.push(fortuna.random_with_floor(n).unwrap());
    }

    assert_eq!(random_u8_vec.len(), 10_000);
    assert_eq!(random_u16_vec.len(), 10_000);
    assert_eq!(random_u32_vec.len(), 10_000);
    assert_eq!(random_u64_vec.len(), 10_000);

    assert_eq!(random_i8_vec.len(), 10_000);
    assert_eq!(random_i16_vec.len(), 10_000);
    assert_eq!(random_i32_vec.len(), 10_000);
    assert_eq!(random_i64_vec.len(), 10_000);

    assert_eq!(random_f32_vec.len(), 10_000);
    assert_eq!(random_f64_vec.len(), 10_000);

    assert_eq!(lowercase_char_vec.len(), 10_000);
    assert_eq!(uppercase_char_vec.len(), 10_000);

    assert_eq!(ascii_char_vec.len(), 10_000);

    assert_eq!(random_bool_vec.len(), 10_000);

    assert_eq!(random_u_range_vec.len(), 10_000);
    assert_eq!(random_u32_range_vec.len(), 10_000);
    assert_eq!(random_u64_range_vec.len(), 10_000);

    assert_eq!(random_i_range_vec.len(), 10_000);
    assert_eq!(random_i32_range_vec.len(), 10_000);
    assert_eq!(random_i64_range_vec.len(), 10_000);

    assert_eq!(random_f32_range_vec.len(), 10_000);
    assert_eq!(random_f64_range_vec.len(), 10_000);

    assert_eq!(random_index_vec.len(), 10_000);

    assert_eq!(random_ceiling_vec.len(), 10_000);
    assert_eq!(random_floor_vec.len(), 10_000);
}

#[test]
fn seeded_complete() {
    let mut fortuna = Fortuna::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

    let mut random_u8_vec = Vec::new();
    let mut random_u16_vec = Vec::new();
    let mut random_u32_vec = Vec::new();
    let mut random_u64_vec = Vec::new();

    let mut random_i8_vec = Vec::new();
    let mut random_i16_vec = Vec::new();
    let mut random_i32_vec = Vec::new();
    let mut random_i64_vec = Vec::new();

    let mut random_f32_vec = Vec::new();
    let mut random_f64_vec = Vec::new();

    let mut lowercase_char_vec = Vec::new();
    let mut uppercase_char_vec = Vec::new();

    let mut ascii_char_vec = Vec::new();

    let mut random_bool_vec = Vec::new();

    let mut random_u_range_vec = Vec::new();
    let mut random_u32_range_vec = Vec::new();
    let mut random_u64_range_vec = Vec::new();

    let mut random_i_range_vec = Vec::new();
    let mut random_i32_range_vec = Vec::new();
    let mut random_i64_range_vec = Vec::new();

    let mut random_f32_range_vec = Vec::new();
    let mut random_f64_range_vec = Vec::new();

    let mut random_index_vec = Vec::new();

    let mut random_ceiling_vec = Vec::new();
    let mut random_floor_vec = Vec::new();

    for n in 0..10_000 {
        random_u8_vec.push(fortuna.random_u8().unwrap());
        random_u16_vec.push(fortuna.random_u16().unwrap());
        random_u32_vec.push(fortuna.random_u32().unwrap());
        random_u64_vec.push(fortuna.random_u64().unwrap());

        random_i8_vec.push(fortuna.random_i8().unwrap());
        random_i16_vec.push(fortuna.random_i16().unwrap());
        random_i32_vec.push(fortuna.random_i32().unwrap());
        random_i64_vec.push(fortuna.random_i64().unwrap());

        random_f32_vec.push(fortuna.random_f32().unwrap());
        random_f64_vec.push(fortuna.random_f64().unwrap());

        lowercase_char_vec.push(fortuna.random_latin_char(false).unwrap());
        uppercase_char_vec.push(fortuna.random_latin_char(true).unwrap());

        ascii_char_vec.push(fortuna.random_ascii_char().unwrap());

        random_bool_vec.push(fortuna.random_bool().unwrap());

        random_u_range_vec.push(fortuna.random_from_range(0, n).unwrap());
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64).unwrap());

        random_i_range_vec.push(
            fortuna
                .random_from_i_range(-(n as isize), n as isize)
                .unwrap(),
        );
        random_i32_range_vec.push(
            fortuna
                .random_from_i32_range(-(n as i32), n as i32)
                .unwrap(),
        );
        random_i64_range_vec.push(
            fortuna
                .random_from_i64_range(-(n as i64), n as i64)
                .unwrap(),
        );

        random_f32_range_vec.push(
            fortuna
                .random_from_f32_range(-(n as f32), n as f32)
                .unwrap(),
        );
        random_f64_range_vec.push(
            fortuna
                .random_from_f64_range(-(n as f64), n as f64)
                .unwrap(),
        );

        random_index_vec.push(fortuna.random_index(n).unwrap());

        random_ceiling_vec.push(fortuna.random_with_ceiling(n).unwrap());
        random_floor_vec.push(fortuna.random_with_floor(n).unwrap());
    }

    assert_eq!(random_u8_vec.len(), 10_000);
    assert_eq!(random_u16_vec.len(), 10_000);
    assert_eq!(random_u32_vec.len(), 10_000);
    assert_eq!(random_u64_vec.len(), 10_000);

    assert_eq!(random_i8_vec.len(), 10_000);
    assert_eq!(random_i16_vec.len(), 10_000);
    assert_eq!(random_i32_vec.len(), 10_000);
    assert_eq!(random_i64_vec.len(), 10_000);

    assert_eq!(random_f32_vec.len(), 10_000);
    assert_eq!(random_f64_vec.len(), 10_000);

    assert_eq!(lowercase_char_vec.len(), 10_000);
    assert_eq!(uppercase_char_vec.len(), 10_000);

    assert_eq!(ascii_char_vec.len(), 10_000);

    assert_eq!(random_bool_vec.len(), 10_000);

    assert_eq!(random_u_range_vec.len(), 10_000);
    assert_eq!(random_u32_range_vec.len(), 10_000);
    assert_eq!(random_u64_range_vec.len(), 10_000);

    assert_eq!(random_i_range_vec.len(), 10_000);
    assert_eq!(random_i32_range_vec.len(), 10_000);
    assert_eq!(random_i64_range_vec.len(), 10_000);

    assert_eq!(random_f32_range_vec.len(), 10_000);
    assert_eq!(random_f64_range_vec.len(), 10_000);

    assert_eq!(random_index_vec.len(), 10_000);

    assert_eq!(random_ceiling_vec.len(), 10_000);
    assert_eq!(random_floor_vec.len(), 10_000);
}

#[test]
fn pool_with_regeneration() {
    let mut fortuna = Fortuna::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

    let mut random_u8_vec = Vec::new();
    let mut random_u16_vec = Vec::new();
    let mut random_u32_vec = Vec::new();
    let mut random_u64_vec = Vec::new();

    let mut random_i8_vec = Vec::new();
    let mut random_i16_vec = Vec::new();
    let mut random_i32_vec = Vec::new();
    let mut random_i64_vec = Vec::new();

    let mut random_f32_vec = Vec::new();
    let mut random_f64_vec = Vec::new();

    let mut lowercase_char_vec = Vec::new();
    let mut uppercase_char_vec = Vec::new();

    let mut ascii_char_vec = Vec::new();

    let mut random_bool_vec = Vec::new();

    let mut random_u_range_vec = Vec::new();
    let mut random_u32_range_vec = Vec::new();
    let mut random_u64_range_vec = Vec::new();

    let mut random_i_range_vec = Vec::new();
    let mut random_i32_range_vec = Vec::new();
    let mut random_i64_range_vec = Vec::new();

    let mut random_f32_range_vec = Vec::new();
    let mut random_f64_range_vec = Vec::new();

    let mut random_index_vec = Vec::new();

    let mut random_ceiling_vec = Vec::new();
    let mut random_floor_vec = Vec::new();

    // polls 250 000 times (25 items * 10 000 iterations)
    for n in 0..10_000 {
        random_u8_vec.push(fortuna.random_u8().unwrap());
        random_u16_vec.push(fortuna.random_u16().unwrap());
        random_u32_vec.push(fortuna.random_u32().unwrap());
        random_u64_vec.push(fortuna.random_u64().unwrap());

        random_i8_vec.push(fortuna.random_i8().unwrap());
        random_i16_vec.push(fortuna.random_i16().unwrap());
        random_i32_vec.push(fortuna.random_i32().unwrap());
        random_i64_vec.push(fortuna.random_i64().unwrap());

        random_f32_vec.push(fortuna.random_f32().unwrap());
        random_f64_vec.push(fortuna.random_f64().unwrap());

        lowercase_char_vec.push(fortuna.random_latin_char(false).unwrap());
        uppercase_char_vec.push(fortuna.random_latin_char(true).unwrap());

        ascii_char_vec.push(fortuna.random_ascii_char().unwrap());

        random_bool_vec.push(fortuna.random_bool().unwrap());

        random_u_range_vec.push(fortuna.random_from_range(0, n).unwrap());
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64).unwrap());

        random_i_range_vec.push(
            fortuna
                .random_from_i_range(-(n as isize), n as isize)
                .unwrap(),
        );
        random_i32_range_vec.push(
            fortuna
                .random_from_i32_range(-(n as i32), n as i32)
                .unwrap(),
        );
        random_i64_range_vec.push(
            fortuna
                .random_from_i64_range(-(n as i64), n as i64)
                .unwrap(),
        );

        random_f32_range_vec.push(
            fortuna
                .random_from_f32_range(-(n as f32), n as f32)
                .unwrap(),
        );
        random_f64_range_vec.push(
            fortuna
                .random_from_f64_range(-(n as f64), n as f64)
                .unwrap(),
        );

        random_index_vec.push(fortuna.random_index(n).unwrap());

        random_ceiling_vec.push(fortuna.random_with_ceiling(n).unwrap());
        random_floor_vec.push(fortuna.random_with_floor(n).unwrap());
    }

    assert_eq!(random_u8_vec.len(), 10_000);
    assert_eq!(random_u16_vec.len(), 10_000);
    assert_eq!(random_u32_vec.len(), 10_000);
    assert_eq!(random_u64_vec.len(), 10_000);

    assert_eq!(random_i8_vec.len(), 10_000);
    assert_eq!(random_i16_vec.len(), 10_000);
    assert_eq!(random_i32_vec.len(), 10_000);
    assert_eq!(random_i64_vec.len(), 10_000);

    assert_eq!(random_f32_vec.len(), 10_000);
    assert_eq!(random_f64_vec.len(), 10_000);

    assert_eq!(lowercase_char_vec.len(), 10_000);
    assert_eq!(uppercase_char_vec.len(), 10_000);

    assert_eq!(ascii_char_vec.len(), 10_000);

    assert_eq!(random_bool_vec.len(), 10_000);

    assert_eq!(random_u_range_vec.len(), 10_000);
    assert_eq!(random_u32_range_vec.len(), 10_000);
    assert_eq!(random_u64_range_vec.len(), 10_000);

    assert_eq!(random_i_range_vec.len(), 10_000);
    assert_eq!(random_i32_range_vec.len(), 10_000);
    assert_eq!(random_i64_range_vec.len(), 10_000);

    assert_eq!(random_f32_range_vec.len(), 10_000);
    assert_eq!(random_f64_range_vec.len(), 10_000);

    assert_eq!(random_index_vec.len(), 10_000);

    assert_eq!(random_ceiling_vec.len(), 10_000);
    assert_eq!(random_floor_vec.len(), 10_000);
}

#[test]
fn pool_with_regeneration_large_seed() {
    let mut fortuna = Fortuna::new(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ]);

    let mut random_u8_vec = Vec::new();
    let mut random_u16_vec = Vec::new();
    let mut random_u32_vec = Vec::new();
    let mut random_u64_vec = Vec::new();

    let mut random_i8_vec = Vec::new();
    let mut random_i16_vec = Vec::new();
    let mut random_i32_vec = Vec::new();
    let mut random_i64_vec = Vec::new();

    let mut random_f32_vec = Vec::new();
    let mut random_f64_vec = Vec::new();

    let mut lowercase_char_vec = Vec::new();
    let mut uppercase_char_vec = Vec::new();

    let mut ascii_char_vec = Vec::new();

    let mut random_bool_vec = Vec::new();

    let mut random_u_range_vec = Vec::new();
    let mut random_u32_range_vec = Vec::new();
    let mut random_u64_range_vec = Vec::new();

    let mut random_i_range_vec = Vec::new();
    let mut random_i32_range_vec = Vec::new();
    let mut random_i64_range_vec = Vec::new();

    let mut random_f32_range_vec = Vec::new();
    let mut random_f64_range_vec = Vec::new();

    let mut random_index_vec = Vec::new();

    let mut random_ceiling_vec = Vec::new();
    let mut random_floor_vec = Vec::new();

    // polls 250 000 times (25 items * 10 000 iterations)
    for n in 0..10_000 {
        random_u8_vec.push(fortuna.random_u8().unwrap());
        random_u16_vec.push(fortuna.random_u16().unwrap());
        random_u32_vec.push(fortuna.random_u32().unwrap());
        random_u64_vec.push(fortuna.random_u64().unwrap());

        random_i8_vec.push(fortuna.random_i8().unwrap());
        random_i16_vec.push(fortuna.random_i16().unwrap());
        random_i32_vec.push(fortuna.random_i32().unwrap());
        random_i64_vec.push(fortuna.random_i64().unwrap());

        random_f32_vec.push(fortuna.random_f32().unwrap());
        random_f64_vec.push(fortuna.random_f64().unwrap());

        lowercase_char_vec.push(fortuna.random_latin_char(false).unwrap());
        uppercase_char_vec.push(fortuna.random_latin_char(true).unwrap());

        ascii_char_vec.push(fortuna.random_ascii_char().unwrap());

        random_bool_vec.push(fortuna.random_bool().unwrap());

        random_u_range_vec.push(fortuna.random_from_range(0, n).unwrap());
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64).unwrap());

        random_i_range_vec.push(
            fortuna
                .random_from_i_range(-(n as isize), n as isize)
                .unwrap(),
        );
        random_i32_range_vec.push(
            fortuna
                .random_from_i32_range(-(n as i32), n as i32)
                .unwrap(),
        );
        random_i64_range_vec.push(
            fortuna
                .random_from_i64_range(-(n as i64), n as i64)
                .unwrap(),
        );

        random_f32_range_vec.push(
            fortuna
                .random_from_f32_range(-(n as f32), n as f32)
                .unwrap(),
        );
        random_f64_range_vec.push(
            fortuna
                .random_from_f64_range(-(n as f64), n as f64)
                .unwrap(),
        );

        random_index_vec.push(fortuna.random_index(n).unwrap());

        random_ceiling_vec.push(fortuna.random_with_ceiling(n).unwrap());
        random_floor_vec.push(fortuna.random_with_floor(n).unwrap());
    }

    assert_eq!(random_u8_vec.len(), 10_000);
    assert_eq!(random_u16_vec.len(), 10_000);
    assert_eq!(random_u32_vec.len(), 10_000);
    assert_eq!(random_u64_vec.len(), 10_000);

    assert_eq!(random_i8_vec.len(), 10_000);
    assert_eq!(random_i16_vec.len(), 10_000);
    assert_eq!(random_i32_vec.len(), 10_000);
    assert_eq!(random_i64_vec.len(), 10_000);

    assert_eq!(random_f32_vec.len(), 10_000);
    assert_eq!(random_f64_vec.len(), 10_000);

    assert_eq!(lowercase_char_vec.len(), 10_000);
    assert_eq!(uppercase_char_vec.len(), 10_000);

    assert_eq!(ascii_char_vec.len(), 10_000);

    assert_eq!(random_bool_vec.len(), 10_000);

    assert_eq!(random_u_range_vec.len(), 10_000);
    assert_eq!(random_u32_range_vec.len(), 10_000);
    assert_eq!(random_u64_range_vec.len(), 10_000);

    assert_eq!(random_i_range_vec.len(), 10_000);
    assert_eq!(random_i32_range_vec.len(), 10_000);
    assert_eq!(random_i64_range_vec.len(), 10_000);

    assert_eq!(random_f32_range_vec.len(), 10_000);
    assert_eq!(random_f64_range_vec.len(), 10_000);

    assert_eq!(random_index_vec.len(), 10_000);

    assert_eq!(random_ceiling_vec.len(), 10_000);
    assert_eq!(random_floor_vec.len(), 10_000);
}

#[test]
fn is_seeded_really_seeded() {
    let mut seeded1 = Fortuna::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut seeded2 = Fortuna::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(seeded1.random_u8().unwrap(), seeded2.random_u8().unwrap());
    assert_eq!(seeded1.random_u16().unwrap(), seeded2.random_u16().unwrap());
    assert_eq!(seeded1.random_u32().unwrap(), seeded2.random_u32().unwrap());
    assert_eq!(seeded1.random_u64().unwrap(), seeded2.random_u64().unwrap());

    assert_eq!(seeded1.random_i8().unwrap(), seeded2.random_i8().unwrap());
    assert_eq!(seeded1.random_i16().unwrap(), seeded2.random_i16().unwrap());
    assert_eq!(seeded1.random_i32().unwrap(), seeded2.random_i32().unwrap());
    assert_eq!(seeded1.random_i64().unwrap(), seeded2.random_i64().unwrap());

    assert_eq!(seeded1.random_f32().unwrap(), seeded2.random_f32().unwrap());
    assert_eq!(seeded1.random_f64().unwrap(), seeded2.random_f64().unwrap());

    assert_eq!(
        seeded1.random_latin_char(false).unwrap(),
        seeded2.random_latin_char(false).unwrap()
    );
    assert_eq!(
        seeded1.random_latin_char(true).unwrap(),
        seeded2.random_latin_char(true).unwrap()
    );

    assert_eq!(
        seeded1.random_ascii_char().unwrap(),
        seeded2.random_ascii_char().unwrap()
    );

    assert_eq!(
        seeded1.random_bool().unwrap(),
        seeded2.random_bool().unwrap()
    );

    assert_eq!(
        seeded1.random_from_range(0, 10).unwrap(),
        seeded2.random_from_range(0, 10).unwrap()
    );
    assert_eq!(
        seeded1.random_from_u32_range(0, 10).unwrap(),
        seeded2.random_from_u32_range(0, 10).unwrap()
    );
    assert_eq!(
        seeded1.random_from_u64_range(0, 10).unwrap(),
        seeded2.random_from_u64_range(0, 10).unwrap()
    );

    assert_eq!(
        seeded1.random_from_i_range(-10, 10).unwrap(),
        seeded2.random_from_i_range(-10, 10).unwrap()
    );
    assert_eq!(
        seeded1.random_from_i32_range(-10, 10).unwrap(),
        seeded2.random_from_i32_range(-10, 10).unwrap()
    );
    assert_eq!(
        seeded1.random_from_i64_range(-10, 10).unwrap(),
        seeded2.random_from_i64_range(-10, 10).unwrap()
    );

    assert_eq!(
        seeded1.random_from_f32_range(-10.0, 10.0).unwrap(),
        seeded2.random_from_f32_range(-10.0, 10.0).unwrap()
    );
    assert_eq!(
        seeded1.random_from_f64_range(-10.0, 10.0).unwrap(),
        seeded2.random_from_f64_range(-10.0, 10.0).unwrap()
    );

    assert_eq!(
        seeded1.random_index(10).unwrap(),
        seeded2.random_index(10).unwrap()
    );
    assert_eq!(
        seeded1.random_with_ceiling(10).unwrap(),
        seeded2.random_with_ceiling(10).unwrap()
    );
    assert_eq!(
        seeded1.random_with_floor(10).unwrap(),
        seeded2.random_with_floor(10).unwrap()
    );
}

#[test]
fn seeded_completeness() {
    let mut fortuna = Fortuna::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat !234567890?-_.:,;<>|@€+*~#''/-()/=[{&%$§`´¸`}]".as_bytes().to_vec());
    let mut map: HashMap<u8, _> = HashMap::new();

    for _ in 0..100_000 {
        map.insert(fortuna.random_u8().unwrap(), ());
    }

    // This should Ideally be true.
    assert_eq!(map.len(), 256);

    // absolute minimum
    assert!(map.len() > 35);
}
