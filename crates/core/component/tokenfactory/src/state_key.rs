use penumbra_sdk_asset::asset;

pub mod parameters {
    pub fn key() -> &'static str {
        "tokenfactory/params"
    }

    pub fn updated_flag() -> &'static str {
        "tokenfactory/params/updated"
    }
}

pub fn token_supply(asset_id: &asset::Id) -> String {
    format!("factory/{}", asset_id)
}

pub fn nft_minting_rights(seq: u64, asset_id: &asset::Id) -> String {   
    format!("factory_mint_{}_{}", seq, asset_id)
}
