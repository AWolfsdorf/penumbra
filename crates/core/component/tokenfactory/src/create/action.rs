use serde::{Deserialize, Serialize};
use anyhow::Context;

use penumbra_sdk_asset::asset::Metadata;
use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{core::component::tokenfactory::v1alpha as pb, DomainType};
use penumbra_sdk_txhash::{EffectHash, EffectingData};

use crate::{TokenId, TokenFactoryPosition};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "pb::TokenCreate", into = "pb::TokenCreate")]
pub struct TokenCreate {
    pub metadata: Metadata,
    pub nonce: [u8; 32],
    pub initial_supply: Amount,
}

impl TokenCreate {
    pub fn token_id(&self) -> TokenId {
        let position = TokenFactoryPosition { nonce: self.nonce };
        position.id()
    }
}

impl EffectingData for TokenCreate {
    fn effect_hash(&self) -> EffectHash {
        EffectHash::from_proto_effecting_data(&self.to_proto())
    }
}

/* Protobuf impls */

impl DomainType for TokenCreate {
    type Proto = pb::TokenCreate;
}

impl From<TokenCreate> for pb::TokenCreate {
    fn from(value: TokenCreate) -> Self {
        Self {
            metadata: Some(value.metadata.into()),
            nonce: value.nonce.to_vec(),
            initial_supply: Some(value.initial_supply.into()),
        }
    }
}

impl TryFrom<pb::TokenCreate> for TokenCreate {
    type Error = anyhow::Error;
    fn try_from(value: pb::TokenCreate) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: value
                .metadata
                .ok_or_else(|| anyhow::anyhow!("missing metadata"))?
                .try_into()
                .context("metadata malformed")?,
            nonce: value
                .nonce
                .try_into()
                .map_err(|e| anyhow::anyhow!("nonce must be 32 bytes: {:#?}", e))?,
            initial_supply: value
                .initial_supply
                .ok_or_else(|| anyhow::anyhow!("missing initial supply"))?
                .try_into()
                .context("initial supply malformed")?,
        })
    }
} 