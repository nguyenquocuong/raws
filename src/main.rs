use clap::{Parser, Subcommand};
use ecs_exec_checker::EcsExecChecker;

mod ecs_exec_checker;

#[derive(Parser)]
#[command(name = "raws")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    EcsExecChecker(EcsExecChecker),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::EcsExecChecker(subcommand) => subcommand.execute().await,
    }
}
