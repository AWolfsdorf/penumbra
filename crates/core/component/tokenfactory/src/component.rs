use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_asset::asset;
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};

use crate::{TokenFactory, TokenFactoryRead};

#[async_trait]
pub trait TokenFactoryComponent: StateRead + StateWrite {
    async fn create_denom(&mut self, creator: String, subdenom: String) -> anyhow::Result<()> {
        <Self as TokenFactory>::create_denom(self, creator, subdenom).await
    }

    async fn mint_tokens(&mut self, admin: String, denom: String, amount: u128) -> anyhow::Result<()> {
        <Self as TokenFactory>::mint_tokens(self, admin, denom, amount).await
    }

    async fn burn_tokens(&mut self, admin: String, denom: String, amount: u128) -> anyhow::Result<()> {
        <Self as TokenFactory>::burn_tokens(self, admin, denom, amount).await
    }

    async fn change_admin(&mut self, current_admin: String, denom: String, new_admin: String) -> anyhow::Result<()> {
        <Self as TokenFactory>::change_admin(self, current_admin, denom, new_admin).await
    }
}

impl<T: StateRead + StateWrite + ?Sized> TokenFactoryComponent for T {} 