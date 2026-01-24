use crate::basic_shapes::SvgElement;
use crate::ElementID;
use crate::heatmap::Heatmap;
use crate::plots::{AxisSet, matrix_shape, PlotError};
use crate::styling::Style;

/// and how coordinate mapping behaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotType {
    /// Standard Cartesian plot:
    /// Bottom X axis, Left Y axis.
    ///
    /// The most common scientific plot layout.
    Cartesian,

    /// Cartesian plot with axes crossing at (0, 0).
    ///
    /// Useful for mathematical function plots.
    CartesianCentered,

    /// Four-sided rectangular frame:
    /// Top, Bottom, Left, Right axes.
    ///
    /// Often used in publications and boxed charts.
    Rectangular,

    /// Only X axis at the bottom.
    ///
    /// Useful for time-series strips, histograms, or aligned panels.
    XOnly,

    /// Only Y axis on the left.
    ///
    /// Useful for vertical strips or color bars.
    YOnly,

    /// Full 4-sided axes with center crossing at (0, 0).
    ///
    /// Rare but useful for symmetric scientific diagrams.
    RectangularCentered,
}

pub struct Plot {
    pub id: ElementID,
    axes: AxisSet,
    screen_width: f32,
    screen_height: f32,
    screen_x0: f32,
    screen_y0: f32,
    svg_elements: Vec<SvgElement>
}

impl Plot {
    pub fn new(id: impl Into<ElementID>, axes: AxisSet, screen_width: f32, screen_height: f32) -> Self {
        Plot{
            id: id.into(),
            axes,
            screen_width,
            screen_height,
            screen_x0: 0.0,
            screen_y0: 0.0,
            svg_elements: vec![],
        }
    }

    pub fn translate(&mut self, offset_x: f32, offset_y: f32) {
        self.screen_x0 = offset_x;
        self.screen_y0 = offset_y;
    }

    pub fn heatmap<I, R, T>(&mut self, data: I) -> Result<(), PlotError>
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<f64> {

        let screen = self.axes.screen_box()?;

        let data: Vec<Vec<f64>> = data
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect();

        let (n_rows, n_cols) = matrix_shape(&data);
        let htmp = Heatmap::from_matrix(format!("{}htmp", &self.id),
            screen.width() / n_cols as f32, screen.height() / n_rows as f32, data);
        let mut htmp_svg = htmp.create_element();
        htmp_svg.translate(screen.x_beg, screen.y_beg);
        self.svg_elements.push(htmp_svg);

        return Ok(());
    }

    pub fn create_element(&self) -> SvgElement {
        let mut plot_components = vec![];
        for c in &self.svg_elements {
            plot_components.push(c.clone());
        }
        plot_components.push(self.axes.create_element());
        SvgElement::group(format!("{}", self.id), plot_components)
            .with_style(Style::new().font_family("'system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', 'Liberation Sans', sans-serif'"))
    }
}

