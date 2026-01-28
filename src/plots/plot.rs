use crate::basic_shapes::SvgElement;
use crate::ElementID;
use crate::heatmap::Heatmap;
use crate::plots::{AxisSet, AxisSetBuilder, Box2D, matrix_shape, PlotError, point_outside_box};
use crate::styling::Style;

use std::fmt;
use std::str::FromStr;

/// Marker symbols for scatter plots (subset of Matplotlib markers).
///
/// Supported markers:
/// - `"+"` : plus
/// - `"x"` : cross
/// - `"o"` / `"c"` : empty circle
/// - `"O"` / `"C"` : filled circle
/// - `"."` : point
/// - `"s"` : empty square
/// - `"s"` : filled square
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Plus,
    Cross,
    Circle,
    FilledCircle,
    Point,
    Square,
    FilledSquare
}

impl MarkerType {
    /// Create SVG elements representing this marker centered at (cx, cy).
    ///
    /// `size` is the full marker size in screen units.
    pub fn draw(&self, id: &str, cx: f32, cy: f32, size: f32) -> SvgElement {
        let h = size * 0.5;

        match self {
            MarkerType::Plus => {
                let mut elems = Vec::new();
                elems.push(SvgElement::line(format!("{id}_h"), cx - h, cy, cx + h, cy));
                elems.push(SvgElement::line(format!("{id}_v"), cx, cy - h, cx, cy + h));
                let grp = SvgElement::group(id, elems);
                return grp;
            }

            MarkerType::Cross => {
                let mut elems = Vec::new();
                elems.push(SvgElement::line(format!("{id}_d1"), cx - h, cy - h, cx + h, cy + h));
                elems.push(SvgElement::line(format!("{id}_d2"), cx - h, cy + h, cx + h, cy - h));
                let grp = SvgElement::group(id, elems);
                return grp;
            }

            MarkerType::Circle | MarkerType::FilledCircle => {
                return SvgElement::circle(format!("{id}_c"), cx, cy, h);
            }

            MarkerType::Point => {
                return SvgElement::circle(format!("{id}_p"), cx, cy, h * 0.3);
            }

            MarkerType::Square | MarkerType::FilledSquare => {
                return SvgElement::rect(format!("{id}_s"), cx - h, cy - h, size, size);
            }
        }
    }
}


impl FromStr for MarkerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MarkerType::*;

        match s {
            "+" => Ok(Plus),
            "x" => Ok(Cross),
            "." => Ok(Point),
            "o" | "c" => Ok(Circle),
            "O" | "C" => Ok(FilledCircle),
            "s" => Ok(Square),
            "S" => Ok(FilledSquare),
            _ => Err(format!("Unknown marker type: '{}'", s)),
        }
    }
}

impl fmt::Display for MarkerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MarkerType::*;

        let s = match self {
            Plus => "+",
            Cross => "x",
            Point => ".",
            Circle => "o",
            FilledCircle => "O",
            Square  => "s",
            FilledSquare => "S",
        };

        write!(f, "{}", s)
    }
}


pub struct Plot {
    pub id: ElementID,
    axes: AxisSet,
    screen_x0: f32,
    screen_y0: f32,
    svg_elements: Vec<SvgElement>,
    scatter_series: Vec<DataSeries>
}

impl Plot {
    pub fn new(id: impl Into<ElementID>, axes: AxisSet) -> Self {
        Plot{ id: id.into(), axes, screen_x0: 0.0, screen_y0: 0.0, svg_elements: vec![], scatter_series: vec![] }
    }

    pub fn cartesian(id: impl Into<ElementID>, plotting_box: Box2D<f32>) -> Self {
        let axes = AxisSetBuilder::new("LB", plotting_box)
            .center(0.0, 0.0).arrowheads(true).ntics(5).build();

        return Plot::new(id, axes);
    }

    pub fn axes(&mut self) -> &mut AxisSet { &mut self.axes }

    pub fn translate(&mut self, offset_x: f32, offset_y: f32) {
        self.screen_x0 = offset_x;
        self.screen_y0 = offset_y;
    }

    pub fn scatter(&mut self, x: &[f32], y: &[f32]) {
        assert_eq!(x.len(), y.len(), "x and y must have the same length");

        let pbox = self.axes.plot_box();
        if point_outside_box(x, y, pbox) {

        }
        // if point_outside_box(&x, &y, self.pl)
        let data = x.iter()
            .copied()
            .zip(y.iter().copied())
            .collect();
        self.scatter_series.push(DataSeries{ data: data, marker: MarkerType::Circle {} })
    }

    /// Plots a heatmap from a rectangular 2D dataset
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

    /// Creates an SVG group element that holds all the graphical elements of this plot
    pub fn create_element(&self) -> SvgElement {
        let mut plot_components = vec![];
        for c in &self.svg_elements {
            plot_components.push(c.clone());
        }
        for ser in &self.scatter_series {
            let mut markers = vec![];
            let size = 5.0_f32;
            for (i,(dx,dy)) in ser.data.iter().enumerate() {
                let (sx,sy) = self.axes.to_screen(*dx, *dy);
                markers.push(ser.marker.draw(&format!("s1{}", i), sx, sy, size));
            }
            let series = SvgElement::group("s1", markers)
                .with_style(Style::new().stroke("#000000").fill("#000000"));
            plot_components.push(series);
        }
        plot_components.push(self.axes.create_element());
        let mut plot = SvgElement::group(format!("{}", self.id), plot_components)
            .with_style(Style::new().font_family("'system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', 'Liberation Sans', sans-serif'"));
        plot.translate(self.screen_x0, self.screen_y0);

        return plot;
    }
}

struct DataSeries {
    data: Vec<(f32,f32)>,
    marker: MarkerType
}
