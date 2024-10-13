#[cfg(test)]
mod tests {
    use super::super::EntropyPool;
    #[test]
    #[ignore]
    /// takes a while to run > 60s
    fn all_bytes_generated() {
        let mut pool = EntropyPool::new();
        let mut ok = false;
        let mut check_vec = {
            let mut out: Vec<u8> = Vec::new();
            for n in 0..=255 {
                out.push(n);
            }
            out
        };
        while !ok {
            let byte = pool.get_random_byte();
            if check_vec.contains(&byte) {
                println!("FOUND {:?}", byte);
                check_vec.remove(check_vec.iter().position(|&x| x == byte).unwrap());
            }
            if check_vec.is_empty() {
                ok = true;
            }
        }
        assert!(check_vec.is_empty());
    }

    #[test]
    /// fast
    fn create_and_use_normal_1mil_bytes() {
        let mut pool = EntropyPool::new();
        let mut ok = 1_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }

    #[test]
    #[ignore]
    /// > 40s
    fn create_and_use_restricted_1mil_bytes_small_size() {
        let mut pool = EntropyPool::create_size_restricted(1_000);
        let mut ok = 1_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }

    #[test]
    /// fast
    fn create_and_use_restricted_1mil_bytes_medium_size() {
        let mut pool = EntropyPool::create_size_restricted(25_000);
        let mut ok = 1_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }

    #[test]
    /// fast
    fn create_and_use_restricted_1mil_bytes_large_size() {
        let mut pool = EntropyPool::create_size_restricted(100_000);
        let mut ok = 1_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }

    #[test]
    /// fast
    fn create_and_use_restricted_1mil_bytes_very_large_size() {
        let mut pool = EntropyPool::create_size_restricted(500_000);
        let mut ok = 1_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }

    #[test]
    #[ignore]
    /// > 20s
    fn create_and_use_restricted_1mil_bytes_stupidly_large_size() {
        let mut pool = EntropyPool::create_size_restricted(10_000_000);
        let mut ok = 100_000_000;
        for _ in 0..ok {
            pool.get_random_byte();
            ok -= 1;
        }
        assert!(ok == 0);
    }
}
