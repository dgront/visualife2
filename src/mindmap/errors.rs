use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Errors that may be thrown by the heatmap module
pub enum MindmapError {
    #[error("Can't find node for the ID: {node_id}")]
    /// Can't find node for the given ID
    NodeNotFound {
        /// the given node ID
        node_id: String,
    },
}

impl MindmapError {
    /// Creates an error about a missing node
    pub fn node_not_found(id: impl Into<String>) -> Self {
        MindmapError::NodeNotFound { node_id: id.into() }
    }
}
