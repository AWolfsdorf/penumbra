use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_num::Amount;

use crate::{TokenFactory, TokenFactoryNft, TokenMetadata};

#[async_trait]
pub trait TokenFactoryComponent: StateRead + StateWrite + TokenFactory {
    async fn create_token_component(
        &mut self,
        metadata: TokenMetadata,
        initial_supply: Amount,
    ) -> anyhow::Result<(String, TokenFactoryNft)> {
        <Self as TokenFactory>::create_token(self, metadata, initial_supply).await
    }

    async fn mint_token_component(
        &mut self,
        nft: &TokenFactoryNft,
        amount: Amount,
    ) -> anyhow::Result<TokenFactoryNft> {
        <Self as TokenFactory>::mint_token(self, nft, amount).await
    }

    async fn burn_mint_authority_component(
        &mut self,
        nft: &TokenFactoryNft,
    ) -> anyhow::Result<()> {
        <Self as TokenFactory>::burn_mint_authority(self, nft).await
    }
}

impl<T: StateRead + StateWrite + TokenFactory + ?Sized> TokenFactoryComponent for T {} 