use penumbra_sdk_asset::{asset::Id, Balance, Value};
use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1alpha as pb, DomainType};
use serde::{Deserialize, Serialize};

use super::TokenBurn;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "pb::TokenBurnPlan", into = "pb::TokenBurnPlan")]
pub struct TokenBurnPlan {
    pub asset_id: Id,
    pub amount: Amount,
}

impl TokenBurnPlan {
    pub fn to_action(&self) -> TokenBurn {
        TokenBurn {
            token_id: self.asset_id.into(),
            amount: self.amount,
        }
    }

    pub fn balance(&self) -> Balance {
        // The burn action consumes the specified value
        Balance::from(Value {
            asset_id: self.asset_id,
            amount: self.amount,
        })
    }
}

impl DomainType for TokenBurnPlan {
    type Proto = pb::TokenBurnPlan;
}

impl From<TokenBurnPlan> for pb::TokenBurnPlan {
    fn from(domain: TokenBurnPlan) -> Self {
        Self {
            asset_id: Some(domain.asset_id.into()),
            amount: Some(domain.amount.into()),
        }
    }
}

impl TryFrom<pb::TokenBurnPlan> for TokenBurnPlan {
    type Error = anyhow::Error;
    fn try_from(msg: pb::TokenBurnPlan) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_id: msg
                .asset_id
                .ok_or_else(|| anyhow::anyhow!("TokenBurnPlan message is missing a asset id"))?
                .try_into()?,
            amount: msg
                .amount
                .ok_or_else(|| anyhow::anyhow!("TokenBurnPlan message is missing a amount"))?
                .try_into()?,
        })
    }
}
