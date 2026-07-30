mod axes;

pub use axes::*;

mod box2d;
pub use box2d::*;

mod plot;
pub use plot::*;

mod plot_error;
pub use plot_error::*;

mod markers;
pub use markers::*;

pub(crate) const PLOT_FONT_FAMILY: &str = "DejaVuSansRegular,'DejaVu Sans','Liberation Sans',Arial,Helvetica,sans-serif;";
pub(crate) const PLOT_FONT_WEIGHT: &str = "300";


