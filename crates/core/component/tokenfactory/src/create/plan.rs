use penumbra_sdk_asset::{Balance, Value};
use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1alpha as pb, DomainType};
use serde::{Deserialize, Serialize};

use super::TokenCreate;

use crate::TokenId;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "pb::TokenCreatePlan", into = "pb::TokenCreatePlan")]
pub struct TokenCreatePlan {
    pub token_id: TokenId,
    pub nonce: [u8; 32],
    pub initial_supply: Amount,
}

impl TokenCreatePlan {
    pub fn to_action(&self) -> TokenCreate {
        TokenCreate {
            token_id: self.token_id,
            nonce: self.nonce,
            initial_supply: self.initial_supply,
        }
    }

    pub fn balance(&self) -> Balance {

        // The create action produces the initial supply and the minting rights NFT
        let position = self.to_action().position();
        let nft = crate::TokenFactoryNft::new(position.token_id, 0);
        
        Balance::from(Value {
            asset_id: position.denom(),
            amount: self.initial_supply,
        }) + Balance::from(Value {
            asset_id: nft.denom().into(),
            amount: 1u128.into(),
        })
    }
}

impl DomainType for TokenCreatePlan {
    type Proto = pb::TokenCreatePlan;
}

impl From<TokenCreatePlan> for pb::TokenCreatePlan {
    fn from(domain: TokenCreatePlan) -> Self {
        Self {
            token_id: Some(domain.token_id.into()),
            nonce: domain.nonce.to_vec(),
            initial_supply: Some(domain.initial_supply.into()),
        }
    }
}

impl TryFrom<pb::TokenCreatePlan> for TokenCreatePlan {
    type Error = anyhow::Error;
    fn try_from(msg: pb::TokenCreatePlan) -> Result<Self, Self::Error> {
        Ok(Self {
            token_id: msg.token_id
                .ok_or_else(|| anyhow::anyhow!("missing token id"))?
                .try_into()?,
            nonce: msg
                .nonce
                .try_into()
                .map_err(|e| anyhow::anyhow!("nonce must be 32 bytes: {:#?}", e))?,
            initial_supply: msg
                .initial_supply
                .ok_or_else(|| anyhow::anyhow!("TokenCreatePlan message is missing initial supply"))?
                .try_into()?,
        })
    }
} 