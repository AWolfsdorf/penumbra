use crate::component::{StateReadExt, TokenManager};
use crate::create::TokenCreate;
use crate::EventTokenCreate;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use cnidarium::StateWrite;
use cnidarium_component::ActionHandler;
use penumbra_sdk_proto::{DomainType, StateWriteProto};

#[async_trait]
impl ActionHandler for TokenCreate {
    type CheckStatelessContext = ();

    async fn check_stateless(&self, _context: Self::CheckStatelessContext) -> Result<()> {
        Ok(())
    }

    async fn check_and_execute<S: StateWrite>(&self, mut state: S) -> Result<()> {
        let token_factory_params = state.get_token_factory_params().await?;

        ensure!(
            token_factory_params.is_enabled,
            "Token factory MUST be enabled to create tokens."
        );

        // Create the token and mint the initial supply
        state.create_token(
            self.metadata.clone(),
            self.nonce,
            self.initial_supply,
        ).await?;

        // Record the event
        state.record_proto(EventTokenCreate {
            token_id: self.token_id(),
            amount: self.initial_supply,
        }.to_proto());

        Ok(())
    }
} 