use crate::basic_shapes::SvgElement;
use crate::ElementID;
use crate::heatmap::Heatmap;
use crate::plots::{AxisIntercept, AxisSet, Box2D, matrix_shape, PLOT_FONT_FAMILY, PLOT_FONT_WEIGHT, PlotError, point_outside_box, TickDirection, update_plot_box};
use crate::styling::{darker, Style};

use std::fmt;
use std::str::FromStr;
use crate::styling::palettes::{ACCENT};

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
    axes2: Option<AxisSet>,
    screen_x0: f32,
    screen_y0: f32,
    svg_elements: Vec<SvgElement>,
    scatter_series: Vec<DataSeries>
}

impl Plot {
    pub fn new(id: impl Into<ElementID>, axes: AxisSet) -> Self {
        Plot{ id: id.into(), axes, axes2: None, screen_x0: 0.0, screen_y0: 0.0, svg_elements: vec![], scatter_series: vec![] }
    }

    /// Cartesian axes that cross at 0,0
    pub fn cartesian<R: Into<Box2D<f32>>>(id: impl Into<ElementID>, screen_box: R) -> Self {
        let mut axes = AxisSet::new(screen_box);
        axes.set_intercept_point(0.0, 0.0);
        axes.has_arrowhead = true;
        axes.set_intercept_point(0.0, 0.0);
        axes.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(7);

        return Plot::new(id, axes);
    }

    pub fn rectangular<R: Into<Box2D<f32>>>(id: impl Into<ElementID>, screen_box: R) -> Self {
        let screen_box = screen_box.into();
        let mut axes = AxisSet::new(screen_box.clone());
        axes.has_arrowhead = false;
        axes.set_intercept(AxisIntercept::AutoStart, AxisIntercept::AutoStart);
        axes.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(7);
        axes.y.ticks_location = TickDirection::UpLeft;
        axes.y.label_location = TickDirection::UpLeft;
        axes.x.ticks_location = TickDirection::DownRight;
        axes.x.label_location = TickDirection::DownRight;

        let mut plot = Plot::new(id, axes);

        let mut axes2 = AxisSet::new(screen_box.clone());
        axes2.set_intercept(AxisIntercept::AutoEnd, AxisIntercept::AutoEnd);
        axes2.has_arrowhead = false;
        axes2.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes2.x.set_nticks(7);
        axes2.y.set_nticks(7);
        axes2.y.ticks_location = TickDirection::DownRight;
        axes2.y.label_location = TickDirection::DownRight;
        axes2.x.ticks_location = TickDirection::UpLeft;
        axes2.x.label_location = TickDirection::UpLeft;

        plot.axes2 = Some(axes2);

        return plot;
    }

    pub fn set_plot_box<R: Into<Box2D<f32>>>(&mut self, range: R)  {
        let data_box = range.into();
        if let Some(ax2) = &mut self.axes2 { ax2.set_plot_box(data_box.clone())}
        self.axes.set_plot_box(data_box);
    }

    pub fn set_nticks(&mut self, n_ticks: usize)  {
        if let Some(ax2) = &mut self.axes2 {
            ax2.x.set_nticks(n_ticks);
            ax2.y.set_nticks(n_ticks);
        }
        self.axes.x.set_nticks(n_ticks);
        self.axes.y.set_nticks(n_ticks);
    }


    pub fn axes_mut(&mut self) -> &mut AxisSet { &mut self.axes }

    pub fn axes2_mut(&mut self) -> &mut Option<AxisSet> { &mut self.axes2 }

    pub fn translate(&mut self, offset_x: f32, offset_y: f32) {
        self.screen_x0 = offset_x;
        self.screen_y0 = offset_y;
    }

    pub fn scatter(&mut self, x: &[f32], y: &[f32]) {
        assert_eq!(x.len(), y.len(), "x and y must have the same length");

        let mut pbox = self.axes.plot_box();
        if point_outside_box(x, y, &pbox) {
            self.axes.set_plot_box(update_plot_box(x,y,&pbox));
        }
        let data = x.iter()
            .copied()
            .zip(y.iter().copied())
            .collect();
        self.scatter_series.push(
            DataSeries{ data: data, marker: MarkerType::Circle {}, marker_size: 7.0, color: ACCENT[0] })
    }

    /// Plots a heatmap from a rectangular 2D dataset
    pub fn heatmap<I, R, T>(&mut self, data: I) -> Result<(), PlotError>
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<f64> {

        let screen = self.axes.screen_box();

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

        // --- draw axes
        plot_components.push(self.axes.create_element());
        if let Some(ax2) = &self.axes2 {
            plot_components.push(ax2.create_element());
        }

        // --- draw explicit SVG content
        for c in &self.svg_elements {
            plot_components.push(c.clone());
        }
        // --- draw scatter series
        for ser in &self.scatter_series {
            let mut markers = vec![];
            let size = 5.0_f32;
            for (i,(dx,dy)) in ser.data.iter().enumerate() {
                let (sx,sy) = self.axes.to_screen(*dx, *dy);
                markers.push(ser.marker.draw(&format!("s1{}", i), sx, sy, size));
            }

            let series = SvgElement::group("s1", markers)
                .with_style(Style::new().stroke(&darker(ser.color, 0.1).unwrap()).fill(ser.color));
            plot_components.push(series);
        }

        // --- group for the whole plot
        let mut plot = SvgElement::group(format!("{}", self.id), plot_components)
            .with_style(Style::new().font_family(PLOT_FONT_FAMILY).font_weight(PLOT_FONT_WEIGHT));
        plot.translate(self.screen_x0, self.screen_y0);

        return plot;
    }
}

struct DataSeries {
    data: Vec<(f32,f32)>,
    marker: MarkerType,
    marker_size: f32,
    color: &'static str
}
