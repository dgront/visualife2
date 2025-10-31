use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Errors that may be thrown by the heatmap module
pub enum HeatmapError {
    #[error("Expected aligned sequences of length {n_labels_expected}, found sequences of lengths: {n_labels_found}")]
    /// Expected number of labels differs from the actual one
    IncorrectNumberOfLabels {
        /// Expected number of labels
        n_labels_expected: usize,
        /// Found number of labels
        n_labels_found: usize,
    },
}
