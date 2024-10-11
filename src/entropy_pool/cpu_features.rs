#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub fn get_cpu_features() -> Vec<&'static str> {
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
pub fn get_cpu_features() -> Vec<&'static str> {
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
pub fn get_cpu_features() -> Vec<&'static str> {
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
