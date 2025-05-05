use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::{core::component::tokenfactory::v1alpha as pb, DomainType};

use crate::{TokenBurn, TokenCreate, TokenId};

#[derive(Debug, Clone)]
pub struct EventTokenBurn {
    pub token_id: TokenId,
    pub amount: Amount,
}

impl From<TokenBurn> for EventTokenBurn {
    fn from(value: TokenBurn) -> Self {
        Self::from(&value)
    }
}

impl From<&TokenBurn> for EventTokenBurn {
    fn from(value: &TokenBurn) -> Self {
        Self {
            token_id: value.token_id,
            amount: value.amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventTokenCreate {
    pub token_id: TokenId,
    pub amount: Amount,
}

impl From<TokenCreate> for EventTokenCreate {
    fn from(value: TokenCreate) -> Self {
        Self::from(&value)
    }
}

impl From<&TokenCreate> for EventTokenCreate {
    fn from(value: &TokenCreate) -> Self {
        Self {
            token_id: value.token_id(),
            amount: value.initial_supply,
        }
    }
}


/* Protobuf impls */

impl DomainType for EventTokenBurn {
    type Proto = pb::EventTokenBurn;
}

impl From<EventTokenBurn> for pb::EventTokenBurn {
    fn from(value: EventTokenBurn) -> Self {
        Self {
            token_id: Some(value.token_id.into()),
            amount: Some(value.amount.into()),
        }
    }
}

impl TryFrom<pb::EventTokenBurn> for EventTokenBurn {
    type Error = anyhow::Error;

    fn try_from(value: pb::EventTokenBurn) -> Result<Self, Self::Error> {
        Ok(Self {
            token_id: value
                .token_id
                .ok_or_else(|| anyhow::anyhow!("missing token id"))?
                .try_into()?,
            amount: value
                .amount
                .ok_or_else(|| anyhow::anyhow!("missing amount"))?
                .try_into()?,
        })
    }
}

impl DomainType for EventTokenCreate {
    type Proto = pb::EventTokenCreate;
}

impl From<EventTokenCreate> for pb::EventTokenCreate {
    fn from(value: EventTokenCreate) -> Self {
        Self {
            token_id: Some(value.token_id.into()),
            amount: Some(value.amount.into()),
        }
    }
}

impl TryFrom<pb::EventTokenCreate> for EventTokenCreate {
    type Error = anyhow::Error;

    fn try_from(value: pb::EventTokenCreate) -> Result<Self, Self::Error> {
        Ok(Self {
            token_id: value
                .token_id
                .ok_or_else(|| anyhow::anyhow!("missing token id"))?
                .try_into()?,
            amount: value
                .amount
                .ok_or_else(|| anyhow::anyhow!("missing amount"))?
                .try_into()?,
        })
    }
}
