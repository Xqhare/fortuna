use fortuna::Fortuna;

#[test]
fn defaults() {
    let mut fortuna = Fortuna::new();

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
fn custom_large_pool_with_regeneration() {
    let mut fortuna = Fortuna::create_size_restricted(200_000);

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
fn custom_large_pool_without_regeneration() {
    let mut fortuna = Fortuna::create_size_restricted(300_000);

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
fn custom_small_pool_with_10_regenerations() {
    let mut fortuna = Fortuna::create_size_restricted(25_000);

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
#[ignore]
// > 15s
fn custom_small_pool_with_100_regenerations() {
    let mut fortuna = Fortuna::create_size_restricted(2_500);

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
#[ignore]
// > 60s
fn custom_small_pool_with_1000_regenerations() {
    let mut fortuna = Fortuna::create_size_restricted(250);

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
        random_u8_vec.push(fortuna.random_u8());
        random_u16_vec.push(fortuna.random_u16());
        random_u32_vec.push(fortuna.random_u32());
        random_u64_vec.push(fortuna.random_u64());

        random_i8_vec.push(fortuna.random_i8());
        random_i16_vec.push(fortuna.random_i16());
        random_i32_vec.push(fortuna.random_i32());
        random_i64_vec.push(fortuna.random_i64());

        random_f32_vec.push(fortuna.random_f32());
        random_f64_vec.push(fortuna.random_f64());

        lowercase_char_vec.push(fortuna.random_latin_char(false));
        uppercase_char_vec.push(fortuna.random_latin_char(true));

        ascii_char_vec.push(fortuna.random_ascii_char());

        random_bool_vec.push(fortuna.random_bool());

        random_u_range_vec.push(fortuna.random_from_range(0, n));
        random_u32_range_vec.push(fortuna.random_from_u32_range(0, n as u32));
        random_u64_range_vec.push(fortuna.random_from_u64_range(0, n as u64));

        random_i_range_vec.push(fortuna.random_from_i_range(-(n as isize), n as isize));
        random_i32_range_vec.push(fortuna.random_from_i32_range(-(n as i32), n as i32));
        random_i64_range_vec.push(fortuna.random_from_i64_range(-(n as i64), n as i64));

        random_f32_range_vec.push(fortuna.random_from_f32_range(-(n as f32), n as f32));
        random_f64_range_vec.push(fortuna.random_from_f64_range(-(n as f64), n as f64));

        random_index_vec.push(fortuna.random_index(n));

        random_ceiling_vec.push(fortuna.random_with_ceiling(n));
        random_floor_vec.push(fortuna.random_with_floor(n));
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
