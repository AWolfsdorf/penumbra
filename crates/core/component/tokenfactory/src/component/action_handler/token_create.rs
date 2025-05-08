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

        let position = self.position();
        
        // Create the token and mint the initial supply
        state.create_token(&position).await?;

        // Record the event
        state.record_proto(EventTokenCreate {
            token_id: position.token_id,
            amount: position.initial_supply,
        }.to_proto());

        Ok(())
    }
} 