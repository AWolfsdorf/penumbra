use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_asset::{asset::REGISTRY, Value};
use penumbra_sdk_num::Amount;
use penumbra_sdk_keys::Address;

use crate::{
    TokenFactoryRead,
    note_manager::TokenFactoryNoteManager,
    note_finder::TokenFactoryNoteFinder,
};

#[async_trait]
pub trait TokenFactoryMint: StateRead + StateWrite + TokenFactoryRead + TokenFactoryNoteManager + TokenFactoryNoteFinder {
    async fn mint_tokens_to_address(
        &mut self,
        admin: String,
        denom: String,
        amount: Amount,
        recipient: Address,
    ) -> anyhow::Result<()> {
        // Verify admin
        let current_admin = self.get_denom_admin(&denom).await?
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;
        
        if current_admin != admin {
            return Err(anyhow::anyhow!("unauthorized"));
        }

        // Parse denomination to get asset ID
        let base_denom = REGISTRY
            .parse_denom(&denom)
            .ok_or_else(|| anyhow::anyhow!("invalid denomination"))?;

        // Create value and mint
        let _value = Value {
            amount,
            asset_id: base_denom.id(),
        };

        // Create a new note for the recipient
        let note = penumbra_sdk_shielded_pool::Note::generate(
            &mut rand::thread_rng(),
            &recipient,
            _value,
        );

        // Add the note to the shielded pool
        self.add_note(note).await?;

        Ok(())
    }

    async fn burn_tokens_from_address(
        &mut self,
        admin: String,
        denom: String,
        amount: Amount,
        owner: Address,
    ) -> anyhow::Result<()> {
        // Verify admin
        let current_admin = self.get_denom_admin(&denom).await?
            .ok_or_else(|| anyhow::anyhow!("denom not found"))?;
        
        if current_admin != admin {
            return Err(anyhow::anyhow!("unauthorized"));
        }

        // Parse denomination to get asset ID
        let base_denom = REGISTRY
            .parse_denom(&denom)
            .ok_or_else(|| anyhow::anyhow!("invalid denomination"))?;

        // Create value and burn
        let _value = Value {
            amount,
            asset_id: base_denom.id(),
        };

        // Find and remove notes from the owner
        let notes = self.find_notes_by_address(&owner).await?;
        let mut remaining = amount;
        let mut notes_to_remove = Vec::new();

        for note in notes {
            if note.asset_id() == base_denom.id() {
                if note.amount() <= remaining {
                    remaining = remaining - note.amount();
                    notes_to_remove.push(note);
                } else {
                    // Split the note if needed
                    let (split_note, _) = self.split_note(note, remaining).await?;
                    notes_to_remove.push(split_note);
                    remaining = Amount::zero();
                }
            }
            if remaining == Amount::zero() {
                break;
            }
        }

        if remaining != Amount::zero() {
            return Err(anyhow::anyhow!("insufficient balance"));
        }

        // Remove the notes
        for note in notes_to_remove {
            self.remove_note(note).await?;
        }

        Ok(())
    }
}

impl<T: StateRead + StateWrite + TokenFactoryRead + TokenFactoryNoteManager + TokenFactoryNoteFinder + ?Sized> TokenFactoryMint for T {} 