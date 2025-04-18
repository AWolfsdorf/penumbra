use penumbra_sdk_asset::asset;

/// A token factory NFT represents minting rights for a token created through the token factory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenFactoryNft {
    token_id: String,
    sequence: u64,
    base_denom: asset::Metadata,
}

impl TokenFactoryNft {
    pub fn new(token_id: String, sequence: u64) -> Self {
        let base_denom = asset::REGISTRY
            .parse_denom(&format!("factory_mint_{sequence}_{token_id}"))
            .expect("base denom format is valid");
        
        TokenFactoryNft {
            token_id,
            sequence,
            base_denom,
        }
    }
    
    pub fn asset_id(&self) -> asset::Id {
        self.base_denom.id()
    }

    pub fn token_id(&self) -> &str {
        &self.token_id
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn next_sequence(&self) -> Self {
        Self::new(self.token_id.clone(), self.sequence + 1)
    }
}

impl std::fmt::Display for TokenFactoryNft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "factory_mint_{}_{}", self.sequence, self.token_id)
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
        let token_id = parts[3].to_string();
        Ok(Self::new(token_id, sequence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_factory_nft_creation() {
        let token_id = "test123".to_string();
        let sequence = 0;
        let nft = TokenFactoryNft::new(token_id.clone(), sequence);
        
        assert_eq!(nft.token_id(), "test123");
        assert_eq!(nft.sequence(), 0);
        assert_eq!(nft.to_string(), "factory_mint_0_test123");
    }

    #[test]
    fn token_factory_nft_next_sequence() {
        let nft = TokenFactoryNft::new("test123".to_string(), 0);
        let next = nft.next_sequence();
        
        assert_eq!(next.sequence(), 1);
        assert_eq!(next.to_string(), "factory_mint_1_test123");
    }
} 