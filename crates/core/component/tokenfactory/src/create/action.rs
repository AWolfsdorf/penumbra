use serde::{Deserialize, Serialize};

use penumbra_sdk_proto::{core::component::tokenfactory::v1alpha as pb, DomainType};
use penumbra_sdk_txhash::{EffectHash, EffectingData};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "pb::TokenBurn", into = "pb::TokenBurn")]
pub struct TokenBurn {
    pub token_id: String,
    pub seq: u64,
    pub amount: u64,
}

impl EffectingData for TokenBurn {
    fn effect_hash(&self) -> EffectHash {
        EffectHash::from_proto_effecting_data(&self.to_proto())
    }
}

impl DomainType for TokenBurn {
    type Proto = pb::TokenBurn;
}

impl From<TokenBurn> for pb::TokenBurn {
    fn from(value: TokenBurn) -> Self {
        Self {
            token_id: value.token_id,
            seq: value.seq,
            amount: value.amount,
        }
    }
}

impl TryFrom<pb::TokenBurn> for TokenBurn {
    type Error = anyhow::Error;
    fn try_from(value: pb::TokenBurn) -> Result<Self, Self::Error> {
        Ok(Self {
            token_id: value.token_id,
            seq: value.seq,
            amount: value.amount,
        })
    }
}
