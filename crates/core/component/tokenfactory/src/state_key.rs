pub mod parameters {
    pub fn key() -> &'static str {
        "tokenfactory/params"
    }

    pub fn updated_flag() -> &'static str {
        "tokenfactory/params/updated"
    }
}

pub mod token_factory {
    use penumbra_sdk_asset::asset;
    
    pub fn prefix() -> &'static str {
        "factory/"
    }

    pub fn by_id(asset_id: &asset::Id) -> String {
        format!("{}{asset_id}", prefix())
    }
}

pub mod token_factory_nft {
    use penumbra_sdk_asset::asset;

    pub fn prefix() -> &'static str {
        "factory_mint/"
    }

    pub fn nft_minting_rights(seq: u64, asset_id: &asset::Id) -> String {
        format!("{}{seq}_{}", prefix(), asset_id)
    }
}


