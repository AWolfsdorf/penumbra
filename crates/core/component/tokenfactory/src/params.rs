use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1alpha as pb, DomainType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "pb::TokenFactoryParameters",
    into = "pb::TokenFactoryParameters"
)]
pub struct TokenFactoryParams {
    pub is_enabled: bool,
}

impl DomainType for TokenFactoryParams {
    type Proto = pb::TokenFactoryParameters;
}

impl From<TokenFactoryParams> for pb::TokenFactoryParameters {
    fn from(_: TokenFactoryParams) -> Self {
        pb::TokenFactoryParameters { is_enabled: true }
    }
}

impl TryFrom<pb::TokenFactoryParameters> for TokenFactoryParams {
    type Error = anyhow::Error;

    fn try_from(_: pb::TokenFactoryParameters) -> anyhow::Result<Self> {
        Ok(TokenFactoryParams { is_enabled: true })
    }
}

impl Default for TokenFactoryParams {
    fn default() -> Self {
        TokenFactoryParams { is_enabled: true }
    }
}
