use crate::traits::CommandExecute;

use async_trait::async_trait;
use clap::Args;
use std::io::Result;

use libecs::run_app;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct EcsArgs;

#[async_trait]
impl CommandExecute for EcsArgs {
    async fn execute(&self) -> Result<()> {
        run_app().await
    }
}
