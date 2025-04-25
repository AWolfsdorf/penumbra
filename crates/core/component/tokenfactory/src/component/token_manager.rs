use async_trait::async_trait;
use cnidarium::StateWrite;
use penumbra_sdk_num::Amount;
use tracing::instrument;
use anyhow::Result;

use crate::TokenId;

/// Manages the creation of new token factories and.
#[async_trait]
pub trait TokenManager: StateWrite {
    #[instrument(name = "token_manager", skip_all)]
    async fn burn_token(&mut self, token_id: TokenId, amount: Amount) -> Result<()> {
        // TODO: Implement
        Ok(())
    }
}

impl<T: StateWrite + ?Sized> TokenManager for T {}
