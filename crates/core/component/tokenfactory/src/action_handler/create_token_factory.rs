use cnidarium_component::ActionHandler;
use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ActionTokenFactoryCreate {
    // Add fields as needed
}

#[async_trait]
impl ActionHandler for ActionTokenFactoryCreate {
    type CheckStatelessContext = ();
    type CheckAndExecuteContext = ();

    async fn check_stateless(&self, _context: Self::CheckStatelessContext) -> Result<()> {
        Ok(())
    }

    async fn check_and_execute(&self, _context: Self::CheckAndExecuteContext) -> Result<()> {
        Ok(())
    }
}