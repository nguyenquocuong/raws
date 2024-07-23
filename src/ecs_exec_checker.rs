use aws_sdk_ecs::types::ClusterField;
use aws_sdk_iam::types::PolicyEvaluationDecisionType;
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
        let sts_client = aws_sdk_sts::Client::new(&config);
        let iam_client = aws_sdk_iam::Client::new(&config);
        let ecs_client = aws_sdk_ecs::Client::new(&config);

        println!("Region : {}", config.region().unwrap());
        println!("Cluster: {}", self.cluster_name);
        println!("Task   : {}", self.ecs_task_id);

        println!("Get caller identity");
        let caller_identity = sts_client.get_caller_identity().send().await.unwrap();

        println!("Check cluster configurations");
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

        println!("Can I ExecuteCommand? {}", caller_identity.arn().unwrap());
        println!(
            "\tecs:ExecuteCommand: {}",
            match iam_client
                .simulate_principal_policy()
                .policy_source_arn(caller_identity.arn().unwrap())
                .action_names("ecs:ExecuteCommand")
                .resource_arns(format!(
                    "arn:aws:ecs:{}:{}:task/{}/{}",
                    config.region().unwrap(),
                    caller_identity.account().unwrap(),
                    self.cluster_name,
                    self.ecs_task_id
                ))
                .send()
                .await
                .unwrap()
                .evaluation_results()[0]
                .eval_decision()
            {
                PolicyEvaluationDecisionType::Allowed =>
                    PolicyEvaluationDecisionType::Allowed.to_string().green(),
                decision => decision.to_string().yellow(),
            }
        )
    }
}
