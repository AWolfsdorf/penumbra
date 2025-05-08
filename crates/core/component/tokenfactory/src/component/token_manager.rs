use anyhow::{bail, ensure, Result};
use async_trait::async_trait;
use cnidarium::{StateWrite, StateRead};
use penumbra_sdk_asset::asset::{self as asset};
use penumbra_sdk_num::Amount;
use penumbra_sdk_proto::StateReadProto;
use pbjson_types::Any;
use tracing::instrument;

use crate::{state_key, TokenFactoryPosition, TokenId};

/// Manages the creation of new token factories and.
#[async_trait]
pub trait TokenManager: StateWrite {
    #[instrument(name = "token_manager", skip_all)]
    async fn burn_token(&mut self, _token_id: TokenId, _amount: Amount) -> Result<()> {
        // TODO: Implement if needed
        Ok(())
    }

    #[instrument(name = "token_manager", skip_all)]
    async fn create_token(
        &mut self,
        token_position: &TokenFactoryPosition,
    ) -> Result<()> {
        let token_id = token_position.token_id;

        // Check that the `token_id` is unused.
        ensure!(
            !self.token_id_exists(token_id).await,
            "the supplied token id is already known to the chain (id={token_id})"
        );

        let token_id: asset::Id = token_id.try_into()?;

        // Add the initial supply to the shielded pool
        let _ = self.set_factory_supply(&token_id, token_position.initial_supply).await;

        // Add the minting rights NFT to the shielded pool
        let _ = self.increment_minting_rights(0, &token_id).await;

        Ok(())
    }
}

impl<T: StateWrite + ?Sized> TokenManager for T {}

#[async_trait]
pub trait TokenFactoryData: StateRead {
    /// Returns the number of minting rights for a given asset.
    /// If there were no counter initialized for a given asset, this default to zero.
    async fn get_factory_minting_rights(&self, seq: u64, asset_id: &asset::Id) -> u32 {
        let path = state_key::token_factory_nft::nft_minting_rights(seq, asset_id);
        self.get_factory_minting_rights_from_key(path.as_bytes()).await
    }

    async fn get_factory_minting_rights_from_key(&self, path: &[u8]) -> u32 {
        let Some(raw_count) = self
            .nonverifiable_get_raw(&path)
            .await
            .expect("no deserialization failure")
        else {
            return 0;
        };

        // This is safe because we only increment the counter via [`Self::increment_factory_mint_count`].
        let raw_count: [u8; 4] = raw_count
            .try_into()
            .expect("position counter is at most two bytes");
        u32::from_be_bytes(raw_count)
    }

    async fn get_factory_mint_seq_from_key(&self, key: &[u8]) -> u32 {
        let Some(raw_seq) = self.nonverifiable_get_raw(key).await.expect("no deserialization failure") else {
            return 0;
        };
        u32::from_be_bytes(raw_seq.try_into().expect("sequence is at most 4 bytes"))
    }

    async fn get_factory_supply(&self, asset_id: &asset::Id) -> u128 {
        let path = state_key::token_factory::by_id(asset_id);
        self.get_factory_supply_from_key(path.as_bytes()).await
    }

    async fn get_factory_supply_from_key(&self, key: &[u8]) -> u128 {
        let Some(raw_supply) = self.nonverifiable_get_raw(key).await.expect("no deserialization failure") else {
            return 0;
        };
        u128::from_be_bytes(raw_supply.try_into().expect("supply is at most 16 bytes"))
    }

    /// Returns whether the supplied `token_id` exists in the chain state.
    async fn token_id_exists(&self, token_id: TokenId) -> bool {
        self.get_raw_token_factory(token_id).await.is_some()
    }

    /// Returns raw token factory data if found under the specified `token_id`,
    /// and `None` otherwise
    async fn get_raw_token_factory(&self, token_id: TokenId) -> Option<Any> {
        let asset_id = token_id.try_into().ok()?;
        self.get_proto(&state_key::token_factory::by_id(&asset_id))
            .await
            .expect("no storage errors")
    }
}

impl<T: StateRead + ?Sized> TokenFactoryData for T {}

trait Inner: StateWrite {
    async fn set_factory_supply(&mut self, asset_id: &asset::Id, supply: Amount) -> Result<()> {
        let key = state_key::token_factory::by_id(asset_id);
        self.put_raw(key, supply.to_be_bytes().to_vec());
        Ok(())
    }

    async fn increment_minting_rights(&mut self, seq: u64, asset_id: &asset::Id) -> Result<()> {
        let key = state_key::token_factory_nft::nft_minting_rights(seq, asset_id);
        let prev: u32 = self.get_factory_minting_rights(seq, asset_id).await;

        let Some(new_total) = prev.checked_add(1) else {
            bail!("incrementing factory mint count would overflow")
        };

        self.put_raw(key, new_total.to_be_bytes().to_vec());
        Ok(())
    }

    async fn decrement_minting_rights(&mut self, seq: u64, asset_id: &asset::Id) -> Result<()> {
        let key = state_key::token_factory_nft::nft_minting_rights(seq, asset_id);
        let prev: u32 = self.get_factory_minting_rights(seq, asset_id).await;

        let Some(new_total) = prev.checked_sub(1) else {
            bail!("decrementing factory mint count would underflow")
        };

        self.put_raw(key, new_total.to_be_bytes().to_vec());
        Ok(())
    }
}

impl<T: StateWrite + ?Sized> Inner for T {}