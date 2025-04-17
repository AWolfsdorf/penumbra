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
use penumbra_sdk_asset::asset::{self, Metadata, Id};
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};
use tracing::Instrument;
use rand::Rng;
use sha2::{Sha256, Digest};
use hex;

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

// NFT structure for minting rights
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenFactoryNFT {
    pub token_id: [u8; 32],
    pub sequence: u64,
}

impl TokenFactoryNFT {
    pub fn mint_authority_id(&self) -> Id {
        // Generate a deterministic asset ID for the mint authority NFT
        let mut hasher = Sha256::new();
        hasher.update(b"factory_mint");
        hasher.update(&self.token_id);
        hasher.update(&self.sequence.to_le_bytes());
        Id::from(hasher.finalize().to_vec())
    }
}

#[async_trait]
pub trait TokenFactory: StateWrite {
    /// Create a new token with optional bonding curve
    #[instrument(skip(self))]
    async fn create_token(
        &mut self,
        metadata: Metadata,
        initial_supply: u128,
        with_minting: bool,
    ) -> anyhow::Result<(String, Option<TokenFactoryNFT>)> {
        // Generate a random 32-byte nonce for the token ID
        let mut rng = rand::thread_rng();
        let mut token_id = [0u8; 32];
        rng.fill(&mut token_id);
        
        let denom = format!("factory/{}", hex::encode(token_id));
        
        // Check if denom already exists
        if self.get_denom_creator(&denom).await.is_some() {
            return Err(anyhow::anyhow!("denom already exists"));
        }

        // Register the denom in the asset registry
        self.register_denom(&metadata).await;

        // Create initial supply
        if initial_supply > 0 {
            let value = asset::Value {
                amount: initial_supply.into(),
                asset_id: metadata.id(),
            };
            // TODO: Implement minting through shielded pool integration
            tracing::info!(?value, "creating initial supply");
        }

        // Create mint authority NFT if requested
        let nft = if with_minting {
            Some(TokenFactoryNFT {
                token_id,
                sequence: 0,
            })
        } else {
            None
        };

        Ok((denom, nft))
    }

    /// Create token with automatic bonding curve setup
    #[instrument(skip(self))]
    async fn create_token_with_bonding_curve(
        &mut self,
        metadata: Metadata,
        initial_supply: u128,
        curve_points: Vec<(asset::Id, u128)>, // (asset_id, price) pairs
    ) -> anyhow::Result<String> {
        // Create token without minting rights
        let (denom, _) = self.create_token(metadata.clone(), initial_supply, false).await?;

        // Create LP positions for each point in the bonding curve
        for (asset_id, price) in curve_points {
            // TODO: Integrate with DEX to create LP position
            // Parameters: 
            // - Base asset: newly created token
            // - Quote asset: asset_id from curve_points
            // - Price: price from curve_points
            // - Fee: 0%
            tracing::info!(?asset_id, ?price, "would create LP position");
        }

        Ok(denom)
    }

    /// Mint additional tokens using mint authority NFT
    #[instrument(skip(self))]
    async fn mint_with_authority(
        &mut self,
        nft: TokenFactoryNFT,
        amount: u128,
    ) -> anyhow::Result<(asset::Value, TokenFactoryNFT)> {
        let denom = format!("factory/{}", hex::encode(nft.token_id));
        
        // Get asset metadata
        let metadata = self.denom_metadata_by_asset(&asset::Id::from(denom.clone()))
            .await
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;

        // Create new tokens
        let value = asset::Value {
            amount: amount.into(),
            asset_id: metadata.id(),
        };

        // Create next sequence NFT
        let next_nft = TokenFactoryNFT {
            token_id: nft.token_id,
            sequence: nft.sequence + 1,
        };

        // TODO: Implement actual minting through shielded pool integration
        tracing::info!(?value, "minting tokens with authority");

        Ok((value, next_nft))
    }

    /// Burn tokens explicitly
    #[instrument(skip(self))]
    async fn burn_tokens(&mut self, value: asset::Value) -> anyhow::Result<()> {
        // TODO: Implement actual burning through shielded pool integration
        tracing::info!(?value, "burning tokens");
        Ok(())
    }
}

impl<T: StateWrite + ?Sized> TokenFactory for T {} 