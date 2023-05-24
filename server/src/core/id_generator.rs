

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const BASE: usize = ALPHABET.len();

pub fn seed_to_id(seed: usize) -> String{

    let mut seed = seed;
    let mut result = String::new();
    while seed > 0 {
        result.push(ALPHABET.as_bytes()[seed%BASE] as char);
        seed /= BASE;
    }
    result
}

#[cfg(test)]
mod tests {

    use super::seed_to_id;
    #[test]
    fn generator_test() {
        assert_eq!(seed_to_id(1), "b");
        assert_eq!(seed_to_id(2), "c");
        assert_eq!(seed_to_id(53), "1");
        assert_eq!(seed_to_id(61), "9");
        assert_eq!(seed_to_id(62), "ab");
        assert_eq!(seed_to_id(1000), "iq");
        assert_eq!(seed_to_id(999999), "bjme");
    }
}