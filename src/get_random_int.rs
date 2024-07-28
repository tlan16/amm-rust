use rand::Rng;

pub fn get_random_i32(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn get_random_u64(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_int() {
        let result = get_random_i32(-10, 10);
        assert!(result >= -10);
        assert!(result <= 10);
    }

    #[test]
    fn test_get_random_u64() {
        let result = get_random_u64(0, 10);
        assert!(result >= 0);
        assert!(result <= 10);
    }
}
