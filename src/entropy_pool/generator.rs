use std::{os::unix::fs::MetadataExt, time::{Instant, SystemTime}};

use crate::entropy_pool::cpu_features::get_cpu_features;

pub fn generate_entropy_pool(with_salt: bool, custom_salt: Option<Vec<u8>>, with_entropy: bool, custom_entropy: Vec<u8>) -> Vec<u8> {
    let time_now = Instant::now();

    // time
    let system_time = {
        // Hacky af, but works...
        let system_time_string = format!("{:?}", SystemTime::now());
        let store = {
            let mut tmp: Vec<u8> = Vec::new();
            for c in system_time_string.chars() {
                if c.is_ascii_digit() {
                    tmp.push(c as u8 - b'0');
                }
            }
            tmp
        };
        // this inverts the Unix timestamp, putting the least changing bits last.
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
    
    // fs
    // 1. used space on disk
    // 2. size of root dir
    let fs_start_time = Instant::now();
    let curr_dir = std::env::current_dir().unwrap();
    let dir_nest_depth = curr_dir.ancestors().count();
    let root_dir = curr_dir.ancestors().nth(curr_dir.ancestors().count() - 1).unwrap();
    let root_dir_size = root_dir.metadata().unwrap().size();
    let root_dir_device = root_dir.metadata().unwrap().dev();
    let root_dir_ino = root_dir.metadata().unwrap().ino();
    let fs_time_spend_in_nsec = fs_start_time.elapsed().as_nanos();

    // CPU features
    // ARM / Aarch64
    let cpu_time_dur = Instant::now();
    let cpu_features = get_cpu_features();
    let cpu_time_spend_in_nsec = cpu_time_dur.elapsed().as_nanos();

    let time_spend_in_nsec = time_now.elapsed().as_nanos();

    let all_time_spend_vec = vec![system_time_dur, matrix_time_spend_in_nsec, complete_system_time_in_nsec, fs_time_spend_in_nsec, cpu_time_spend_in_nsec, time_spend_in_nsec];

    let mut all_time_spend_matrix: Vec<u8> = Vec::new();
    for i in 0..all_time_spend_vec.len() {
        for j in 0..all_time_spend_vec.len() {
            let tmp_bind = all_time_spend_vec[i].saturating_mul(all_time_spend_vec[j]);
            all_time_spend_matrix.append(&mut tmp_bind.to_le_bytes().to_vec());
        }
    }
    all_time_spend_matrix.retain(|&x| x != 0);
    all_time_spend_matrix.retain(|&x| x != 255);

    let mut salt: Vec<u8> = Vec::new();
    let try_dir_nest_depth = TryInto::<u8>::try_into(dir_nest_depth);
    if try_dir_nest_depth.is_ok() {
        salt.push(try_dir_nest_depth.unwrap());
    }
    let try_root_dir_size = TryInto::<u8>::try_into(root_dir_size);
    if try_root_dir_size.is_ok() {
        salt.push(try_root_dir_size.unwrap());
    }
    if with_salt {
        for (feature, custom_salt) in cpu_features.iter().zip(custom_salt.unwrap().iter()) {
            let tmp = feature.as_bytes();
            salt.append(&mut tmp.to_vec());
            salt.push(*custom_salt);
        }
    } else {
        for feature in cpu_features {
            let tmp = feature.as_bytes();
            salt.append(&mut tmp.to_vec());
        }
    }
    let try_root_dir_device = TryInto::<u8>::try_into(root_dir_device);
    if try_root_dir_device.is_ok() {
        salt.push(try_root_dir_device.unwrap());
    }
    let try_root_dir_ino = TryInto::<u8>::try_into(root_dir_ino);
    if try_root_dir_ino.is_ok() {
        salt.push(try_root_dir_ino.unwrap());
    }

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
        if with_entropy {
            if salted_time_spend_matrix[all_matrix_matrix[0] as usize] % 2 == 0 {
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
                let tmp_zip3 = tmp_comb2.iter().zip(custom_entropy.iter());
                let mut tmp_comb3 = Vec::new();
                for (a, b) in tmp_zip3 {
                    tmp_comb3.push(*a);
                    tmp_comb3.push(b);
                }
                tmp_comb3
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
                let tmp_zip3 = tmp_comb2.iter().zip(custom_entropy.iter());
                let mut tmp_comb3 = Vec::new();
                for (a, b) in tmp_zip3 {
                    tmp_comb3.push(*a);
                    tmp_comb3.push(b);
                }
                tmp_comb3
            }
        } else {
            if salted_time_spend_matrix[all_matrix_matrix[0] as usize] % 2 == 0 {
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
        scrambled_pool.push(*all_matrix_combined[*salted_index as usize]);
    }
    return scrambled_pool;
}
