use rand::Rng;
use rand::rngs::OsRng;

fn one_random_number() -> u8 {
    // GENERATE RANDOM NUMBER BETWEEN 33 (!) AND 126 (~)
    // THIS WILL REPRESENT A (VALID FOR PASSWORD) CHARACTER IN ASCII TABLE
    OsRng.gen_range(33..=126)
}

pub fn random_numbers_32() -> Vec<u8> {
    // PASSWORD WILL HAVE LENGTH BETWEEN 55 AND 64
    let password_length: u8 = 32;
    // CREATE 32 RANDOM NUMBERS
    let mut numbers: Vec<u8> = Vec::new();
    for _ in 0..password_length {
        let number: u8 = one_random_number();
        numbers.push(number);
    }
    numbers
}
