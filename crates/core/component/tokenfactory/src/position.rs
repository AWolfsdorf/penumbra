use anyhow::Context;
use penumbra_sdk_asset::asset::{self, Metadata};
use penumbra_sdk_num::Amount;
use rand_core::CryptoRngCore;
use penumbra_sdk_proto::{
    core::component::tokenfactory::v1alpha as pb, serializers::bech32str, DomainType
};

use serde::{Deserialize, Serialize};

use crate::state_key;

pub struct TokenFactoryPosition {
    pub token_id: TokenId,
    pub nonce: [u8; 32],
    pub metadata: Metadata,
    pub initial_supply: Amount,
}

impl TokenFactoryPosition {
    pub fn new<R: CryptoRngCore>(mut rng: R, initial_supply: Amount) -> Self {
        let mut nonce = [0; 32];
        rng.fill_bytes(&mut nonce);

        let mut state = blake2b_simd::Params::default()
            .personal(b"penumbra_tokenfactory_id")
            .to_state();

        state.update(&nonce);
        state.update(&initial_supply.to_be_bytes());

        let hash = state.finalize();
        let mut bytes = [0; 32];
        bytes[0..32].copy_from_slice(&hash.as_bytes()[0..32]);
        let token_id = TokenId(bytes);

        let metadata = asset::REGISTRY
            .parse_denom(&state_key::token_factory::by_id(&token_id.into()))
            .expect("base denom format is valid");

        Self { token_id, nonce, metadata, initial_supply }
    }

    pub fn denom(&self) -> asset::Id {
        self.metadata.id()
    }
}

/// A hash of a [`Position`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize)]
#[serde(try_from = "pb::TokenId", into = "pb::TokenId")]
pub struct TokenId(pub [u8; 32]);

impl TokenId {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let inner = bech32str::decode(
            s,
            bech32str::tokenfactory::BECH32_PREFIX,
            bech32str::Bech32m,
        )?;
        Ok(TokenId(inner.try_into().unwrap()))
    }
}


impl From<asset::Id> for TokenId {
    fn from(asset_id: asset::Id) -> Self {
        let mut bytes = [0; 32];
        bytes[0..32].copy_from_slice(&asset_id.to_bytes());
        TokenId(bytes)
    }
}

impl From<TokenId> for asset::Id {
    fn from(token_id: TokenId) -> Self {
        asset::Id::try_from(token_id.0).unwrap()
    }
}

/* Protobuf impl */

impl DomainType for TokenId {
    type Proto = pb::TokenId;
}

impl From<TokenId> for pb::TokenId {
    fn from(domain: TokenId) -> Self {
        Self {
            inner: domain.0.to_vec(),
            // Never produce a proto encoding with the alt field set.
            alt_bech32m: String::new(),
        }
    }
}

impl TryFrom<pb::TokenId> for TokenId {
    type Error = anyhow::Error;

    fn try_from(msg: pb::TokenId) -> Result<Self, Self::Error> {
        match (msg.inner.is_empty(), msg.alt_bech32m.is_empty()) {
            (false, true) => Ok(TokenId(msg
                .inner
                .as_slice()
                .try_into()
                .context("expected 32-byte id")?)),
            (true, false) => msg.alt_bech32m.parse(),
            (false, false) => Err(anyhow::anyhow!(
                "AssetId proto has both inner and alt_bech32m fields set"
            )),
            (true, true) => Err(anyhow::anyhow!(
                "AssetId proto has neither inner nor alt_bech32m fields set"
            )),
        }
    }
}
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
