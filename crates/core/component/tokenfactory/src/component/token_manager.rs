use async_trait::async_trait;
use cnidarium::StateWrite;
use tracing::instrument;

/// Manages the creation of new token factories and.
#[async_trait]
pub trait TokenManager: StateWrite {
    #[instrument(name = "token_manager", skip_all)]
    async fn burn_token(&mut self, token_id: String, amount: u64) -> Result<()> {
        Ok(())
    }
}

impl<T: StateWrite + ?Sized> TokenManager for T {}
