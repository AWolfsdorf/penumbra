# Penumbra Token Factory

This component is based on the design discussed in the [Penumbra Forum](https://forum.penumbra.zone/t/token-factory-on-penumbra/127/4).

The Token Factory is a component that allows users to create and manage custom tokens within the Penumbra ecosystem. One of its most powerful features is the ability to compose with bonding curves to enable completely fair token launches with no frontrunning of initial price discovery. This is possible because of Penumbra's batched DEX operations, which ensure all users access the bonding curve at the same "time".

The token factory uses a "stateful NFT" pattern for controlling public state from inside the shielded pool, similar to how auctions work in Penumbra. Each action on a token can consume the previous state and produce a new state, maintaining a clear sequence of operations.

## Architecture

The token factory is organized into the following components:

### Core Types

1. **TokenId**: A unique identifier for tokens created through the token factory.
   - 32-byte array derived from a hash of the nonce and initial supply
   - Uses "Penumbra_TF_Id" as the personalization string for the hash

2. **TokenFactoryPosition**: Represents a token's state in the factory.
   - Contains the token ID, nonce, metadata, and initial supply
   - Used to generate proper asset IDs and denominations

### Actions

1. **TokenCreate**: Action to create a new token.
   ```rust
   pub struct TokenCreate {
       pub token_id: TokenId,
       pub nonce: [u8; 32],
       pub initial_supply: Amount,
   }
   ```
   - Creates a new token with the specified initial supply
   - Generates a unique token ID using the nonce and initial supply
   - Creates corresponding metadata and asset ID

2. **TokenBurn**: Action to burn tokens.
   ```rust
   pub struct TokenBurn {
       pub token_id: TokenId,
       pub amount: Amount,
   }
   ```
   - Burns the specified amount of tokens
   - Verifies the token exists and has sufficient supply

3. **TokenFactoryMintAction** (TBD)
   - Will allow minting additional tokens
   - Will require proper authorization
   - Will maintain supply tracking

### State Management

1. **State Keys**: Defines key formats for storing data in the state.
   - Token ID to denomination mappings
   - Token metadata storage
   - Supply tracking

2. **Action Handler**: Handles token factory actions.
   - Validates action parameters
   - Updates state accordingly
   - Maintains consistency of token data

## Usage Examples

### Creating a New Token

```rust
use penumbra_sdk_token_factory::{
    TokenCreate,
    TokenFactoryPosition,
    Amount,
};

async fn create_new_token() -> anyhow::Result<TokenCreate> {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1312);
    let initial_supply = Amount::from(1_000_000u64);
    
    // Create a position to generate the token ID
    let position = TokenFactoryPosition::new(rng, initial_supply);
    
    // Create the token creation action
    let create = TokenCreate {
        token_id: position.token_id,
        nonce: position.nonce,
        initial_supply,
    };

    Ok(create)
}
```

### Burning Tokens

```rust
async fn burn_tokens(token_id: TokenId, amount: Amount) -> anyhow::Result<TokenBurn> {
    let burn = TokenBurn {
        token_id,
        amount,
    };

    Ok(burn)
}
```

## Integration with Shielded Pool

The token factory integrates with Penumbra's shielded pool to ensure privacy for all token operations:

1. When tokens are created, they are added to the shielded pool
2. Token balances are private and stored as encrypted notes
3. All operations maintain privacy guarantees

## Security Considerations

1. **Token ID Generation**: Uses cryptographically secure random nonces
2. **Supply Control**: Tracks and verifies token supply for all operations
3. **Privacy**: All operations are conducted within the shielded pool

## Implementation Notes

1. Token IDs are generated using blake2b with a specific personalization string
2. Asset IDs are derived from token IDs through proper denomination paths
3. All operations maintain proper state consistency

## Future Enhancements

1. **Minting Support**: Implementation of TokenFactoryMintAction
   - Will use NFT-based minting rights (`factory_mint_N_[ID]`)
   - Each mint operation will consume the current mint NFT and produce the next one
   - Supports both centralized minting and one-time minting use cases

2. **Bonding Curve Integration**: Support for fair token launches
   - Create sequences of LP positions with ascending prices to replicate bonding curves
   - Enable multi-asset bonding curves through Penumbra's DEX routing
   - Allow assets used at lower prices to provide liquidity on other routes
   - Support for immutable liquidity through LPNFT burning

3. **Metadata Updates**: Ability to update token metadata after creation
4. **Admin Controls**: Additional functionality for token administrators

## Value Balance Operations

The token factory actions affect the transaction's value balance in the following ways:

### TokenCreate
- `+SUPPLY` of the newly created `factory/[ID]` token
- `+1` of a "token factory NFT", `factory_mint_0_[ID]`, representing minting rights

### TokenBurn
- Consumes the specified value from the transaction's value balance
- Produces nothing (explicit burn)

### TokenFactoryMint (Future)
- `+SUPPLY` of the `factory/[ID]` token
- `-1` of the `factory_mint_[N]_[ID]` NFT with sequence number N
- `+1` of the `factory_mint_[N+1]_[ID]` NFT with sequence number N+1

## Development

### Proto Files
The token factory's protobuf definitions are located in:
- `crates/proto/proto/penumbra/core/component/tokenfactory/v1alpha/tokenfactory.proto`
- Generated Rust code: `crates/proto/src/gen/penumbra.core.component.tokenfactory.v1alpha.rs`

### Running Tests
To run the token factory tests:

```bash
# Run all token factory tests
cargo test -p penumbra-sdk-token-factory

# Run specific test
cargo test -p penumbra-sdk-token-factory token_create

# Run tests with logging
RUST_LOG=debug cargo test -p penumbra-sdk-token-factory -- --nocapture
```

Note: The tests are located in `crates/core/app-tests/tests/` and are part of the app-tests package. To run them directly:

```bash
# Run all app tests
cargo test -p penumbra-sdk-app-tests

# Run specific token factory tests
cargo test -p penumbra-sdk-app-tests token_create
cargo test -p penumbra-sdk-app-tests token_burn
```

### Test Structure
The main test files are:
- `crates/core/app-tests/tests/token_create.rs`: Tests for token creation
- `crates/core/app-tests/tests/token_burn.rs`: Tests for token burning

Each test file uses the `TempStorage` utility to create an isolated test environment.

### Development Setup
1. Ensure you have the required dependencies:
   ```bash
   cargo build -p penumbra-token-factory
   ```

2. The component is part of the core Penumbra stack, so it's automatically included in the main build.

3. For local development, you can use the test utilities in `crates/core/app-tests` to create isolated test environments.

### Common Development Tasks

#### Adding a New Action
1. Create a new action struct in `src/[action_name]/action.rs`
2. Add protobuf definitions in the proto file
3. Implement the necessary traits (DomainType, EffectingData)
4. Add tests in `crates/core/app-tests/tests/`

#### Modifying Token ID Generation
The token ID generation logic is in `src/position.rs`. Any changes should maintain the 32-byte requirement and proper personalization string usage.
