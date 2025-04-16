use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};
use penumbra_sdk_asset::{asset, Value, Metadata};

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

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryCreate {
    pub metadata: Metadata,
    pub initial_supply: u128,
    pub with_minting: bool,
}

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryCreateWithBondingCurve {
    pub metadata: Metadata,
    pub initial_supply: u128,
    pub curve_points: Vec<(asset::Id, u128)>,
}

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryMint {
    pub nft: TokenFactoryNFT,
    pub amount: u128,
}

#[derive(Debug, Clone)]
pub struct ActionBurn {
    pub value: Value,
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

    async fn handle_create(
        &mut self,
        action: ActionTokenFactoryCreate,
    ) -> anyhow::Result<(String, Option<TokenFactoryNFT>)> {
        <Self as TokenFactory>::create_token(
            self,
            action.metadata,
            action.initial_supply,
            action.with_minting,
        )
        .await
    }

    async fn handle_create_with_bonding_curve(
        &mut self,
        action: ActionTokenFactoryCreateWithBondingCurve,
    ) -> anyhow::Result<String> {
        <Self as TokenFactory>::create_token_with_bonding_curve(
            self,
            action.metadata,
            action.initial_supply,
            action.curve_points,
        )
        .await
    }

    async fn handle_mint(
        &mut self,
        action: ActionTokenFactoryMint,
    ) -> anyhow::Result<(Value, TokenFactoryNFT)> {
        <Self as TokenFactory>::mint_with_authority(self, action.nft, action.amount).await
    }

    async fn handle_burn(&mut self, action: ActionBurn) -> anyhow::Result<()> {
        <Self as TokenFactory>::burn_tokens(self, action.value).await
    }
}

impl<T: StateRead + StateWrite + ?Sized> TokenFactoryActionHandler for T {} 