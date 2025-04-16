use std::str::FromStr;

use penumbra_sdk_asset::asset;
use penumbra_sdk_keys::Address;

/// A token factory note represents a token created through the token factory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenFactoryNote {
    creator: String,
    subdenom: String,
    base_denom: asset::Metadata,
}

impl TokenFactoryNote {
    pub fn new(creator: String, subdenom: String) -> Self {
        let denom = format!("factory/{}/{}", creator, subdenom);
        let base_denom = asset::REGISTRY
            .parse_denom(&denom)
            .expect("base denom format is valid");
        TokenFactoryNote {
            creator,
            subdenom,
            base_denom,
        }
    }

    /// Get the base denomination for this token factory note.
    pub fn denom(&self) -> asset::Metadata {
        self.base_denom.clone()
    }

    /// Get the default display denomination for this token factory note.
    pub fn default_unit(&self) -> asset::Unit {
        self.base_denom.default_unit()
    }

    /// Get the asset ID for this token factory note.
    pub fn id(&self) -> asset::Id {
        self.base_denom.id()
    }

    /// Get the creator of this token factory note.
    pub fn creator(&self) -> &str {
        &self.creator
    }

    /// Get the subdenom of this token factory note.
    pub fn subdenom(&self) -> &str {
        &self.subdenom
    }
}

impl TryFrom<asset::Metadata> for TokenFactoryNote {
    type Error = anyhow::Error;

    fn try_from(base_denom: asset::Metadata) -> Result<Self, Self::Error> {
        let base_string = base_denom.to_string();
        let parts: Vec<&str> = base_string.split('/').collect();
        if parts.len() != 3 || parts[0] != "factory" {
            return Err(anyhow::anyhow!("invalid token factory denom format"));
        }
        Ok(Self {
            creator: parts[1].to_string(),
            subdenom: parts[2].to_string(),
            base_denom,
        })
    }
}

impl FromStr for TokenFactoryNote {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        asset::REGISTRY
            .parse_denom(s)
            .ok_or_else(|| anyhow::anyhow!("could not parse {} as base denomination", s))?
            .try_into()
    }
}

impl std::fmt::Display for TokenFactoryNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base_denom.fmt(f)
    }
}

impl std::hash::Hash for TokenFactoryNote {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base_denom.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_factory_note_denomination_round_trip() {
        let creator = "penumbravalid1abcdef".to_string();
        let subdenom = "test".to_string();

        let note = TokenFactoryNote::new(creator.clone(), subdenom.clone());

        let denom = note.to_string();
        let note2 = TokenFactoryNote::from_str(&denom).unwrap();
        let denom2 = note2.to_string();

        assert_eq!(denom, denom2);
        assert_eq!(note, note2);
        assert_eq!(note.creator(), creator);
        assert_eq!(note.subdenom(), subdenom);
    }
} 