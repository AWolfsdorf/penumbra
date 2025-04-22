use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_sdk_shielded_pool::Note;
use penumbra_sdk_num::Amount;

use crate::state_key;

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

/// Trait for token factory note management
#[async_trait]
pub trait TokenFactoryNoteManager: TokenFactoryRead + StateWrite {
    /// Add a note to the shielded pool
    async fn add_note(&mut self, note: Note) -> anyhow::Result<()> {
        // TODO: Implement actual note adding logic
        // This would need to integrate with the shielded pool's note system
        // For now we just log
        tracing::info!(?note, "adding note");
        Ok(())
    }

    /// Remove a note from the shielded pool
    async fn remove_note(&mut self, note: Note) -> anyhow::Result<()> {
        // TODO: Implement actual note removing logic
        // This would need to integrate with the shielded pool's note system
        // For now we just log
        tracing::info!(?note, "removing note");
        Ok(())
    }

    /// Split a note into two notes
    async fn split_note(&mut self, note: Note, amount: Amount) -> anyhow::Result<(Note, Note)> {
        // TODO: Implement actual note splitting logic
        // This would need to integrate with the shielded pool's note system
        // For now we just log
        tracing::info!(?note, ?amount, "splitting note");
        
        // Create two placeholder notes with same values
        let note1 = note.clone();
        let note2 = note;
        
        Ok((note1, note2))
    }
}

impl<T: StateRead + StateWrite + ?Sized> TokenFactoryNoteManager for T {} 