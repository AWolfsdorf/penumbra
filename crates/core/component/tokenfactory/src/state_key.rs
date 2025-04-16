use penumbra_sdk_asset::asset;

pub mod denom_creator {
    pub fn by_denom(denom: &str) -> String {
        format!("tokenfactory/denom_creator/{}", denom)
    }
}

pub mod denom_admin {
    pub fn by_denom(denom: &str) -> String {
        format!("tokenfactory/denom_admin/{}", denom)
    }
} 