use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_asset::{Value};
use penumbra_sdk_num::Amount;

use crate::{
    TokenFactory, 
    TokenFactoryNft, 
    factory::TokenMetadata, 
    bonding_curve::{BondingCurveFactory, CurveParams}
};

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryCreate {
    pub metadata: TokenMetadata,
    pub initial_supply: Amount,
}

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryCreateWithBondingCurve {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub initial_supply: Amount,
    pub curve_params: CurveParams,
}

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryMint {
    pub nft: TokenFactoryNft,
    pub amount: Amount,
}

#[derive(Debug, Clone)]
pub struct ActionBurn {
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct ActionBurnMintAuthority {
    pub nft: TokenFactoryNft,
}

#[async_trait]
pub trait TokenFactoryActionHandler: StateRead + StateWrite + TokenFactory + BondingCurveFactory {
    async fn handle_create(
        &mut self,
        action: ActionTokenFactoryCreate,
    ) -> anyhow::Result<(String, TokenFactoryNft)> {
        <Self as TokenFactory>::create_token(
            self,
            action.metadata,
            action.initial_supply,
        )
        .await
    }

    async fn handle_create_with_bonding_curve(
        &mut self,
        action: ActionTokenFactoryCreateWithBondingCurve,
    ) -> anyhow::Result<(String, TokenFactoryNft)> {
        <Self as BondingCurveFactory>::create_with_bonding_curve(
            self,
            action.name,
            action.symbol,
            action.description,
            action.initial_supply,
            action.curve_params,
        )
        .await
    }

    async fn handle_mint(
        &mut self,
        action: ActionTokenFactoryMint,
    ) -> anyhow::Result<TokenFactoryNft> {
        <Self as TokenFactory>::mint_token(self, &action.nft, action.amount).await
    }

    async fn handle_burn(
        &mut self,
        action: ActionBurn,
    ) -> anyhow::Result<()> {
        // This is a simplified burn that doesn't use admin/denom
        tracing::info!(?action.value, "burning tokens");
        Ok(())
    }

    async fn handle_burn_mint_authority(
        &mut self,
        action: ActionBurnMintAuthority,
    ) -> anyhow::Result<()> {
        <Self as TokenFactory>::burn_mint_authority(self, &action.nft).await
    }
}

impl<T: StateRead + StateWrite + TokenFactory + BondingCurveFactory + ?Sized> TokenFactoryActionHandler for T {} 