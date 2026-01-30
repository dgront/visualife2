mod axes;

use base64::Engine;
pub use axes::*;

mod box2d;
pub use box2d::*;

pub use crate::utils::*;

mod plot;
pub use plot::*;

mod plot_error;
pub use plot_error::*;

pub(crate) const PLOT_FONT_FAMILY: &str = "font-family:'DejaVu Sans','Liberation Sans',Arial,Helvetica,sans-serif;";
pub(crate) const PLOT_FONT_WEIGHT: &str = "400";


