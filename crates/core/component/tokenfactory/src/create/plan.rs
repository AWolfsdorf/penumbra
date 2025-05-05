use penumbra_sdk_asset::{asset::Metadata, Balance, Value};
use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1alpha as pb, DomainType};
use serde::{Deserialize, Serialize};


use super::TokenCreate;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "pb::TokenCreatePlan", into = "pb::TokenCreatePlan")]
pub struct TokenCreatePlan {
    pub metadata: Metadata,
    pub nonce: [u8; 32],
    pub initial_supply: Amount,
}

impl TokenCreatePlan {
    pub fn to_action(&self) -> TokenCreate {
        TokenCreate {
            metadata: self.metadata.clone(),
            nonce: self.nonce,
            initial_supply: self.initial_supply,
        }
    }

    pub fn balance(&self) -> Balance {
        // The create action produces the initial supply and the minting rights NFT
        let token_id = self.to_action().token_id();
        let nft = crate::TokenFactoryNft::new(token_id, 0);
        
        Balance::from(Value {
            asset_id: self.metadata.id(),
            amount: self.initial_supply,
        }) + Balance::from(Value {
            asset_id: nft.asset_id(),
            amount: Amount::from(1u128),
        })
    }
}

impl DomainType for TokenCreatePlan {
    type Proto = pb::TokenCreatePlan;
}

impl From<TokenCreatePlan> for pb::TokenCreatePlan {
    fn from(domain: TokenCreatePlan) -> Self {
        Self {
            metadata: Some(domain.metadata.into()),
            nonce: domain.nonce.to_vec(),
            initial_supply: Some(domain.initial_supply.into()),
        }
    }
}

impl TryFrom<pb::TokenCreatePlan> for TokenCreatePlan {
    type Error = anyhow::Error;
    fn try_from(msg: pb::TokenCreatePlan) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: msg
                .metadata
                .ok_or_else(|| anyhow::anyhow!("TokenCreatePlan message is missing metadata"))?
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