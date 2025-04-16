use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_asset::asset;
use penumbra_sdk_keys::Address;
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};

use crate::{TokenFactory, TokenFactoryRead};

#[async_trait]
pub trait TokenFactoryMint: StateRead + StateWrite {
    async fn mint_tokens_to_address(
        &mut self,
        admin: String,
        denom: String,
        amount: u128,
        recipient: Address,
    ) -> anyhow::Result<()> {
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

        // Create a new note for the recipient
        let note = penumbra_sdk_shielded_pool::Note::generate(
            &mut rand::thread_rng(),
            &recipient,
            value,
        );

        // Add the note to the shielded pool
        self.add_note(note).await?;

        Ok(())
    }

    async fn burn_tokens_from_address(
        &mut self,
        admin: String,
        denom: String,
        amount: u128,
        owner: Address,
    ) -> anyhow::Result<()> {
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

        // Find and remove notes from the owner
        let notes = self.find_notes_by_address(&owner).await?;
        let mut remaining = value.amount;
        let mut notes_to_remove = Vec::new();

        for note in notes {
            if note.asset_id() == value.asset_id {
                if note.amount() <= remaining {
                    remaining -= note.amount();
                    notes_to_remove.push(note);
                } else {
                    // Split the note if needed
                    let split_note = note.split(remaining)?;
                    notes_to_remove.push(split_note.0);
                    remaining = 0.into();
                }
            }
            if remaining == 0.into() {
                break;
            }
        }

        if remaining != 0.into() {
            return Err(anyhow::anyhow!("insufficient balance"));
        }

        // Remove the notes
        for note in notes_to_remove {
            self.remove_note(note).await?;
        }

        Ok(())
    }
}

impl<T: StateRead + StateWrite + ?Sized> TokenFactoryMint for T {} 