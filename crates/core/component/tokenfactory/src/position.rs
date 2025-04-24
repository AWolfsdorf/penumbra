use rand_core::CryptoRngCore;

use penumbra_sdk_proto::core::component::tokenfactory::v1alpha as pb;
use penumbra_sdk_proto::serializers::bech32str;
use serde::{Deserialize, Serialize};

pub struct TokenFactoryPosition {
    pub nonce: [u8; 32],
}

impl TokenFactoryPosition {
    pub fn new<R: CryptoRngCore>(mut rng: R) -> Self {
        let mut nonce = [0; 32];
        rng.fill_bytes(&mut nonce);

        Self { nonce }
    }

    pub fn id(&self) -> TokenId {
        let mut state = blake2b_simd::Params::default()
            .personal(b"penumbra_tokenfactory_id")
            .to_state();

        state.update(&self.nonce);

        let hash = state.finalize();
        let mut bytes = [0; 32];
        bytes[0..32].copy_from_slice(&hash.as_bytes()[0..32]);
        TokenId(bytes)
    }
}

/// A hash of a [`Position`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize)]
#[serde(try_from = "pb::TokenId", into = "pb::TokenId")]
pub struct TokenId(pub [u8; 32]);

impl std::fmt::Debug for TokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bech32str::encode(
            &self.0,
            bech32str::tokenfactory::BECH32_PREFIX,
            bech32str::Bech32m,
        ))
    }
}

impl std::fmt::Display for TokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&bech32str::encode(
            &self.0,
            bech32str::tokenfactory::BECH32_PREFIX,
            bech32str::Bech32m,
        ))
    }
}

impl std::str::FromStr for TokenId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = bech32str::decode(
            s,
            bech32str::tokenfactory::BECH32_PREFIX,
            bech32str::Bech32m,
        )?;
        pb::TokenId {
            inner,
            alt_bech32m: String::new(),
        }
        .try_into()
    }
}
