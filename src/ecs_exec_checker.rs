use aws_sdk_ecs::types::{ClusterField, LaunchType};
use aws_sdk_iam::types::PolicyEvaluationDecisionType;
use clap::Args;
use colored::Colorize;
use version_compare::{compare_to, Cmp};

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

        let cluster = clusters.clusters().first().unwrap();
        match cluster.configuration() {
            None => println!(
                "Cluster configuration: {}",
                "Audit Logging Not Configured".yellow()
            ),
            Some(_) => {
                println!("{:#?}", cluster);
                todo!()
            }
        }

        let ecs_tasks = ecs_client
            .describe_tasks()
            .cluster(self.cluster_name.clone())
            .tasks(self.ecs_task_id.clone())
            .send()
            .await
            .unwrap();
        if !ecs_tasks.failures().is_empty() {
            println!("The specified ECS task does not exist.");
            panic!("{:#?}", ecs_tasks.failures());
        }

        let ecs_task = ecs_tasks.tasks().first().unwrap();

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
                .evaluation_results()
                .first()
                .unwrap()
                .eval_decision()
            {
                PolicyEvaluationDecisionType::Allowed =>
                    PolicyEvaluationDecisionType::Allowed.to_string().green(),
                decision => decision.to_string().yellow(),
            }
        );
        println!(
            "\tssm:StartSession denied?: {}",
            match iam_client
                .simulate_principal_policy()
                .policy_source_arn(caller_identity.arn().unwrap())
                .action_names("ssm:StartSession")
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
                .evaluation_results()
                .first()
                .unwrap()
                .eval_decision()
            {
                PolicyEvaluationDecisionType::Allowed =>
                    PolicyEvaluationDecisionType::Allowed.to_string().yellow(),
                decision => decision.to_string().green(),
            }
        );

        let task_status = ecs_task.last_status().unwrap();
        let launch_type = ecs_task.launch_type().unwrap();
        println!(
            "{: <21}: {}",
            "Task Status",
            match task_status {
                "RUNNING" => task_status.green(),
                "PROVISIONING" | "ACTIVATING" | "PENDING" => task_status.yellow(),
                "DEACTIVATING" | "STOPPING" | "DEPROVISIONING" => task_status.red(),
                "STOPPED" => format!(
                    "{} ({})",
                    task_status.red(),
                    ecs_task.stopped_reason().unwrap()
                )
                .to_string()
                .into(),
                _ => task_status.red(),
            }
        );

        println!(
            "{: <21}: {}",
            "Launch Type",
            match launch_type {
                LaunchType::Fargate | LaunchType::Ec2 => launch_type.to_string().green(),
                _ => launch_type.to_string().yellow(),
            }
        );

        match launch_type {
            LaunchType::Fargate => {
                let platform_family = ecs_task.platform_family().unwrap();
                let require_platform_version = match platform_family {
                    _ if platform_family.contains("Windows") => "1.0.0",
                    _ => "1.4.0",
                };

                let platform_version = ecs_task.platform_version().unwrap();
                println!(
                    "{: <21}: {}",
                    "Platform Version",
                    match compare_to(platform_version, require_platform_version, Cmp::Ge).unwrap() {
                        true => platform_version.green(),
                        false => format!(
                            "{} (Required: >= {})",
                            platform_version,
                            require_platform_version.red()
                        )
                        .to_string()
                        .into(),
                    }
                );
            }
            _ => todo!(),
        };

        println!(
            "{: <21}: {}",
            "Exec Command Enabled?",
            match ecs_task.enable_execute_command {
                true => "OK".green(),
                false => "NO".red(),
            }
        );
    }
}
