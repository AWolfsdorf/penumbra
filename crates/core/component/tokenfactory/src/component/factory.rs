use anyhow::Result;
use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use cnidarium_component::Component;
use penumbra_sdk_proto::{StateReadProto, StateWriteProto};
use std::sync::Arc;
use tendermint::v0_37::abci;

use crate::{genesis, params::TokenFactoryParams, state_key};

pub struct TokenFactory {}

#[async_trait]
impl Component for TokenFactory {
    type AppState = genesis::Content;

    async fn init_chain<S: StateWrite>(mut state: S, app_state: Option<&Self::AppState>) {
        match app_state {
            None => { /* no-op */ }
            Some(app_state) => {
                state.put_token_factory_params(app_state.token_factory_params.clone());
            }
        }
    }

    async fn begin_block<S: StateWrite + 'static>(
        _state: &mut Arc<S>,
        _begin_block: &abci::request::BeginBlock,
    ) {
        // TODO: handle begin block events
    }

    async fn end_block<S: StateWrite + 'static>(
        _state: &mut Arc<S>,
        _end_block: &abci::request::EndBlock,
    ) {
        // TODO: handle end block events
    }
}

#[async_trait]
pub trait StateWriteExt: StateWrite {
    fn put_token_factory_params(&mut self, params: TokenFactoryParams) {
        self.object_put(state_key::parameters::updated_flag(), ());
        self.put(state_key::parameters::key().into(), params);
    }
}

impl<T: StateWrite + ?Sized> StateWriteExt for T {}

#[async_trait]
pub trait StateReadExt: StateRead {
    async fn get_token_factory_params(&self) -> Result<TokenFactoryParams> {
        self.get(state_key::parameters::key().into())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Missing TokenFactoryParams"))
    }
}

impl<T: StateRead + ?Sized> StateReadExt for T {}
