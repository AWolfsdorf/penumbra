use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};

use crate::{TokenFactory, TokenFactoryRead};

#[derive(Debug, Clone)]
pub struct CreateDenom {
    pub creator: String,
    pub subdenom: String,
}

#[derive(Debug, Clone)]
pub struct MintTokens {
    pub admin: String,
    pub denom: String,
    pub amount: u128,
}

#[derive(Debug, Clone)]
pub struct BurnTokens {
    pub admin: String,
    pub denom: String,
    pub amount: u128,
}

#[derive(Debug, Clone)]
pub struct ChangeAdmin {
    pub current_admin: String,
    pub denom: String,
    pub new_admin: String,
}

#[async_trait]
pub trait TokenFactoryActionHandler: StateRead + StateWrite {
    async fn create_denom(&mut self, action: CreateDenom) -> anyhow::Result<()> {
        <Self as TokenFactory>::create_denom(self, action.creator, action.subdenom).await
    }

    async fn mint_tokens(&mut self, action: MintTokens) -> anyhow::Result<()> {
        <Self as TokenFactory>::mint_tokens(self, action.admin, action.denom, action.amount).await
    }

    async fn burn_tokens(&mut self, action: BurnTokens) -> anyhow::Result<()> {
        <Self as TokenFactory>::burn_tokens(self, action.admin, action.denom, action.amount).await
    }

    async fn change_admin(&mut self, action: ChangeAdmin) -> anyhow::Result<()> {
        <Self as TokenFactory>::change_admin(self, action.current_admin, action.denom, action.new_admin).await
    }
}

impl<T: StateRead + StateWrite + ?Sized> TokenFactoryActionHandler for T {} 