#[derive(Debug, Clone)]
pub struct ClusterItem {
    pub arn: String,
    pub name: String,
}

impl From<String> for ClusterItem {
    fn from(arn: String) -> Self {
        let (_, name) = arn.split_once('/').unwrap();
        Self {
            arn: arn.clone(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub caller_arn: Option<String>,
    pub cluster_arns: Vec<String>,
}

//impl Default for State {
//    fn default() -> Self {
//        Self {
//            caller_arn: None,
//            cluster_arns: None,
//        }
//    }
//}
