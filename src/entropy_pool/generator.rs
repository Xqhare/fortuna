use std::time::{Instant, SystemTime};

// linux spec use:
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;

// Windows spec use
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use crate::entropy_pool::cpu_features::get_cpu_features;

pub fn generate_seeded_pool(seed: &Vec<u8>, iterations: u32) -> Vec<u8> {
    let mut iteration_seed: Vec<u8> = Vec::new();
    if seed.len() <= 50 {
        // probably always a bit too large, should still save on heap allocations though
        iteration_seed.reserve(seed.len() * 3);
        for s in seed {
            iteration_seed.push(*s);
            if *s != 0 {
                if iterations % 9 != 0 {
                    for n in 0..=iterations % 9 {
                        let s1: u16 = (*s as u16 >> n) as u16;
                        let s2: u16 = ((*s as u16) << n) as u16;
                        iteration_seed.extend_from_slice(&s1.to_le_bytes());
                        iteration_seed.extend_from_slice(&s2.to_le_bytes());
                    }
                }
            }
        }
    } else {
        iteration_seed = seed.clone();
    }
    
    let mut reversed_seed = iteration_seed.clone();
    reversed_seed.reverse();

    // Combining seeds

    let combined_pool: Vec<u8> = {
        let mut all_matrix_matrix: Vec<u8> = Vec::new();
        for i in 0..iteration_seed.len() {
            for j in 0..reversed_seed.len() {
                let tmp = {
                    if reversed_seed[j] == 0 {
                        2
                    } else {
                        reversed_seed[j]
                    }
                };
                let tmp_bind = iteration_seed[i].overflowing_mul(tmp.into()).0;
                all_matrix_matrix.append(&mut tmp_bind.to_le_bytes().to_vec());
            }
        }

        let mut all_matrix_divided: Vec<u8> = Vec::new();
        for i in 0..iteration_seed.len() {
            for j in 0..reversed_seed.len() {
                let tmp = {
                    if reversed_seed[j] == 0 {
                        2
                    } else {
                        reversed_seed[j]
                    }
                };
                let tmp_bind = iteration_seed[i].overflowing_div(tmp.into()).0;
                all_matrix_divided.append(&mut tmp_bind.to_le_bytes().to_vec());
            }
        }

        let mut all_matrix_mul_with_extrema: Vec<u8> = Vec::new();
        for i in 0..iteration_seed.len() {
            for j in 0..reversed_seed.len() {
                let tmp = {
                    if reversed_seed[j] == 0 {
                        2
                    } else {
                        reversed_seed[j]
                    }
                };
                if j % 2 == 0 && i % 3 == 0 {
                    let tmp_bind: u8 = iteration_seed[i].overflowing_mul(tmp.into()).0;
                    all_matrix_mul_with_extrema.append(&mut tmp_bind.to_le_bytes().to_vec());
                } else {
                    let tmp_bind: u8 = iteration_seed[i].overflowing_div(tmp.into()).0;
                    all_matrix_mul_with_extrema.append(&mut tmp_bind.to_le_bytes().to_vec());
                }
            }
        }

        let tmp_zip = all_matrix_matrix.iter().zip(all_matrix_divided.iter());
        let mut tmp = Vec::new();
        for (i, j) in tmp_zip {
            tmp.push(*i);
            tmp.push(*j);
        }
        let tmp_zip2 = tmp.iter().zip(all_matrix_mul_with_extrema.iter());
        let mut tmp2 = Vec::new();
        for (i, j) in tmp_zip2 {
            tmp2.push(*i);
            tmp2.push(*j);
        }
        tmp2
    };

    let mut scrambled_pool: Vec<u8> = Vec::new();
    for i in 0..combined_pool.len() {
        scrambled_pool.push(combined_pool[(combined_pool[i] as usize) % combined_pool.len()]);
    }
    scrambled_pool
}

pub fn generate_entropy_pool() -> Vec<u8> {
    let time_now = Instant::now();

    // time
    let system_time = {
        // Hacky af, but works...
        let system_time_string = format!("{:?}", SystemTime::now());
        // remove all non-ascii digits (0-9)
        let store = {
            let mut tmp: Vec<u8> = Vec::new();
            for c in system_time_string.chars() {
                if c.is_ascii_digit() {
                    tmp.push(c as u8 - b'0');
                }
            }
            tmp
        };
        // this inverts the timestamp, putting the least changing bits last.
        let out = {
            let mut tmp: Vec<u8> = Vec::new();
            for t in store {
                if t == 0 {
                    tmp.insert(0, 1);
                } else if t == 1 {
                    tmp.insert(0, 2);
                } else {
                    tmp.insert(0, t);
                }
            }
            tmp
        };
        out
    };
    let system_time_dur = time_now.elapsed().as_nanos();

    // matrix math time
    // split system_time into two parts and multiply them together
    let matrix_time_dur = Instant::now();
    // multiply, take first element of vec1 and multiply with every element of vec2
    let mut sys_time_matrix: Vec<u8> = Vec::new();
    for i in 0..system_time.len() {
        for j in 0..system_time.len() {
            sys_time_matrix.push(system_time[i].saturating_mul(system_time[j]) % 255);
        }
    }
    let matrix_time_spend_in_nsec = matrix_time_dur.elapsed().as_nanos();
    let complete_system_time_in_nsec = time_now.elapsed().as_nanos();

    // CPU features
    // riscv / aarch64 / x86_64
    let cpu_time_dur = Instant::now();
    let cpu_features = get_cpu_features();
    let cpu_time_spend_in_nsec = cpu_time_dur.elapsed().as_nanos();

    let salt_time_dur = Instant::now();

    // salt itself will never change on the same machine, if executed from the same path
    let mut salt: Vec<u8> = Vec::new();

    // fs part 1
    let fs_start_time = Instant::now();
    salt.append(&mut fs_part_1());
    let fs_time_spend_in_nsec = fs_start_time.elapsed().as_nanos();

    if cpu_features.is_empty() {
        // if no CPU features detected, fallback to pre-generated salt
        // As CPU features do not change on the same machine anyway, this should be fine
        let mut pre_generated = vec![
            195, 15, 51, 98, 244, 101, 246, 245, 194, 184, 82, 102, 170, 119, 58, 233, 92, 9, 91,
            170, 15, 45, 220, 17, 34, 110, 241, 177, 33, 227, 14, 50, 197, 23, 198, 83, 218, 168,
            34, 18, 49, 224, 42, 160, 178, 80, 218, 43, 27, 225, 50, 240, 65, 187, 133, 206, 17,
            123, 135, 130, 153, 107, 185, 84, 156, 45, 232, 19, 192, 15, 198, 95, 147, 240, 150,
            180, 210, 254, 149, 133, 99, 250, 111, 183, 211, 6, 135, 95, 120, 33, 154, 209, 42,
            238, 28, 107, 130, 110, 164, 8, 212, 103, 28, 56, 240, 41, 166, 149, 142, 96, 254, 155,
            214, 156, 51, 0, 105, 21, 171, 65, 177, 165, 234, 35, 230,
        ];
        salt.append(&mut pre_generated);
    } else {
        for feature in cpu_features {
            let tmp = feature.as_bytes();
            salt.append(&mut tmp.to_vec());
        }
    }

    // fs part 2
    let fs_start_time2 = Instant::now();
    salt.append(&mut fs_part_2());
    let fs_time_spend_in_nsec2 = fs_start_time2.elapsed().as_nanos();

    let salt_time_spend_in_nsec = salt_time_dur.elapsed().as_nanos();
    let time_spend_in_nsec = time_now.elapsed().as_nanos();

    let all_time_spend_vec = vec![
        system_time_dur,
        matrix_time_spend_in_nsec,
        complete_system_time_in_nsec,
        fs_time_spend_in_nsec,
        fs_time_spend_in_nsec2,
        salt_time_spend_in_nsec,
        cpu_time_spend_in_nsec,
        time_spend_in_nsec,
    ];

    let mut all_time_spend_matrix: Vec<u8> = Vec::new();
    for i in 0..all_time_spend_vec.len() {
        for j in 0..all_time_spend_vec.len() {
            let tmp_bind = all_time_spend_vec[i].saturating_mul(all_time_spend_vec[j]);
            all_time_spend_matrix.append(&mut tmp_bind.to_le_bytes().to_vec());
        }
    }
    all_time_spend_matrix.retain(|&x| x != 0 && x != 255);

    let salted_time_spend_matrix = {
        let tmp_zip = all_time_spend_matrix.iter().zip(salt.iter());
        let mut tmp = Vec::new();
        for (i, j) in tmp_zip {
            tmp.push(i);
            tmp.push(j);
        }
        tmp
    };

    let mut all_matrix_matrix: Vec<u8> = Vec::new();
    for i in 0..all_time_spend_matrix.len() {
        for j in 0..sys_time_matrix.len() {
            let tmp = {
                if sys_time_matrix[j] == 0 {
                    2
                } else {
                    sys_time_matrix[j]
                }
            };
            let tmp_bind = all_time_spend_matrix[i].overflowing_mul(tmp.into()).0;
            all_matrix_matrix.append(&mut tmp_bind.to_le_bytes().to_vec());
        }
    }

    let mut all_matrix_divided: Vec<u8> = Vec::new();
    for i in 0..all_time_spend_matrix.len() {
        for j in 0..sys_time_matrix.len() {
            let tmp = {
                if sys_time_matrix[j] == 0 {
                    2
                } else {
                    sys_time_matrix[j]
                }
            };
            let tmp_bind = all_time_spend_matrix[i].overflowing_div(tmp.into()).0;
            all_matrix_divided.append(&mut tmp_bind.to_le_bytes().to_vec());
        }
    }

    let mut all_matrix_mul_with_extrema: Vec<u8> = Vec::new();
    for i in 0..all_time_spend_matrix.len() {
        for j in 0..sys_time_matrix.len() {
            let tmp = {
                if sys_time_matrix[j] == 0 {
                    2
                } else {
                    sys_time_matrix[j]
                }
            };
            if j % 2 == 0 && i % 3 == 0 {
                let tmp_bind: u8 = all_time_spend_matrix[i].overflowing_mul(tmp.into()).0;
                all_matrix_mul_with_extrema.append(&mut tmp_bind.to_le_bytes().to_vec());
            } else {
                let tmp_bind: u8 = all_time_spend_matrix[i].overflowing_div(tmp.into()).0;
                all_matrix_mul_with_extrema.append(&mut tmp_bind.to_le_bytes().to_vec());
            }
        }
    }

    let all_matrix_combined = {
        if salted_time_spend_matrix[all_matrix_matrix[0] as usize % salted_time_spend_matrix.len()]
            % 2
            == 0
        {
            let tmp_zip = all_matrix_matrix.iter().zip(all_matrix_divided.iter());
            let mut tmp_comb = Vec::new();
            for (a, b) in tmp_zip {
                tmp_comb.push(a);
                tmp_comb.push(b);
            }
            let tmp_zip2 = tmp_comb.iter().zip(all_matrix_mul_with_extrema.iter());
            let mut tmp_comb2 = Vec::new();
            for (a, b) in tmp_zip2 {
                tmp_comb2.push(*a);
                tmp_comb2.push(b);
            }
            tmp_comb2
        } else {
            let tmp_zip = all_matrix_divided.iter().zip(all_matrix_matrix.iter());
            let mut tmp_comb = Vec::new();
            for (a, b) in tmp_zip {
                tmp_comb.push(a);
                tmp_comb.push(b);
            }
            let tmp_zip2 = tmp_comb.iter().zip(all_matrix_mul_with_extrema.iter());
            let mut tmp_comb2 = Vec::new();
            for (a, b) in tmp_zip2 {
                tmp_comb2.push(*a);
                tmp_comb2.push(b);
            }
            tmp_comb2
        }
    };

    let mut scrambled_pool: Vec<u8> = Vec::new();
    let salted_len = salted_time_spend_matrix.len();
    let mut salted_index_counter = 0;
    for _ in 0..all_matrix_combined.len() {
        // first random index to pull, then push
        let salted_index = salted_time_spend_matrix[salted_index_counter];
        salted_index_counter += 1;
        if salted_index_counter == salted_len {
            salted_index_counter = 0;
        }
        let tmp_index = {
            if *salted_index as usize >= all_matrix_combined.len() {
                *salted_index as usize % all_matrix_combined.len()
            } else {
                *salted_index as usize
            }
        };
        scrambled_pool.push(*all_matrix_combined[tmp_index]);
    }
    return scrambled_pool;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
/// Linux or mac
fn fs_part_1() -> Vec<u8> {
    let mut salt: Vec<u8> = Vec::new();
    if let Ok(directory) = std::env::current_dir() {
        if let Ok(try_dir_nest_depth) = TryInto::<u8>::try_into(directory.ancestors().count()) {
            salt.push(try_dir_nest_depth);
        }
        if let Some(root_dir) = directory.ancestors().nth(directory.ancestors().count() - 1) {
            if let Ok(metadata) = root_dir.metadata() {
                if let Ok(root_dir_size) = TryInto::<u8>::try_into(metadata.size()) {
                    salt.push(root_dir_size);
                }
            }
        }
    }
    salt
}

#[cfg(target_os = "windows")]
/// Windows only
fn fs_part_1() -> Vec<u8> {
    let mut salt: Vec<u8> = Vec::new();
    if let Ok(directory) = std::env::current_dir() {
        if let Some(root_dir) = directory.ancestors().nth(directory.ancestors().count() - 1) {
            if let Ok(metadata) = root_dir.metadata() {
                if let Ok(root_dir_size) = TryInto::<u8>::try_into(metadata.file_size()) {
                    salt.push(root_dir_size);
                }
            }
        }
    }
    salt
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
/// Linux or mac
fn fs_part_2() -> Vec<u8> {

    let mut salt: Vec<u8> = Vec::new();
    if let Ok(directory) = std::env::current_dir() {
        if let Some(root_dir) = directory.ancestors().nth(directory.ancestors().count() - 1) {
            if let Ok(metadata) = root_dir.metadata() {
                if let Ok(root_dir_device) = TryInto::<u8>::try_into(metadata.dev()) {
                    salt.push(root_dir_device);
                }
                if let Ok(root_dir_ino) = TryInto::<u8>::try_into(metadata.ino()) {
                    salt.push(root_dir_ino);
                }
            }
        }
    }
    salt
}

#[cfg(target_os = "windows")]
/// windows only
fn fs_part_2() -> Vec<u8> {

    let mut salt: Vec<u8> = Vec::new();
    if let Ok(directory) = std::env::current_dir() {
        if let Some(root_dir) = directory.ancestors().nth(directory.ancestors().count() - 1) {
            if let Ok(metadata) = root_dir.metadata() {
                if let Ok(root_last_access) = TryInto::<u8>::try_into(metadata.last_access_time()) {
                    salt.push(root_last_access);
                }
                if let Ok(root_last_write) = TryInto::<u8>::try_into(metadata.last_write_time()) {
                    salt.push(root_last_write);
                }
            }
        }
    }
    salt
}
