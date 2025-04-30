# Penumbra Token Factory

The Token Factory is a component that allows users to create and manage custom tokens within the Penumbra ecosystem. It provides functionality for creating tokens, minting additional tokens, and managing minting rights through NFTs.

## Architecture

The token factory is organized into the following components:

### Core Modules

1. **TokenFactoryNft** (`nft.rs`): Represents minting rights for a token created through the token factory.

   - Uses the format `factory_mint_[N]_[ID]` for NFT denominations
   - Includes sequence numbers for tracking minting rights transfers

2. **TokenFactory** (`factory.rs`): Main trait for token factory operations.

   - Create new tokens with metadata and initial supply
   - Mint additional tokens using NFT-based authority
   - Burn minting rights when no longer needed

3. **NoteManager** (`note_manager.rs`): Manages the integration with the shielded pool.

   - Add notes to the shielded pool
   - Remove notes from the shielded pool
   - Split notes for partial transfers

4. **NoteFinder** (`note_finder.rs`): Provides functionality to find notes in the shielded pool.
   - Find notes by address
   - Find notes by asset ID
   - Find notes by denomination

### Additional Components

1. **State Keys** (`state_key.rs`): Defines key formats for storing data in the state.

   - Denomination creator mappings
   - Denomination admin mappings

2. **Action Handler** (`action_handler.rs`): Handles token factory actions.
   - Create tokens
   - Mint additional tokens
   - Burn mint authority
   - Create tokens with bonding curves

## Usage Examples

### Creating a New Token

```rust
use penumbra_sdk_token_factory::{
    TokenFactory,
    TokenMetadata,
    TokenFactoryNft,
    Amount,
};

async fn create_new_token(factory: &mut impl TokenFactory) -> anyhow::Result<(String, TokenFactoryNft)> {
    let metadata = TokenMetadata {
        name: "My Token".to_string(),
        symbol: "MTK".to_string(),
        description: "A test token".to_string(),
        uri: None,
        uri_hash: None,
    };

    let initial_supply = Amount::from(1000u128);
    factory.create_token(metadata, initial_supply).await
}
```

### Minting Additional Tokens

```rust
async fn mint_more_tokens(
    factory: &mut impl TokenFactory,
    nft: &TokenFactoryNft,
    amount: Amount,
) -> anyhow::Result<TokenFactoryNft> {
    factory.mint_token(nft, amount).await
}
```

### Burning Minting Rights

```rust
async fn burn_minting_rights(
    factory: &mut impl TokenFactory,
    nft: &TokenFactoryNft,
) -> anyhow::Result<()> {
    factory.burn_mint_authority(nft).await
}
```

### Complete Example Workflow

```rust
async fn token_factory_workflow(factory: &mut impl TokenFactory) -> anyhow::Result<()> {
    // 1. Create a new token
    let (denom, nft) = create_new_token(factory).await?;
    println!("Created token with denom: {}", denom);

    // 2. Mint more tokens
    let new_nft = mint_more_tokens(factory, &nft, Amount::from(500u128)).await?;
    println!("Minted more tokens, new NFT sequence: {}", new_nft.sequence());

    // 3. Burn minting rights
    burn_minting_rights(factory, &new_nft).await?;
    println!("Burned minting rights");

    Ok(())
}
```

## Integration with Shielded Pool

The token factory integrates with Penumbra's shielded pool to ensure privacy for all token operations:

1. When tokens are created or minted, notes are added to the shielded pool.
2. Minting rights are represented as NFTs that can be transferred like any other asset.
3. Token balances are private and stored as encrypted notes.

## Security Considerations

1. **NFT-based Authority**: Only the owner of the minting NFT can mint new tokens.
2. **Sequence Numbers**: Each minting operation increments the sequence number to prevent replay attacks.
3. **Privacy**: All operations are conducted within the shielded pool for maximum privacy.

## Implementation Notes

1. The token factory follows patterns used by other components like AuctionNft and LpNft.
2. Asset IDs are generated deterministically based on the token ID and metadata.
3. Minting rights can be transferred as standard assets within the Penumbra ecosystem.

## Future Enhancements

1. **Bonding Curve Support**: Automatic creation of token pools with bonding curves.
2. **Admin Controls**: Additional functionality for token administrators.
3. **Metadata Updates**: Ability to update token metadata after creation.
