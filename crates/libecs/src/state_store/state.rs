#[derive(Debug, Clone)]
pub struct ClusterItem {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub caller_arn: Option<String>,
    pub cluster_map: Vec<ClusterItem>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            caller_arn: None,
            cluster_map: Vec::new(),
        }
    }
}
