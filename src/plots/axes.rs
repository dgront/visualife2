use std::fmt;
use std::fmt::Display;

use crate::basic_shapes::{SvgElement, triangle_arrow};
use crate::plots::box2d::Box2D;
use crate::plots::{linspace, PLOT_FONT_FAMILY, PLOT_FONT_WEIGHT, PlotError};
use crate::styling::Style;



/// Specifies where tick marks are drawn relative to the plot area
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TickDirection {
    /// Tick marks extend right or down for Y and X axis, respectively
    DownRight,
    /// Tick marks extend up or left for X and Y axis, respectively
    UpLeft,
    /// No tick marks are drawn.
    None,
}

/// A simple tick representation (plot-space value + optional label).
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    /// Tick location in the data / plot coordinates
    pub value: f32,
    /// Optional text label
    ///
    /// When not provided, the actual value will be used
    pub label: Option<String>,
}

impl Tick {
    /// Create a new default tick at a given location
    pub fn new(value: f32) -> Self { Self { value, label: None } }

    /// Create a new labeled tick at a given location
    pub fn with_label(value: f32, label: impl Into<String>) -> Self {
        Self { value, label: Some(label.into()) }
    }

    /// Returns the string label defined for this tick.
    ///
    /// If no label has been assigned, the value converted to string is returned
    pub fn label(&self) -> String {
        return if let Some(lbl) = &self.label { lbl.clone() } else {
            format!("{:.2}", self.value)
        }
    }
}

/// Specifies how the intercept point is defined for each axis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisIntercept {
    /// This axix is attached to the start of the other axis
    AutoStart,
    /// This axix is attached to the end of the other axis
    AutoEnd,
    /// This axix crosses  the other axis at a given value
    AtValue(f32),
}

/// An axis that maps plot-space values to screen coordinates along a single dimension.
///
/// - For TOP/BOTTOM axes: varies in X, constant in Y
/// - For LEFT/RIGHT axes: varies in Y, constant in X.
#[derive(Debug, Clone)]
pub struct Axis {
    /// Smallest data value plotted by this axis
    pub plot_from: f32,
    /// Largest data value plotted by this axis
    pub plot_to: f32,
    /// How tics should be drawn in respected to their axis
    pub tics_location: TickDirection,
    /// How labels should be drawn in respected to their axis
    pub label_location: TickDirection,
    /// Intercept point - given in the data units of the other axis.
    ///
    /// Note that `intercept` defines the X or Y coordinate for the Y or X axis, respectively
    pub intercept: AxisIntercept,

    // Screen-space interval along the axis direction (SVG/user units).
    screen_from: f32,
    screen_to: f32,

    /// Optional explicit major ticks in plot coordinates;
    plot_ticks: Vec<Tick>,
    /// If ticks are not explicitly set, generate `n_ticks` evenly spaced ticks.
    n_ticks: usize,
}

impl Axis {

    pub fn new(screen_from: f32, screen_to: f32) -> Axis {
        Axis{
            plot_from: 0.0,
            plot_to: 0.0,
            tics_location: TickDirection::DownRight,
            label_location: TickDirection::DownRight,
            intercept: AxisIntercept::AutoStart,
            screen_from,
            screen_to,
            plot_ticks: vec![],
            n_ticks: 0,
        }
    }

    /// Get screen coordinates min/max.
    #[inline]
    pub fn screen_range(&self) -> (f32, f32) { (self.screen_from, self.screen_to) }

    /// Map screen coordinate (along axis direction) back to plot-space value.
    pub fn to_plot(&self, u: f32) -> f32 {
        let denom = self.screen_to - self.screen_from;
        if denom == 0.0 { return self.plot_from; }
        let t = (self.screen_to - u) / denom;
        self.plot_from + t * (self.plot_to - self.plot_from)
    }

    /// Map plot-space value to screen coordinate (along axis direction).
    pub fn to_screen(&self, x: f32, is_y_coordinate: bool) -> f32 {
        if is_y_coordinate {
            let denom = self.plot_to - self.plot_from;
            if denom == 0.0 { return self.screen_from; }
            let t = (x - self.plot_from) / denom;
            self.screen_to - t * (self.screen_to - self.screen_from)
        } else {
            let denom = self.plot_to - self.plot_from;
            if denom == 0.0 { return self.screen_from; }
            let t = (x - self.plot_from) / denom;
            self.screen_from + t * (self.screen_to - self.screen_from)
        }
    }

    pub fn set_nticks(&mut self, n: usize) {
        self.n_ticks = n;
        self.update_ticks();
    }

    /// Update a vector of ticks for this axis if needed.
    pub(crate) fn update_ticks(&mut self) {

        if self.plot_ticks.len()==0 && self.n_ticks > 0 {
            for v in linspace(self.n_ticks, self.plot_from, self.plot_to, true) {
                self.plot_ticks.push(Tick::with_label(v, format!("{:.2}", v)));
            }
        }
    }
}

/// Compute a "nice" plotting range that encloses [min, max].
///
/// Example:
///   [0.023, 1.899999] -> [0.0, 2.0]
///   [12.1, 98.7]      -> [10.0, 100.0]
fn nice_plot_range(min: f32, max: f32) -> (f32, f32) {
    assert!(min.is_finite() && max.is_finite());
    assert!(min < max);

    let span = max - min;

    // Order of magnitude of the span
    let exponent = span.abs().log10().floor();
    let base = 10f32.powf(exponent);

    // Normalized span in [1, 10)
    let frac = span / base;

    // Choose a nice multiplier
    let nice_frac = if frac <= 1.0 {
        1.0
    } else if frac <= 2.0 {
        2.0
    } else if frac <= 5.0 {
        5.0
    } else {
        10.0
    };

    let nice_step = nice_frac * base;

    let nice_min = (min / nice_step).floor() * nice_step;
    let nice_max = (max / nice_step).ceil() * nice_step;

    (nice_min, nice_max)
}


/// Coordinate system defined by two axes.
///
/// # Example
/// ```
/// use visualife::plots::AxisSet;
/// let plot_screen_area = (50.0, 350.0, 50.0, 250.0).into();
/// let mut axes = AxisSet::new(plot_screen_area);
/// axes.set_plot_box((-1.0, 1.0, -1.0, 1.0).into());
/// axes.set_intercept_point(0.0, 0.0);
///
/// ```
#[derive(Debug, Clone)]
pub struct AxisSet {
    pub x: Axis,
    pub y: Axis,
    /// whether the axes end with an arrowhead
    pub has_arrowhead: bool,
    /// stroke width used to draw both axes and (if used) their tics
    pub stroke_width: f32,
    /// tics width
    pub tics_width: f32,
    /// font size for tics labels
    pub font_size: f32,
    pub arrowhead_size: f32,
}

impl AxisSet {
    
    pub fn new<R: Into<Box2D<f32>>>(screen_box: R) -> AxisSet {
        let screen_box = screen_box.into();
        let shortest_len = screen_box.width().min(screen_box.height());
        AxisSet {
            x: Axis::new(screen_box.x_beg, screen_box.x_end),
            y: Axis::new(screen_box.y_beg, screen_box.y_end),
            has_arrowhead: false,
            stroke_width: shortest_len/400.0,
            tics_width: shortest_len/100.0,
            font_size: shortest_len/40.0,
            arrowhead_size: shortest_len/40.0,
        }
    }
    
    /// Return immutable reference to the X axis
    pub fn x_axis(&self) -> &Axis { &self.x }

    /// Return immutable reference to the Y axis
    pub fn y_axis(&self) -> &Axis { &self.y }

    /// Return mutable reference to the X axis
    pub fn x_axis_mut(&mut self) -> &Axis { &mut self.x }

    /// Return immutable reference to the Y axis
    pub fn y_axis_mut(&mut self) -> &Axis { &mut self.y }


    /// Screen coordinates of a rectangle that contains the plotting area.
    ///
    /// The rectangle includes also the ticks if they point inwards
    pub fn screen_box(&self) -> Box2D<f32> {
        Box2D{
            x_beg: self.x.screen_from,
            x_end: self.x.screen_to,
            y_beg: self.y.screen_from,
            y_end: self.y.screen_to
        }
    }

    /// Plot coordinates range as defined by the primary axes
    pub fn plot_box(&self) -> Box2D<f32> {
        Box2D{
            x_beg: self.x.plot_from,
            x_end: self.x.plot_to,
            y_beg: self.y.plot_from,
            y_end: self.y.plot_to
        }
    }

    /// Set the coordinates range for this plot / data
    pub fn set_plot_box<R: Into<Box2D<f32>>>(&mut self, range: R)  {
        let range = range.into();
        self.x.plot_from = range.x_beg;
        self.x.plot_to = range.x_end;

        self.y.plot_from = range.y_beg;
        self.y.plot_to = range.y_end;

        self.x.update_ticks();
        self.y.update_ticks();
    }

    /// Returns the X coordinate of the intercept point in data/plot coordinates
    pub fn intercept_x(&self) -> f32 {
        match self.x.intercept {
            AxisIntercept::AutoStart => {self.y.plot_from}
            AxisIntercept::AutoEnd => {self.y.plot_to}
            AxisIntercept::AtValue(v) => {v}
        }
    }

    /// Returns the Y coordinate of the intercept point in data/plot coordinates
    pub fn intercept_y(&self) -> f32 {
        match self.y.intercept {
            AxisIntercept::AutoStart => {self.x.plot_from}
            AxisIntercept::AutoEnd => {self.x.plot_to}
            AxisIntercept::AtValue(v) => {v}
        }
    }

    pub fn set_intercept_point(&mut self, cx: f32, cy: f32) {
        self.x.intercept = AxisIntercept::AtValue(cy);
        self.y.intercept = AxisIntercept::AtValue(cx);
    }

    pub fn set_intercept(&mut self, cx: AxisIntercept, cy: AxisIntercept) {
        self.x.intercept = cy;
        self.y.intercept = cx;
    }

    /// Convert screen coordinates to plot/data coordinates.
    pub fn to_plot(&self, x: f32, y: f32) -> (f32, f32) {
        let px = self.x.to_plot(x);
        let py = self.y.to_plot(y);
        (px, py)
    }

    /// Convert plot/data coordinates to screen coordinates.
    pub fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let sx = self.x.to_screen(x, false);
        let sy = self.y.to_screen(y, true);
        (sx, sy)
    }

    /// Creates an SVG group element that contains all graphical components representing all axes in this set
    pub fn create_element(&self) ->SvgElement {
        let arrow_w = self.arrowhead_size * 0.5;
        let arrow_l = self.arrowhead_size;
        let mut axes = vec![];
        // ---------- X axis
        let mut x_axis = vec![];
        let y = self.y.to_screen(self.intercept_y(), true);
        let (x1, x2) = (self.x.screen_from, self.x.screen_to);
        x_axis.push(SvgElement::line("x0l", x1, y, x2, y));
        x_axis.push(triangle_arrow("x0a", x2, y, x2 + arrow_l, y, arrow_w));
        // tics and labels
        if self.x.tics_location!=TickDirection::None {
            x_axis.push(self.create_x_tics());
        }
        axes.push( SvgElement::group("x0", x_axis));

        // ---------- Y axis
        let mut y_axis = vec![];
        let x = self.x.to_screen(self.intercept_x(), false);
        // --- Swap Y coordinates!
        let (y1, y2) = (self.y.screen_to, self.y.screen_from);
        y_axis.push(SvgElement::line("y0l", x, y1, x, y2));
        y_axis.push(triangle_arrow("y0a", x, y2 + arrow_l, x, y2, arrow_w));
        // tics and labels here
        if self.y.tics_location!=TickDirection::None {
            y_axis.push(self.create_y_tics());
        }
        axes.push(SvgElement::group("y0", y_axis));

        return SvgElement::group("axes", axes).with_style(
            Style::new().stroke_width(self.stroke_width).stroke("#000000").fill("#000000")
        );
    }

    fn create_x_tics(&self) -> SvgElement {
        // ---------- Prepare ticks
        let tics = &self.x.plot_ticks;

        // ---------- Create SVG elements
        let mut elements = vec![];
        let d = if (self.x.tics_location == TickDirection::DownRight) { self.tics_width } else { -self.tics_width };
        let y = self.y.to_screen(self.intercept_y(), true);
        let text_style;
        let label_offset;
        if self.x.label_location == TickDirection::DownRight {
            text_style = Style::new().text_anchor("middle").dominant_baseline("hanging");
            label_offset = if self.x.tics_location == TickDirection::DownRight {
                self.tics_width + self.font_size * 0.5
            } else {
                self.font_size * 0.5
            };
        } else {
            text_style = Style::new().text_anchor("middle").dominant_baseline("text-bottom");
            if self.x.label_location == TickDirection::DownRight {
                label_offset = -self.font_size * 0.5;
            } else {
                label_offset = -self.tics_width -self.font_size * 0.5;
            }
        };
        for (i, t) in tics.iter().enumerate() {
            let x = self.x.to_screen(t.value, false);
            elements.push(SvgElement::line(format!("x0t{}", i), x, y, x, y+d));
            elements.push(
                SvgElement::text("x0l", x, y + label_offset, t.label())
                    .with_style(text_style.clone().font_size(&format!("{}",self.font_size)))
            );
        }

        return SvgElement::group(format!("x0t"), elements);
    }

    fn create_y_tics(&self) -> SvgElement {
        // ---------- Prepare ticks
        let tics = &self.y.plot_ticks;

        // ---------- Create SVG elements
        let mut elements = vec![];
        let d = if (self.y.tics_location == TickDirection::DownRight) { self.tics_width } else { -self.tics_width };
        let x = self.x.to_screen(self.intercept_x(), false);
        let text_style;
        let label_offset;

        if self.y.label_location == TickDirection::DownRight {
            text_style = Style::new().text_anchor("start").dominant_baseline("middle");
            label_offset = if self.y.label_location == TickDirection::DownRight {
                self.tics_width + self.font_size * 0.5
            } else {
                self.font_size * 0.5
            }
        } else {
            text_style = Style::new().text_anchor("end").dominant_baseline("middle");
            label_offset = if self.y.label_location == TickDirection::DownRight {
                - self.font_size * 0.5
            } else {
                -self.tics_width - self.font_size * 0.5
            }
        }

        for (i, t) in tics.iter().enumerate() {
            let y = self.y.to_screen(t.value, true);
            elements.push(SvgElement::line(format!("y0t{}", i), x, y, x+d, y));
            elements.push(
                SvgElement::text(format!("y0l{}", i), x+label_offset, y, t.label())
                    .with_style(text_style.clone().font_size(&format!("{}",self.font_size)))
            );
        }
        
        return SvgElement::group("y0t", elements).with_style(
            Style::new().font_family(PLOT_FONT_FAMILY).font_weight(PLOT_FONT_WEIGHT));
    }
}


impl Into<SvgElement> for AxisSet {
    /// [`AxisSet`] can be turned into [`SvgElement`] which facilitates their insertion into a drawing.
    ///
    /// # Example
    /// ```
    /// use visualife::plots::AxisSet;
    /// use visualife::SvgDrawing;
    /// let mut axis = AxisSet::new((25.0, 225.0, 25.0, 225.0));
    /// axis.set_plot_box((-1.0, 1.0, -1.0, 1.0));
    /// axis.set_intercept_point(0.0, 0.0);
    /// let mut drawing = SvgDrawing::new(250.0, 250.0);
    /// drawing.add_element(axis);  // --- Here is where we actually use the Into<SvgElement>
    /// ```
    fn into(self) -> SvgElement { self.create_element() }
}


