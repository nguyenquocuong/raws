use aws_sdk_ecs::types::ClusterField;
use clap::Args;
use colored::Colorize;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct EcsExecChecker {
    #[arg(short, long)]
    #[arg(required = true)]
    cluster_name: String,

    #[arg(short, long)]
    #[arg(required = true)]
    ecs_task_id: String,
}

impl EcsExecChecker {
    pub async fn execute(&self) {
        println!("{self:?}");

        let config = aws_config::load_from_env().await;

        println!("Region : {}", config.region().unwrap());
        println!("Cluster: {}", self.cluster_name);
        println!("Task   : {}", self.ecs_task_id);

        println!("Check cluster configurations");
        let ecs_client = aws_sdk_ecs::Client::new(&config);

        let clusters = ecs_client
            .describe_clusters()
            .clusters(self.cluster_name.clone())
            .include(ClusterField::Configurations)
            .send()
            .await
            .unwrap();

        if !clusters.failures().is_empty() {
            panic!("{:#?}", clusters.failures());
        }

        match clusters.clusters()[0].configuration() {
            None => println!(
                "Cluster configuration: {}",
                "Audit Logging Not Configured".yellow()
            ),
            Some(_) => {
                println!("{:#?}", clusters.clusters());
                todo!()
            }
        }
    }
}
