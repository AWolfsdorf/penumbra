use crate::TokenBurn;
use penumbra_sdk_proto::{penumbra::core::component::tokenfactory::v1 as pb, DomainType};

#[derive(Debug, Clone)]
pub struct EventTokenBurn {
    pub token_id: String,
    pub amount: u64,
}

impl From<TokenBurn> for EventTokenBurn {
    fn from(value: TokenBurn) -> Self {
        Self::from(&value)
    }
}

impl From<&TokenBurn> for EventTokenBurn {
    fn from(value: &TokenBurn) -> Self {
        Self {
            token_id: value.token_id.clone(),
            amount: value.amount,
        }
    }
}

impl TryFrom<pb::EventTokenBurn> for EventTokenBurn {
    type Error = anyhow::Error;

    fn try_from(value: pb::EventTokenBurn) -> Result<Self, Self::Error> {
        Ok(Self {
            token_id: value.token_id,
            amount: value.amount,
        })
    }
}

impl TryFrom<pb::EventTokenBurn> for EventTokenBurn {
    type Error = anyhow::Error;

    fn try_from(value: pb::EventTokenBurn) -> Result<Self, Self::Error> {
        fn inner(value: pb::EventTokenBurn) -> anyhow::Result<EventTokenBurn> {
            Ok(EventTokenBurn {
                token_id: value..token_id,
                amount: value.amount,
            })
        }
        inner(value).context(format!("parsing {}", pb::EventTokenBurn::NAME))
    }
}

impl DomainType for EventTokenBurn {
    type Proto = pb::EventTokenBurn;
}
