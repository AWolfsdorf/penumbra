pub mod action_handler;
pub mod component;
pub mod mint;
pub mod note;
pub mod note_finder;
pub mod state_key;
pub mod note_manager;

pub use action_handler::*;
pub use component::*;
pub use mint::*;
pub use note::*;
pub use note_finder::*;
pub use note_manager::*;

use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_asset::asset::{self, Metadata};
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};
use tracing::instrument;

use crate::state_key;

#[async_trait]
pub trait TokenFactoryRead: StateRead {
    async fn get_denom_creator(&self, denom: &str) -> Option<String> {
        self.get(&state_key::denom_creator::by_denom(denom))
            .await
            .expect("no deserialization error")
    }

    async fn get_denom_admin(&self, denom: &str) -> Option<String> {
        self.get(&state_key::denom_admin::by_denom(denom))
            .await
            .expect("no deserialization error")
    }
}

impl<T: StateRead + ?Sized> TokenFactoryRead for T {}

#[async_trait]
pub trait TokenFactory: StateWrite {
    /// Create a new denom
    #[instrument(skip(self))]
    async fn create_denom(&mut self, creator: String, subdenom: String) -> anyhow::Result<()> {
        let denom = format!("factory/{}/{}", creator, subdenom);
        
        // Check if denom already exists
        if self.get_denom_creator(&denom).await.is_some() {
            return Err(anyhow::anyhow!("denom already exists"));
        }

        // Create the denom metadata
        let metadata = Metadata::new(denom.clone())?;

        // Register the denom in the asset registry
        self.register_denom(&metadata).await;

        // Store creator and admin
        self.put(state_key::denom_creator::by_denom(&denom), creator.clone());
        self.put(state_key::denom_admin::by_denom(&denom), creator);

        Ok(())
    }

    /// Mint tokens
    #[instrument(skip(self))]
    async fn mint_tokens(&mut self, admin: String, denom: String, amount: u128) -> anyhow::Result<()> {
        // Verify admin
        let current_admin = self.get_denom_admin(&denom).await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;
        
        if current_admin != admin {
            return Err(anyhow::anyhow!("unauthorized"));
        }

        // Get asset metadata
        let metadata = self.denom_metadata_by_asset(&asset::Id::from(denom.clone()))
            .await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;

        // Create value and mint
        let value = asset::Value {
            amount: amount.into(),
            asset_id: metadata.id(),
        };

        // TODO: Implement actual minting logic
        // This would need to integrate with the shielded pool's note system
        // For now we just log
        tracing::info!(?value, "minting tokens");

        Ok(())
    }

    /// Burn tokens
    #[instrument(skip(self))]
    async fn burn_tokens(&mut self, admin: String, denom: String, amount: u128) -> anyhow::Result<()> {
        // Verify admin
        let current_admin = self.get_denom_admin(&denom).await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;
        
        if current_admin != admin {
            return Err(anyhow::anyhow!("unauthorized"));
        }

        // Get asset metadata
        let metadata = self.denom_metadata_by_asset(&asset::Id::from(denom.clone()))
            .await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;

        // Create value and burn
        let value = asset::Value {
            amount: amount.into(),
            asset_id: metadata.id(),
        };

        // TODO: Implement actual burning logic
        // This would need to integrate with the shielded pool's note system
        // For now we just log
        tracing::info!(?value, "burning tokens");

        Ok(())
    }

    /// Change admin
    #[instrument(skip(self))]
    async fn change_admin(&mut self, current_admin: String, denom: String, new_admin: String) -> anyhow::Result<()> {
        // Verify current admin
        let stored_admin = self.get_denom_admin(&denom).await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;
        
        if stored_admin != current_admin {
            return Err(anyhow::anyhow!("unauthorized"));
        }

        // Update admin
        self.put(state_key::denom_admin::by_denom(&denom), new_admin);

        Ok(())
    }
}

impl<T: StateWrite + ?Sized> TokenFactory for T {} 