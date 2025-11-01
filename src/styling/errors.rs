use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Errors that may be thrown by the VisuaLife styling utilities
pub enum StylingError {

    #[error("unknown color palette name: {palette_name}")]
    /// Unknown color palette name
    UnknownColorPalette {
        /// name of the color palette, the incorrect one
        palette_name: String,
    },

    #[error("colors and stops must have the same length and at least 2 points; given {n_colors} colors and {n_stops} stops")]
    /// Incorrect definition of a color scale
    ColorScaleDefinitionError {
        /// number of colors provided
        n_colors: usize,
        /// number of color stops
        n_stops: usize,
    },

    #[error("invalid hex color format")]
    /// Invalid hex color format
    InvalidHexColor {
        /// the failing hex string
        color: String,
    },
}
