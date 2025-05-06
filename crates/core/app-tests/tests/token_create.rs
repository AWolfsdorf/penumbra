mod common;

use self::common::TempStorageExt;
use cnidarium::{ArcStateDeltaExt, StateDelta, TempStorage};
use cnidarium_component::{ActionHandler, Component};
use penumbra_sdk_asset::asset::{self, Metadata};
use penumbra_sdk_num::Amount;
use penumbra_sdk_token_factory::{create::TokenCreatePlan, component::TokenFactory};
use rand_core::SeedableRng;
use std::sync::Arc;

#[tokio::test]
async fn token_create() -> anyhow::Result<()> {
    let storage = TempStorage::new_with_penumbra_prefixes()
        .await?
        .apply_default_genesis()
        .await?;
    let mut state = StateDelta::new(storage.latest_snapshot());

    TokenFactory::init_chain(&mut state, None).await;

    let mut state = Arc::new(state);

    // Get test metadata from the asset cache
    let metadata = asset::Cache::with_known_assets()
        .get_by_id(asset::Id::from_str("wtest_usd").unwrap())
        .ok_or_else(|| anyhow::anyhow!("wtest_usd not found in asset cache"))?;

    // Generate a random nonce
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);
    let mut nonce_bytes = [0u8; 32];
    rand_core::RngCore::fill_bytes(&mut rng, &mut nonce_bytes);

    // Create a token create plan
    let create_plan = TokenCreatePlan {
        metadata,
        nonce: nonce_bytes,
        initial_supply: Amount::from(1_000_000u64),
    };

    // Convert the plan to an action
    let create = create_plan.to_action();

    // Execute the create action
    create.check_stateless(()).await?;
    create.check_historical(state.clone()).await?;
    let mut state_tx = state.try_begin_transaction().unwrap();
    create.check_and_execute(&mut state_tx).await?;
    state_tx.apply();

    Ok(())
}

#[tokio::test]
async fn token_create_with_zero_supply() -> anyhow::Result<()> {
    let storage = TempStorage::new_with_penumbra_prefixes()
        .await?
        .apply_default_genesis()
        .await?;
    let mut state = StateDelta::new(storage.latest_snapshot());

    TokenFactory::init_chain(&mut state, None).await;

    let mut state = Arc::new(state);

    // Get test metadata from the asset cache
    let metadata = asset::Cache::with_known_assets()
        .get_by_id(asset::Id::from_str("wtest_usd").unwrap())
        .ok_or_else(|| anyhow::anyhow!("wtest_usd not found in asset cache"))?;

    // Generate a random nonce
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);
    let mut nonce_bytes = [0u8; 32];
    rand_core::RngCore::fill_bytes(&mut rng, &mut nonce_bytes);

    // Create a token create plan with zero initial supply
    let create_plan = TokenCreatePlan {
        metadata,
        nonce: nonce_bytes,
        initial_supply: Amount::from(0u64),
    };

    // Convert the plan to an action
    let create = create_plan.to_action();

    // Execute the create action
    create.check_stateless(()).await?;
    create.check_historical(state.clone()).await?;
    let mut state_tx = state.try_begin_transaction().unwrap();
    create.check_and_execute(&mut state_tx).await?;
    state_tx.apply();

    Ok(())
}

#[tokio::test]
async fn token_create_duplicate_id() -> anyhow::Result<()> {
    let storage = TempStorage::new_with_penumbra_prefixes()
        .await?
        .apply_default_genesis()
        .await?;
    let mut state = StateDelta::new(storage.latest_snapshot());

    TokenFactory::init_chain(&mut state, None).await;

    let mut state = Arc::new(state);

    // Get test metadata from the asset cache
    let metadata = asset::Cache::with_known_assets()
        .get_by_id(asset::Id::from_str("wtest_usd").unwrap())
        .ok_or_else(|| anyhow::anyhow!("wtest_usd not found in asset cache"))?;

    // Generate a random nonce
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);
    let mut nonce_bytes = [0u8; 32];
    rand_core::RngCore::fill_bytes(&mut rng, &mut nonce_bytes);

    // Create a token create plan
    let create_plan = TokenCreatePlan {
        metadata: metadata.clone(),
        nonce: nonce_bytes,
        initial_supply: Amount::from(1_000_000u64),
    };

    // Convert the plan to an action
    let create = create_plan.to_action();

    // Execute the create action first time
    create.check_stateless(()).await?;
    create.check_historical(state.clone()).await?;
    let mut state_tx = state.try_begin_transaction().unwrap();
    create.check_and_execute(&mut state_tx).await?;
    state_tx.apply();

    // Try to create the same token again
    let create_plan = TokenCreatePlan {
        metadata,
        nonce: nonce_bytes,
        initial_supply: Amount::from(1_000_000u64),
    };
    let create = create_plan.to_action();

    // This should fail because the token ID already exists
    create.check_stateless(()).await?;
    assert!(create.check_historical(state.clone()).await.is_err());

    Ok(())
} 