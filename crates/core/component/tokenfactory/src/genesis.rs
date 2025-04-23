use serde::{Deserialize, Serialize};

use crate::params::TokenFactoryParams;

use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1 as pb, DomainType};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(try_from = "pb::GenesisContent", into = "pb::GenesisContent")]
pub struct Content {
    pub token_factory_params: TokenFactoryParams,
}


impl From<Content> for pb::GenesisContent {
    fn from(value: Content) -> Self {
        pb::GenesisContent {
            token_factory_params: Some(value.token_factory_params.into()),
        }
    }
}

impl TryFrom<pb::GenesisContent> for Content {
    type Error = anyhow::Error;

    fn try_from(msg: pb::GenesisContent) -> Result<Self, Self::Error> {
        Ok(Content {
            token_factory_params: msg
                .token_factory_params
                .context("token factory params not present in protobuf message")?
                .try_into()?,
        })
    }
}