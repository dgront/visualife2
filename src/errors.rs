use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Errors that may be thrown by the core of the VisuaLife crate
pub enum VisualifeError {
    #[error("Group not found: {group_id}")]
    /// Can't find a group element for a given id
    NoSuchGroup {
        /// failing group id
        group_id: String,
    },

    #[error("Element not found: {group_id}")]
    /// Can't find an element for a given id
    NoSuchElement {
        /// failing element id
        group_id: String,
    },

    #[error("Element of this ID: {group_id} already exists in the SvgDrawing")]
    /// Can't find an element for a given id
    DuplicatedId {
        /// failing element id
        group_id: String,
    },
}
