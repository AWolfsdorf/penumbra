use anyhow::{ensure, Result};
use async_trait::async_trait;
use cnidarium::StateWrite;
use cnidarium_component::ActionHandler;

use crate::{StateReadExt, StateWriteExt, TokenBurn};

#[async_trait]
impl ActionHandler for TokenBurn {
    type CheckStatelessContext = ();

    async fn check_stateless(&self, _context: Self::CheckStatelessContext) -> Result<()> {
        Ok(())
    }

    async fn check_and_execute<S: StateWrite>(&self, mut state: S) -> Result<()> {
        let token_factory_params = state.get_token_factory_params().await?;

        ensure!(
            token_factory_params.is_enabled,
            "Token factory MUST be enabled to burn tokens."
        );

        state.burn_token(self.token_id.clone(), self.amount).await?;
        Ok(())
    }
}
