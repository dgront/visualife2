use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PlotError {
    /// At least one primary axis is missing in this plot
    #[error("At least one primary axis is missing in this plot")]
    NoAxisDefined,

    /// Can't write the plot file
    #[error("Can't write the plot file: {filename}")]
    CantWritePlotImage {filename: String, reason: String },
}