use serde::{Deserialize, Serialize};

use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{core::component::tokenfactory::v1alpha as pb, DomainType};
use penumbra_sdk_txhash::{EffectHash, EffectingData};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "pb::TokenBurn", into = "pb::TokenBurn")]
pub struct TokenBurn {
    pub token_id: Id,
    pub seq: u64,
    pub amount: Amount,
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
            token_id: value
                .token_id
                .inner
                .ok_or_else(|| anyhow::anyhow!("missing token id"))?
                .try_into()
                .context("token id malformed")?,
            seq: value.seq,
            amount: value
                .amount
                .ok_or_else(|| anyhow::anyhow!("missing amount"))?
                .try_into()
                .context("amount malformed")?,
        })
    }
}
