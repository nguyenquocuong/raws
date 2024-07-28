use async_trait::async_trait;

#[async_trait]
pub trait CommandExecute {
    async fn execute(&self) -> Result<(), std::io::Error>;
}
