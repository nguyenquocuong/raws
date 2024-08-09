#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Noop,
    Quit,
    Tick,
    Render,

    GetContextInfo,
    GetClusters,
}
