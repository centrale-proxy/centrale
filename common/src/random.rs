use rand::Rng;
use rand::rngs::OsRng;

const EXCLUDED: &[u8] = &[33, 34, 39, 47, 92, 96]; // ! " ' / \ `

fn one_random_number() -> u8 {
    loop {
        let n: u8 = OsRng.gen_range(33..=126);
        if !EXCLUDED.contains(&n) {
            return n;
        }
    }
}

pub fn random_numbers(password_length: u8) -> Vec<u8> {
    let mut numbers: Vec<u8> = Vec::new();
    for _ in 0..password_length {
        numbers.push(one_random_number());
    }
    numbers
}
