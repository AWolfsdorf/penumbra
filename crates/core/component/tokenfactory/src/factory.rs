use async_trait::async_trait;
use cnidarium::StateRead;
use penumbra_sdk_asset::{asset::{self, Metadata}, Value};
use penumbra_sdk_num::Amount;
use penumbra_sdk_keys::Address;
use penumbra_sdk_shielded_pool::Note;
use penumbra_sdk_proto::core::asset::v1 as pb;
use rand::{RngCore, CryptoRng, rngs::StdRng, SeedableRng};

use crate::{state_key, TokenFactoryNft, TokenFactoryNoteManager};

/// Trait for reading token factory state
#[async_trait]
pub trait TokenFactoryRead: StateRead {
    /// Gets the creator of a denomination
    async fn get_denom_creator(&self, denom: &str) -> anyhow::Result<Option<String>> {
        self.get_raw(&state_key::denom_creator::by_denom(denom))
            .await
            .map(|v| v.map(|bytes| String::from_utf8(bytes).expect("valid UTF-8")))
            .map_err(Into::into)
    }

    /// Gets the admin of a denomination
    async fn get_denom_admin(&self, denom: &str) -> anyhow::Result<Option<String>> {
        self.get_raw(&state_key::denom_admin::by_denom(denom))
            .await
            .map(|v| v.map(|bytes| String::from_utf8(bytes).expect("valid UTF-8")))
            .map_err(Into::into)
    }
}

impl<T: StateRead + ?Sized> TokenFactoryRead for T {}

fn generate_random_bytes() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut bytes);
    bytes
}

fn create_note<R: RngCore + CryptoRng>(rng: &mut R, address: &Address, value: Value) -> Note {
    Note::generate(rng, address, value)
}

fn create_thread_safe_rng() -> impl RngCore + CryptoRng + Send {
    StdRng::from_seed(generate_random_bytes())
}

#[async_trait]
pub trait TokenFactory: TokenFactoryNoteManager {
    /// Create a new token with metadata and initial supply
    async fn create_token(
        &mut self,
        metadata: Metadata,
        initial_supply: Amount,
    ) -> anyhow::Result<(String, TokenFactoryNft)> {
        let token_id = hex::encode(generate_random_bytes());
        let denom = format!("factory/{}", token_id);
        
        // Check if denom already exists
        if self.get_denom_creator(&denom).await?.is_some() {
            return Err(anyhow::anyhow!("denom already exists"));
        }

        // Create initial supply
        if initial_supply > Amount::zero() {
            let value = Value {
                amount: initial_supply.into(),
                asset_id: asset::REGISTRY
                    .parse_denom(&denom)
                    .expect("base denom format is valid")
                    .id(),
            };
            // TODO: Implement minting through shielded pool integration
            tracing::info!(?value, "creating initial supply");
        }

        // Create mint authority NFT
        let nft = TokenFactoryNft::new(token_id, 0);

        Ok((denom, nft))
    }
    
    /// Mint additional tokens using the NFT-based authority
    async fn mint_token(
        &mut self,
        nft: &TokenFactoryNft,
        amount: Amount,
    ) -> anyhow::Result<TokenFactoryNft> {
        // Verify NFT ownership
        if !self.owns_nft(nft).await? {
            return Err(anyhow::anyhow!("unauthorized: NFT not owned"));
        }
        
        // Create new tokens
        let denom = format!("factory/{}", nft.token_id());
        let base_denom = asset::REGISTRY
            .parse_denom(&denom)
            .expect("base denom format is valid");
        
        let value = Value {
            amount,
            asset_id: base_denom.id(),
        };
        
        let mut rng = create_thread_safe_rng();
        let address = self.owner_address().await?;
        let note = create_note(&mut rng, &address, value);
        
        // Add the note to the shielded pool
        self.add_note(note).await?;
        
        // Return the next sequence NFT
        Ok(nft.next_sequence())
    }
    
    /// Burn minting authority NFT
    async fn burn_mint_authority(
        &mut self,
        nft: &TokenFactoryNft,
    ) -> anyhow::Result<()> {
        // Verify NFT ownership
        if !self.owns_nft(nft).await? {
            return Err(anyhow::anyhow!("unauthorized: NFT not owned"));
        }
        
        // Create a Note representing the NFT to burn it
        let nft_value = Value {
            amount: Amount::from(1u32),
            asset_id: nft.asset_id(),
        };
        
        let mut rng = create_thread_safe_rng();
        let address = self.owner_address().await?;
        let nft_note = create_note(&mut rng, &address, nft_value);
        
        // Remove the NFT note from the shielded pool
        self.remove_note(nft_note).await?;
        
        Ok(())
    }
    
    /// Check if the current user owns the given NFT
    async fn owns_nft(&self, _nft: &TokenFactoryNft) -> anyhow::Result<bool> {
        // TODO: Implement NFT ownership verification
        // This would need to check if the current address owns the NFT
        Ok(true)
    }
    
    /// Get the address of the current user
    async fn owner_address(&self) -> anyhow::Result<Address> {
        // TODO: Implement getting the current address
        // This would need to return the address of the current user
        Err(anyhow::anyhow!("not implemented: owner_address"))
    }
}

impl<T: TokenFactoryNoteManager + ?Sized> TokenFactory for T {}