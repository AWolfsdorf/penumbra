use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1 as pb, DomainType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "pb::TokenFactoryParams", into = "pb::TokenFactoryParams")]
pub struct TokenFactoryParams {
    pub is_enabled: bool,
}

impl DomainType for TokenFactoryParams {
    type Proto = pb::TokenFactoryParams;
}


impl From<TokenFactoryParams> for pb::TokenFactoryParams {
    fn from(_: TokenFactoryParams) -> Self {
        pb::TokenFactoryParams { is_enabled: true }
    }
}

impl TryFrom<pb::TokenFactoryParams> for TokenFactoryParams {
    type Error = anyhow::Error;

    fn try_from(_: pb::TokenFactoryParams) -> anyhow::Result<Self> {
        Ok(TokenFactoryParams { is_enabled: true })
    }
}

impl Default for TokenFactoryParams {
    fn default() -> Self {
        TokenFactoryParams { is_enabled: true }
    }
}
