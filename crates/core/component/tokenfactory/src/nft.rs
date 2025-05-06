use penumbra_sdk_asset::asset::{self, Metadata};
use anyhow::Result;
use regex::Regex;

use penumbra_sdk_proto::{core::component::tokenfactory::v1alpha as pb, DomainType};

use crate::TokenId;

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

    pub fn next_sequence(&self) -> Self {
        TokenFactoryNft::new(self.token_id.clone(), self.seq + 1)
    }
}

/* Protobuf impls */
impl DomainType for TokenFactoryNft {
    type Proto = pb::TokenFactoryNft;
}

impl From<TokenFactoryNft> for pb::TokenFactoryNft {
    fn from(domain: TokenFactoryNft) -> Self {
        Self {
            token_id: Some(domain.token_id.into()),
            seq: domain.seq,
        }
    }
}

impl TryFrom<pb::TokenFactoryNft> for TokenFactoryNft {
    type Error = anyhow::Error;

    fn try_from(msg: pb::TokenFactoryNft) -> Result<Self, Self::Error> {
        let token_id: TokenId = msg
            .token_id
            .ok_or_else(|| anyhow::anyhow!("TokenFactoryNft message is missing a token id"))?
            .try_into()?;
        let seq = msg.seq;
        Ok(TokenFactoryNft::new(token_id, seq))
    }
}

impl TryFrom<Metadata> for TokenFactoryNft {
    type Error = anyhow::Error;

    fn try_from(denom: Metadata) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"factory_mint_(?P<seq_num>[0-9]+)_(?P<token_id>[a-zA-HJ-NP-Z0-9]+)$").expect("regex is valid");

        let denom_string = denom.to_string();

        let captures = regex
            .captures(&denom_string)
            .ok_or_else(|| anyhow::anyhow!("invalid token factory NFT format"))?;

        let token_id = captures
            .name("token_id")
            .ok_or_else(|| anyhow::anyhow!("token id not found"))?
            .as_str();

        let seq_num = captures
            .name("seq_num")
            .ok_or_else(|| anyhow::anyhow!("sequence not found"))?
            .as_str();

        let seq_num: u64 = seq_num
            .parse()
            .map_err(|_| anyhow::anyhow!("Failed to parse seq_num to u64"))?;

        let token_id: TokenId = token_id
            .parse()
            .map_err(|_| anyhow::anyhow!("Failed to parse token_id to TokenId"))?;

        Ok(Self::new(token_id, seq_num))
    }
}

impl std::fmt::Display for TokenFactoryNft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "factory_mint_{}_{}", self.seq, self.token_id)
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
