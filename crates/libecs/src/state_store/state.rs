use std::collections::HashMap;

use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput;

#[derive(Debug, Clone)]
pub struct ContextInfo {
    pub caller_identity: GetCallerIdentityOutput,
}

#[derive(Debug, Clone)]
pub struct ClusterItem {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub context_info: Option<ContextInfo>,
    pub cluster_map: HashMap<String, ClusterItem>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            context_info: None,
            cluster_map: HashMap::new(),
        }
    }
}
