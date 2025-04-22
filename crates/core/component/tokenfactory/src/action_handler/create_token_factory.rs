use cnidarium_component::ActionHandler;
use async_trait::async_trait;


#[async_trait]
impl ActionHandler for ActionTokenFactoryCreate {
    type CheckStatelessContext = ();

    async fn check_stateless(&self, _context: Self::CheckStatelessContext) -> Result<()> {
        Ok(())
    }

    async fn check_and_execute(&self, _context: Self::CheckAndExecuteContext) -> Result<()> {
        Ok(())
    }
}