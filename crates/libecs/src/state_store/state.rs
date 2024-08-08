use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ClusterItem {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub cluster_map: HashMap<String, ClusterItem>,
}
