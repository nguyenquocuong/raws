use clap::{Parser, Subcommand};
use ecs::EcsArgs;
use ecs_exec_checker::EcsExecChecker;
use traits::CommandExecute;

mod ecs;
mod ecs_exec_checker;
mod traits;

#[derive(Parser)]
#[command(name = "raws")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ecs(EcsArgs),
    EcsExecChecker(EcsExecChecker),
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ecs(subcommand) => subcommand.execute().await,
        Commands::EcsExecChecker(subcommand) => subcommand.execute().await,
    }
}
