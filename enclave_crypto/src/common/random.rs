use rand::RngCore;

pub fn random_bytes_by_rng(size: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    bytes
}
