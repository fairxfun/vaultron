mod aes_tests;
mod ecies_tests;

use rand::Rng;

fn random_len(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
