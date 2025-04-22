pub struct TokenFactoryParams {
    pub is_enabled: bool,
}

impl std::fmt::Debug for TokenFactoryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenFactoryParams {{ is_enabled: {} }}", self.is_enabled)
    }
}

impl Clone for TokenFactoryParams {
    fn clone(&self) -> Self {
        TokenFactoryParams { is_enabled: self.is_enabled }
    }
}




