use penumbra_sdk_asset::asset;
use penumbra_sdk_proto::serializers::bech32str;
use regex::Regex;
use serde::{Deserialize, Serialize};

use penumbra_sdk_proto::core::component::tokenfactory::v1alpha as pb;

/// A token factory NFT represents minting rights for a token created through the token factory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenFactoryNft {
    /// The unique identifier for the token factory this nft resolves to.
    pub token_id: TokenId,
    /// The sequence number of the token factory this nft resolves to.
    pub seq: u64,
    /// The metadata corresponding to the nft denom.
    pub metadata: asset::Metadata,
}

impl TokenFactoryNft {
    pub fn new(token_id: TokenId, seq: u64) -> Self {
        let metadata = asset::REGISTRY
            .parse_denom(&format!("factory_mint_{seq}_{token_id}"))
            .expect("base denom format is valid");

        TokenFactoryNft {
            token_id,
            seq,
            metadata,
        }
    }

    pub fn asset_id(&self) -> asset::Id {
        self.metadata.id()
    }

    pub fn next_sequence(&self) -> Self {
        TokenFactoryNft::new(self.token_id.clone(), self.seq + 1)
    }
}

impl std::fmt::Display for TokenFactoryNft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "factory_mint_{}_{}", self.seq, self.token_id)
    }
}

impl std::str::FromStr for TokenFactoryNft {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 4 || parts[0] != "factory" || parts[1] != "mint" {
            return Err(anyhow::anyhow!("invalid token factory NFT format"));
        }
        let sequence = parts[2].parse()?;
        let token_id = parts[3].parse()?;
        Ok(Self::new(token_id, sequence))
    }
}

impl TryFrom<asset::Metadata> for TokenFactoryNft {
    type Error = anyhow::Error;

    fn try_from(denom: asset::Metadata) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"factory_mint_(\d+)_(.+)").expect("regex is valid");

        let denom_string = denom.to_string();

        let captures = regex
            .captures(&denom_string)
            .ok_or_else(|| anyhow::anyhow!("invalid token factory NFT format"))?;

        let sequence = captures
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("sequence not found"))?
            .as_str()
            .parse()?;
        let token_id = captures

        Ok(TokenFactoryNft::new(token_id, sequence))
    }
}

#[cfg(test)]
mod tests {

    use crate::TokenFactoryPosition;

    use super::*;

    #[test]
    fn token_factory_nft_creation() {
        let position = TokenFactoryPosition::new(
            rand_core::OsRng,
        );
        
        let token_id = position.id();
        let sequence = 0;
        let nft = TokenFactoryNft::new(token_id.clone(), sequence);

        assert_eq!(nft.token_id, token_id);
        assert_eq!(nft.seq, 0);
        assert_eq!(nft.to_string(), "factory_mint_0_test123");
    }

    #[test]
    fn token_factory_nft_next_sequence() {
        let position = TokenFactoryPosition::new(
            rand_core::OsRng,
        );
        
        let token_id = position.id();
        let sequence = 0;
        let nft = TokenFactoryNft::new(token_id.clone(), sequence);
        let next = nft.next_sequence();

        assert_eq!(next.seq, 1);
        assert_eq!(next.to_string(), "factory_mint_1_test123");
    }
}
