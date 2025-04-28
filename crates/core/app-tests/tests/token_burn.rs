mod common;

use self::common::TempStorageExt;
use cnidarium::{ArcStateDeltaExt, StateDelta, TempStorage};
use cnidarium_component::ActionHandler;
use penumbra_sdk_asset::asset;
use penumbra_sdk_num::Amount;
use penumbra_sdk_tokenfactory::{
    burn::TokenBurnPlan,
    TokenId,
};
use rand_core::SeedableRng;
use std::sync::Arc;

#[tokio::test]
async fn token_burn() -> anyhow::Result<()> {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);

    let storage = TempStorage::new_with_penumbra_prefixes()
        .await?
        .apply_default_genesis()
        .await?;
    let mut state = Arc::new(StateDelta::new(storage.latest_snapshot()));

    // Get a test token to burn
    let gm = asset::Cache::with_known_assets().get_unit("gm").unwrap();
    let token_id: TokenId = gm.id().into();
    let amount = Amount::from(100_000u64);

    // Create a token burn plan
    let burn_plan = TokenBurnPlan {
        asset_id: gm.id(),
        amount,
    };

    // Convert the plan to an action
    let burn = burn_plan.to_action();

    // Execute the burn action
    burn.check_stateless(()).await?;
    burn.check_historical(state.clone()).await?;
    let mut state_tx = state.try_begin_transaction().unwrap();
    // state_tx.put_mock_source(1u8);
    burn.check_and_execute(&mut state_tx).await?;
    state_tx.apply();

    Ok(())
}

#[tokio::test]
async fn token_burn_with_zero_amount() -> anyhow::Result<()> {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);

    let storage = TempStorage::new_with_penumbra_prefixes()
        .await?
        .apply_default_genesis()
        .await?;
    let mut state = Arc::new(StateDelta::new(storage.latest_snapshot()));

    // Get a test token to burn
    let gm = asset::Cache::with_known_assets().get_unit("gm").unwrap();
    let token_id: TokenId = gm.id().into();
    let amount = Amount::from(0u64);

    // Create a token burn plan
    let burn_plan = TokenBurnPlan {
        asset_id: gm.id(),
        amount,
    };

    // Convert the plan to an action
    let burn = burn_plan.to_action();

    // Execute the burn action
    burn.check_stateless(()).await?;
    burn.check_historical(state.clone()).await?;
    let mut state_tx = state.try_begin_transaction().unwrap();
    // state_tx.put_mock_source(1u8);
    burn.check_and_execute(&mut state_tx).await?;
    state_tx.apply();

    Ok(())
} 