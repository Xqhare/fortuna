use std::{os::unix::fs::MetadataExt, time::{Instant, SystemTime}};

mod entropy_pool;

fn main() {
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
    for feature in cpu_features {
        let tmp = feature.as_bytes();
        salt.append(&mut tmp.to_vec());
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
    //println!("SCRAMBLED POOL: {:?}", scrambled_pool);
    println!("COMPLETION TIME: {:?}", time_now.elapsed());
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
fn get_cpu_features() -> Vec<&'static str> {
    let mut features = Vec::new();
    if std::arch::is_riscv_feature_detected!("rv32e") {
        features.push("rv32e");
    }
    if std::arch::is_riscv_feature_detected!("rv32i") {
        features.push("rv32i");
    }
    if std::arch::is_riscv_feature_detected!("rv64i") {
        features.push("rv64i");
    }
    if std::arch::is_riscv_feature_detected!("a") {
        features.push("a");
    }
    if std::arch::is_riscv_feature_detected!("b") {
        features.push("b");
        if std::arch::is_riscv_feature_detected!("zba") {
            features.push("zba");
        }
        if std::arch::is_riscv_feature_detected!("zbb") {
            features.push("zbb");
        }
        if std::arch::is_riscv_feature_detected!("zbc") {
            features.push("zbc");
        }
        if std::arch::is_riscv_feature_detected!("zbs") {
            features.push("zbs");
        }
    }
    if std::arch::is_riscv_feature_detected!("c") {
        features.push("c");
    }
    if std::arch::is_riscv_feature_detected!("d") {
        features.push("d");
    }
    if std::arch::is_riscv_feature_detected!("f") {
        features.push("f");
    }
    if std::arch::is_riscv_feature_detected!("m") {
        features.push("m");
    }
    if std::arch::is_riscv_feature_detected!("q") {
        features.push("q");
    }
    if std::arch::is_riscv_feature_detected!("v") {
        features.push("v");
    }
    if std::arch::is_riscv_feature_detected!("zicntr") {
        features.push("zicntr");
    }
    if std::arch::is_riscv_feature_detected!("zicsr") {
        features.push("zicsr");
    }
    if std::arch::is_riscv_feature_detected!("zifencei") {
        features.push("zifencei");
    }
    if std::arch::is_riscv_feature_detected!("zihintpause") {
        features.push("zihintpause");
    }
    if std::arch::is_riscv_feature_detected!("zihpm") {
        features.push("zihpm");
    }
    if std::arch::is_riscv_feature_detected!("zk") {
        features.push("zk");
        if std::arch::is_riscv_feature_detected!("zbkb") {
            features.push("zbkb");
        }
        if std::arch::is_riscv_feature_detected!("zbkc") {
            features.push("zbkc");
        }
        if std::arch::is_riscv_feature_detected!("zbkx") {
            features.push("zbkx");
        }
        if std::arch::is_riscv_feature_detected!("zkn") {
            features.push("zkn");
            if std::arch::is_riscv_feature_detected!("zknd") {
                features.push("zknd");
            }
            if std::arch::is_riscv_feature_detected!("zkne") {
                features.push("zkne");
            }
            if std::arch::is_riscv_feature_detected!("zknh") {
                features.push("zknh");
            }
        }
        if std::arch::is_riscv_feature_detected!("zkr") {
            features.push("zkr");
        }
        if std::arch::is_riscv_feature_detected!("zks") {
            features.push("zks");
            if std::arch::is_riscv_feature_detected!("zksed") {
                features.push("zksed");
            }
            if std::arch::is_riscv_feature_detected!("zksh") {
                features.push("zksh");
            }
        }
        if std::arch::is_riscv_feature_detected!("zkt") {
            features.push("zkt");
        }
    }
    if std::arch::is_riscv_feature_detected!("zfh") {
        features.push("zfh");
    }
    if std::arch::is_riscv_feature_detected!("zfhmin") {
        features.push("zfhmin");
    }
    if std::arch::is_riscv_feature_detected!("zfinx") {
        features.push("zfinx");
    }
    if std::arch::is_riscv_feature_detected!("zdinx") {
        features.push("zdinx");
    }
    if std::arch::is_riscv_feature_detected!("zhinx") {
        features.push("zhinx");
    }
    if std::arch::is_riscv_feature_detected!("zhinxmin") {
        features.push("zhinxmin");
    }
    if std::arch::is_riscv_feature_detected!("ztso") {
        features.push("ztso");
    }
    if std::arch::is_riscv_feature_detected!("rv128i") {
        features.push("rv128i");
    }
    if std::arch::is_riscv_feature_detected!("j") {
        features.push("j");
    }
    if std::arch::is_riscv_feature_detected!("p") {
        features.push("p");
    }
    if std::arch::is_riscv_feature_detected!("zam") {
        features.push("zam");
    }
    if std::arch::is_riscv_feature_detected!("s") {
        features.push("s");
    }
    if std::arch::is_riscv_feature_detected!("svnapot") {
        features.push("svnapot");
    }
    if std::arch::is_riscv_feature_detected!("svpbmt") {
        features.push("svpbmt");
    }
    if std::arch::is_riscv_feature_detected!("svinval") {
        features.push("svinval");
    }
    if std::arch::is_riscv_feature_detected!("h") {
        features.push("h");
    }
    features
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
/// Tests for Aarch64 features
fn get_cpu_features() -> Vec<&'static str> {
    let mut features = Vec::new();
    if std::arch::is_aarch64_feature_detected!("asmid") {
        features.push("asmid");
    }
    if std::arch::is_aarch64_feature_detected!("pmull") {
        features.push("pmull");
    }
    if std::arch::is_aarch64_feature_detected!("fp") {
        features.push("fp");
    }
    if std::arch::is_aarch64_feature_detected!("fp16") {
        features.push("fp16");
    }
    if std::arch::is_aarch64_feature_detected!("sve") {
        features.push("sve");
    }
    if std::arch::is_aarch64_feature_detected!("crc") {
        features.push("crc");
    }
    if std::arch::is_aarch64_feature_detected!("lse") {
        features.push("lse");
    }
    if std::arch::is_aarch64_feature_detected!("lse2") {
        features.push("lse2");
    }
    if std::arch::is_aarch64_feature_detected!("rdm") {
        features.push("rdm");
    }
    if std::arch::is_aarch64_feature_detected!("rcpc") {
        features.push("rcpc");
    }
    if std::arch::is_aarch64_feature_detected!("rcpc2") {
        features.push("rcpc2");
    }
    if std::arch::is_aarch64_feature_detected!("dotprod") {
        features.push("dotprod");
    }
    if std::arch::is_aarch64_feature_detected!("tme") {
        features.push("tme");
    }
    if std::arch::is_aarch64_feature_detected!("fhm") {
        features.push("fhm");
    }
    if std::arch::is_aarch64_feature_detected!("dit") {
        features.push("dit");
    }
    if std::arch::is_aarch64_feature_detected!("flagm") {
        features.push("flagm");
    }
    if std::arch::is_aarch64_feature_detected!("ssbs") {
        features.push("ssbs");
    }
    if std::arch::is_aarch64_feature_detected!("sb") {
        features.push("sb");
    }
    if std::arch::is_aarch64_feature_detected!("paca") {
        features.push("paca");
    }
    if std::arch::is_aarch64_feature_detected!("pacg") {
        features.push("pacg");
    }
    if std::arch::is_aarch64_feature_detected!("dpb") {
        features.push("dpb");
    }
    if std::arch::is_aarch64_feature_detected!("dpb2") {
        features.push("dpb2");
    }
    if std::arch::is_aarch64_feature_detected!("sve2") {
        features.push("sve2");
    }
    if std::arch::is_aarch64_feature_detected!("sve2-aes") {
        features.push("sve2-aes");
    }
    if std::arch::is_aarch64_feature_detected!("sve2-sm4") {
        features.push("sve2-sm4");
    }
    if std::arch::is_aarch64_feature_detected!("sve2-sha3") {
        features.push("sve2-sha3");
    }
    if std::arch::is_aarch64_feature_detected!("sve2-bitperm") {
        features.push("sve2-bitperm");
    }
    if std::arch::is_aarch64_feature_detected!("frintts") {
        features.push("frintts");
    }
    if std::arch::is_aarch64_feature_detected!("i8mm") {
        features.push("i8mm");
    }
    if std::arch::is_aarch64_feature_detected!("f32mm") {
        features.push("f32mm");
    }
    if std::arch::is_aarch64_feature_detected!("f64mm") {
        features.push("f64mm");
    }
    if std::arch::is_aarch64_feature_detected!("bf16") {
        features.push("bf16");
    }
    if std::arch::is_aarch64_feature_detected!("rand") {
        features.push("rand");
    }
    if std::arch::is_aarch64_feature_detected!("bti") {
        features.push("bti");
    }
    if std::arch::is_aarch64_feature_detected!("mte") {
        features.push("mte");
    }
    if std::arch::is_aarch64_feature_detected!("jsconv") {
        features.push("jsconv");
    }
    if std::arch::is_aarch64_feature_detected!("fcma") {
        features.push("fcma");
    }
    if std::arch::is_aarch64_feature_detected!("aes") {
        features.push("aes");
    }
    if std::arch::is_aarch64_feature_detected!("sha2") {
        features.push("sha2");
    }
    if std::arch::is_aarch64_feature_detected!("sha3") {
        features.push("sha3");
    }
    if std::arch::is_aarch64_feature_detected!("sm4") {
        features.push("sm4");
    }
    features
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// Tests for x86 features
fn get_cpu_features() -> Vec<&'static str> {
    let mut features = Vec::new();
    if std::arch::is_x86_feature_detected!("aes"){
        features.push("aes");
    }
    if std::arch::is_x86_feature_detected!("pclmulqdq"){
        features.push("pclmulqdq");
    }
    if std::arch::is_x86_feature_detected!("rdrand"){
        features.push("rdrand");
    }
    if std::arch::is_x86_feature_detected!("rdseed"){
        features.push("rdseed");
    }
    if std::arch::is_x86_feature_detected!("tsc"){
        features.push("tsc");
    }
    if std::arch::is_x86_feature_detected!("mmx"){
        features.push("mmx");
    }
    if std::arch::is_x86_feature_detected!("sse"){
        features.push("sse");
    }
    if std::arch::is_x86_feature_detected!("sse2"){
        features.push("sse2");
    }
    if std::arch::is_x86_feature_detected!("sse3"){
        features.push("sse3");
    }
    if std::arch::is_x86_feature_detected!("ssse3"){
        features.push("ssse3");
    }
    if std::arch::is_x86_feature_detected!("sse4.1"){
        features.push("sse4.1");
    }
    if std::arch::is_x86_feature_detected!("sse4.2"){
        features.push("sse4.2");
    }
    if std::arch::is_x86_feature_detected!("sse4a"){
        features.push("sse4a");
    }
    if std::arch::is_x86_feature_detected!("sha"){
        features.push("sha");
    }
    if std::arch::is_x86_feature_detected!("avx"){
        features.push("avx");
    }
    if std::arch::is_x86_feature_detected!("avx2"){
        features.push("avx2");
    }
    if std::arch::is_x86_feature_detected!("avx512f"){
        features.push("avx512f");
    }
    if std::arch::is_x86_feature_detected!("avx512cd"){
        features.push("avx512cd");
    }
    if std::arch::is_x86_feature_detected!("avx512er"){
        features.push("avx512er");
    }
    if std::arch::is_x86_feature_detected!("avx512pf"){
        features.push("avx512pf");
    }
    if std::arch::is_x86_feature_detected!("avx512bw"){
        features.push("avx512bw");
    }
    if std::arch::is_x86_feature_detected!("avx512dq"){
        features.push("avx512dq");
    }
    if std::arch::is_x86_feature_detected!("avx512vl"){
        features.push("avx512vl");
    }
    if std::arch::is_x86_feature_detected!("avx512ifma"){
        features.push("avx512ifma");
    }
    if std::arch::is_x86_feature_detected!("avx512vbmi"){
        features.push("avx512vbmi");
    }
    if std::arch::is_x86_feature_detected!("avx512vpopcntdq"){
        features.push("avx512vpopcntdq");
    }
    if std::arch::is_x86_feature_detected!("avx512vbmi2"){
        features.push("avx512vbmi2");
    }
    if std::arch::is_x86_feature_detected!("gfni"){
        features.push("gfni");
    }
    if std::arch::is_x86_feature_detected!("vaes"){
        features.push("vaes");
    }
    if std::arch::is_x86_feature_detected!("vpclmulqdq"){
        features.push("vpclmulqdq");
    }
    if std::arch::is_x86_feature_detected!("avx512vnni"){
        features.push("avx512vnni");
    }
    if std::arch::is_x86_feature_detected!("avx512bitalg"){
        features.push("avx512bitalg");
    }
    if std::arch::is_x86_feature_detected!("avx512bf16"){
        features.push("avx512bf16");
    }
    if std::arch::is_x86_feature_detected!("avx512vp2intersect"){
        features.push("avx512vp2intersect");
    }
    if std::arch::is_x86_feature_detected!("avx512fp16"){
        features.push("avx512fp16");
    }
    if std::arch::is_x86_feature_detected!("f16c"){
        features.push("f16c");
    }
    if std::arch::is_x86_feature_detected!("fma"){
        features.push("fma");
    }
    if std::arch::is_x86_feature_detected!("bmi1"){
        features.push("bmi1");
    }
    if std::arch::is_x86_feature_detected!("bmi2"){
        features.push("bmi2");
    }
    if std::arch::is_x86_feature_detected!("abm"){
        features.push("abm");
    }
    if std::arch::is_x86_feature_detected!("lzcnt"){
        features.push("lzcnt");
    }
    if std::arch::is_x86_feature_detected!("movbe"){
        features.push("movbe");
    }
    if std::arch::is_x86_feature_detected!("xsave"){
        features.push("xsave");
    }
    if std::arch::is_x86_feature_detected!("xsaveopt"){
        features.push("xsaveopt");
    }
    if std::arch::is_x86_feature_detected!("xsavec"){
        features.push("xsavec");
    }
    if std::arch::is_x86_feature_detected!("xsaves"){
        features.push("xsaves");
    }
    if std::arch::is_x86_feature_detected!("cmpxchg16b"){
        features.push("cmpxchg16b");
    }
    if std::arch::is_x86_feature_detected!("adx"){
        features.push("adx");
    }
    if std::arch::is_x86_feature_detected!("rtm"){
        features.push("rtm");
    }
    if std::arch::is_x86_feature_detected!("movbe"){
        features.push("movbe");
    }
    if std::arch::is_x86_feature_detected!("ermsb"){
        features.push("ermsb");
    }
    features
}


    
    


