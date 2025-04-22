use rand_core::CryptoRngCore;

pub struct TokenFactoryPosition {
    pub nonce: [u8; 32],   
}

impl TokenFactoryPosition {
    pub fn new<R: CryptoRngCore>(rng: &mut R) -> Self {
        let mut nonce = [0; 32];
        rng.fill_bytes(&mut nonce);

        Self { nonce }
    }
}