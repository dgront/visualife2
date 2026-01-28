use std::fmt;
use std::fmt::Display;

use crate::basic_shapes::SvgElement;
use crate::plots::box2d::Box2D;
use crate::plots::{linspace, PlotError};
use crate::plots::PlotError::NoAxisDefined;
use crate::styling::Style;

/// Defines which side of the plot rectangle the axis is drawn on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisSide {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

impl Display for AxisSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AxisSide::TOP => write!(f, "xt"),
            AxisSide::BOTTOM => write!(f, "xb"),
            AxisSide::LEFT => write!(f, "yl"),
            AxisSide::RIGHT => write!(f, "yr")
        }
    }
}

/// Specifies where tick marks are drawn relative to the plot area
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TicsLocation {
    /// Tick marks extend into the plot area.
    INNER,
    /// Tick marks extend away from the plot area.
    OUTER,
    /// No tick marks are drawn.
    NONE,
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

/// An axis that maps plot-space values to screen coordinates along a single dimension.
///
/// - For TOP/BOTTOM axes: varies in X, constant in Y
/// - For LEFT/RIGHT axes: varies in Y, constant in X.
#[derive(Debug, Clone)]
pub struct Axis {
    side: AxisSide,
    // intercept for this axis is the point of the other axis where the two lines meet;
    // it is also the other (constant) coordinate to draw the line of this axis
    intercept: f32,
    // Screen-space interval along the axis direction (SVG/user units).
    // For TOP/BOTTOM: x0..x1; for LEFT/RIGHT: y0..y1.
    screen_from: f32,
    screen_to: f32,

    /// Plot-space domain (data values).
    plot_from: f32,
    plot_to: f32,

    /// Optional explicit major ticks in plot coordinates;
    plot_ticks: Vec<Tick>,
    /// If ticks are not explicitly set, generate `n_ticks` evenly spaced ticks.
    n_ticks: usize,
    /// How tics should be drawn
    tics_location: TicsLocation,

    /// whether the axis ends with an arrowhead
    has_arrowhead: bool,
    /// drawing parameters
    stroke_width: f32,
    tics_width: f32,
    font_size: f32
}

impl Axis {

    /// Set plot domain (data min/max).
    pub fn set_plot_range(&mut self, pmin: f32, pmax: f32) {
        self.plot_from = pmin;
        self.plot_to = pmax;
    }

    /// Location of this axis within their plot
    #[inline]
    pub fn side(&self) -> AxisSide { self.side }

    /// Position on the other axis this axes intersects
    #[inline]
    pub fn intercept(&self) -> f32 { self.intercept }

    /// Get plot-space min/max.
    #[inline]
    pub fn plot_range(&self) -> (f32, f32) { (self.plot_from, self.plot_to) }

    /// Get screen coordinates min/max.
    #[inline]
    pub fn screen_range(&self) -> (f32, f32) { (self.plot_from, self.plot_to) }

    /// Map screen coordinate (along axis direction) back to plot-space value.
    pub fn to_plot(&self, u: f32) -> f32 {
        let denom = self.screen_to - self.screen_from;
        if denom == 0.0 { return self.plot_from; }
        let t = match self.side {
            AxisSide::LEFT | AxisSide::RIGHT => {
                // invert screen direction for Y axis
                (self.screen_to - u) / denom
            }
            _ => {
                (u - self.screen_from) / denom
            }
        };
        self.plot_from + t * (self.plot_to - self.plot_from)
    }

    /// Map plot-space value to screen coordinate (along axis direction).
    pub fn to_screen(&self, x: f32) -> f32 {
        let denom = self.plot_to - self.plot_from;
        if denom == 0.0 { return self.screen_from; }
        let t = (x - self.plot_from) / denom;
        match self.side {
            AxisSide::LEFT | AxisSide::RIGHT => {
                // invert mapping for Y axis
                self.screen_to - t * (self.screen_to - self.screen_from)
            }
            _ => {
                self.screen_from + t * (self.screen_to - self.screen_from)
            }
        }
    }

    /// Creates an SVG group element that contains all graphical components representing this axis
    pub fn create_element(&self) -> SvgElement {

        // ---------- Group of lines
        let mut lines = vec![];

        // --- Axis spine
        let (x1, y1, x2, y2) = match self.side {
            AxisSide::TOP | AxisSide::BOTTOM => (self.screen_from, 0.0, self.screen_to, 0.0),
            AxisSide::LEFT | AxisSide::RIGHT => (0.0, self.screen_from, 0.0, self.screen_to),
        };
        lines.push(SvgElement::line(format!("{}l", self.side), x1, y1, x2, y2));
        if self.has_arrowhead {
            lines.push(self.create_arrowhead());
        }
        if self.tics_location != TicsLocation::NONE {
            lines.push(self.create_tics());
        }
        let axis_grp = SvgElement::group(format!("{}", self.side), lines).with_style(
            Style::new().stroke("#000000").stroke_width(self.stroke_width));

        return axis_grp;
    }

    fn create_tics(&self) -> SvgElement {
        // ---------- Prepare ticks
        let mut tics: Vec<Tick> = vec![];
        if self.plot_ticks.len()==0 && self.n_ticks > 0 {
            for v in linspace(self.n_ticks, self.plot_from, self.plot_to, true) {
                tics.push(Tick::with_label(v, format!("{:.2}", v)));
            }
        } else {
            for t in &self.plot_ticks { tics.push(t.clone()); }
        }

        // ---------- Create SVG elements
        let mut elements = vec![];
        let is_horizontal = matches!(self.side, AxisSide::TOP | AxisSide::BOTTOM);
        let dir = if (self.tics_location == TicsLocation::INNER) { 1.0 } else { -1.0 };

        // ---------- ... ticks lines first
        let sign = match self.side {
            AxisSide::TOP    =>  1.0 * dir,
            AxisSide::BOTTOM => -1.0 * dir,
            AxisSide::LEFT   =>  1.0 * dir,
            AxisSide::RIGHT  => -1.0 * dir,
        };
        for (i, t) in tics.iter().enumerate() {
            let ts = self.to_screen(t.value);
            let d  = self.tics_width * sign;

            let (x1, y1, x2, y2) = if is_horizontal {
                (ts, 0.0, ts, d)    // ticks along X axis
            } else {
                (0.0, ts, d, ts)    // ticks along Y axis
            };

            elements.push(SvgElement::line(format!("{}t{}", self.side, i), x1, y1, x2, y2));
        }

        // ---------- ... now ticks labels (text)
        let offset = self.tics_width + self.font_size * 0.5;  // distance from axis to label baseline

        let text_style = match self.side {
            AxisSide::TOP => Style::new().text_anchor("middle").dominant_baseline("text-bottom"),
            AxisSide::BOTTOM => Style::new().text_anchor("middle").dominant_baseline("hanging"),
            AxisSide::LEFT => Style::new().text_anchor("end").dominant_baseline("middle"),
            AxisSide::RIGHT => Style::new().text_anchor("start").dominant_baseline("middle"),
        };
        for (i, t) in tics.iter().enumerate() {
            let ts = self.to_screen(t.value);

            let (lab_x, lab_y) = match self.side {
                AxisSide::BOTTOM => (ts, -offset * dir),
                AxisSide::TOP    => (ts,  offset * dir),
                AxisSide::LEFT   => (offset * dir, ts),
                AxisSide::RIGHT  => (-offset * dir, ts),
            };
            elements.push(
                SvgElement::text(format!("{}l{}", self.side, i), lab_x, lab_y, t.label())
                    .with_style(text_style.clone().font_size(&format!("{}",self.font_size)))
            );
        }

        return SvgElement::group(format!("{}tics", self.side), elements);
    }

    fn create_arrowhead(&self) -> SvgElement {
        let arrow_len = 2.0;
        let arrow_w = 1.0;
        let mut arrow_tip = vec![];

        match self.side {
            AxisSide::BOTTOM | AxisSide::TOP => {
                // Horizontal axis; arrow points toward increasing screen coordinate from -> to
                let dir = (self.screen_to - self.screen_from).signum().max(1.0); // fallback
                let tip_x = self.screen_to;
                let tip_y = self.intercept;
                // Triangle base below the tip (arrow points right)
                let bt_x = tip_x - arrow_len * dir;
                let bt_y = tip_y - arrow_w;
                let bb_x = tip_x - arrow_len * dir;
                let bb_y = tip_y + arrow_w;
                let mut tri = SvgElement::polygon(
                    format!("{}a", self.side),
                    vec![(tip_x, tip_y), (bt_x, bt_y), (bb_x, bb_y)],
                ).with_style(Style::new().fill("#000000"));
                arrow_tip.push(tri);

            }
            AxisSide::LEFT | AxisSide::RIGHT => {
                // Vertical axis; arrow points toward increasing screen coordinate to -> from because Y axis is inverted in SVG
                let dir = (self.screen_from - self.screen_to).signum().max(1.0); // fallback
                let tip_x = self.intercept;
                let tip_y = self.screen_from;
                // Triangle base below the tip (arrow points up)
                let bl_x = tip_x - arrow_w;
                let bl_y = tip_y + arrow_len * dir;
                let br_x = tip_x + arrow_w;
                let br_y = tip_y + arrow_len * dir;
                let tri = SvgElement::polygon(
                    format!("{}a", self.side),
                    vec![(tip_x, tip_y), (bl_x, bl_y), (br_x, br_y)],
                ).with_style(Style::new().fill("#000000"));
                arrow_tip.push(tri);
            }
        }
        return SvgElement::group(format!("{}t", self.side), arrow_tip);
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
impl Into<SvgElement> for Axis {
    /// [`Axis`] can be turned into [`SvgElement`] which facilitates their insertion into a drawing.
    ///
    /// # Example
    /// ```
    /// use visualife::plots::{AxisBuilder, AxisSide};
    /// use visualife::SvgDrawing;
    /// let axis = AxisBuilder::new(AxisSide::BOTTOM, 50.0, 150.0).n_tics(8).arrowhead(true).build();
    /// let mut drawing = SvgDrawing::new(200.0, 50.0);
    /// drawing.add_element(axis);  // --- Here is where we actually use the Into<SvgElement>
    /// ```
    fn into(self) -> SvgElement { self.create_element() }
}

#[derive(Debug, Clone)]
/// Builds an axis and sets its all optional properties.
pub struct AxisBuilder { axis: Axis, }


impl AxisBuilder {
    /// Create a new axis with mandatory screen-space information only.
    ///
    /// Plot range defaults to [0, 1] and can be overridden explicitly
    /// or inferred from data via the builder.
    pub fn new(side: AxisSide, screen_from: f32, screen_to: f32) -> AxisBuilder {
        // stroke width is also a unit for other properties such as tics width
        let w = (screen_to-screen_from).abs() / 400.0;
        AxisBuilder {
            axis: Axis {
                side,
                intercept: 0.0,
                screen_from,
                screen_to,
                plot_from: 0.0,
                plot_to: 1.0,
                plot_ticks: Vec::new(),
                n_ticks: 5, // reasonable default
                tics_location: TicsLocation::OUTER,
                has_arrowhead: false,
                stroke_width: w,
                tics_width: w * 10.0,
                font_size: 10.0,
            },
        }
    }

    /// Explicitly set the plot (data) range.
    pub fn plot_range(mut self, from: f32, to: f32) -> Self {
        self.axis.plot_from = from;
        self.axis.plot_to = to;
        self
    }

    /// Infer plot range from data.
    ///
    /// Ignores NaN values. If no finite values are found,
    /// the existing plot range is left unchanged.
    pub fn plot_range_from_data<D: AsRef<[f32]>>(mut self, data: D) -> Self {
        let slice = data.as_ref();
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;

        for &v in slice {
            if v.is_finite() {
                min = min.min(v);
                max = max.max(v);
            }
        }

        if min.is_finite() && max.is_finite() && min != max {
            (min, max) = nice_plot_range(min, max);
            self.axis.plot_from = min;
            self.axis.plot_to = max;
        }

        self
    }

    /// Sets the axis intercept value in plot (data) coordinates.
    ///
    /// The intercept defines where this axis crosses the orthogonal axis.
    pub fn intercept(mut self, value: f32) -> Self {
        self.axis.intercept = value;
        self
    }

    /// Set the number of automatically generated major ticks.
    pub fn n_tics(mut self, n: usize) -> Self {
        self.axis.n_ticks = n.max(2);
        self
    }

    /// Provide explicit major ticks in plot coordinates.
    pub fn tics_at_values(mut self, tics: &[f32]) -> Self {
        self.axis.plot_ticks.clear();
        self.axis.plot_ticks.extend(tics.iter().map(|v| Tick::new(*v)));
        self
    }

    /// Set how ticks are drawn relative to the axis spine.
    pub fn tics_location(mut self, loc: TicsLocation) -> Self {
        self.axis.tics_location = loc;
        self
    }

    /// Enable or disable an arrowhead at the end of the axis.
    pub fn arrowhead(mut self, enabled: bool) -> Self {
        self.axis.has_arrowhead = enabled;
        self
    }

    /// Sets the stroke width used to draw the axis line and ticks.
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.axis.stroke_width = width;
        self
    }

    /// Sets the font size used for tick labels on this axis.
    pub fn font_size(mut self, size: f32) -> Self {
        self.axis.font_size = size;
        self
    }

    /// Sets the length of tick marks extending from the axis line.
    pub fn tics_width(mut self, width: f32) -> Self {
        self.axis.tics_width = width;
        self
    }

    /// Finalize and return the Axis.
    pub fn build(self) -> Axis { self.axis }
}


/// A collection of axes forming the coordinate frame of a plot.
///
/// An `AxisSet` groups together one or more [`Axis`] objects
/// (e.g. bottom/left, all four sides, dual axes) and is responsible for:
///
/// - defining the plot rectangle in screen space,
/// - managing axis placement and crossings,
/// - converting between plot (data) and screen coordinates,
/// - rendering axes, ticks, labels, and grid lines.
///
/// It is typically created via [`AxisSetBuilder`] and then used by [`Plot`]
/// to draw data series within the defined coordinate system.
#[derive(Debug, Clone)]
pub struct AxisSet {
    axes: Vec<Axis>
}

impl AxisSet {
    /// Return immutable reference to axis on a given side.
    pub fn axis(&self, side: AxisSide) -> Option<&Axis> {
        self.axes.iter().find(|a| a.side == side)
    }

    /// Return mutable reference to axis on a given side.
    pub fn axis_mut(&mut self, side: AxisSide) -> Option<&mut Axis> {
        self.axes.iter_mut().find(|a| a.side == side)
    }

    /// Returns the primary X axis which is the [`AxisSide::BOTTOM`] one
    ///
    /// If not found, [`AxisSide::TOP`] axis is returned
    pub fn primary_x_axis(&self) -> AxisSide {

        if let Some(axis) = self.axis(AxisSide::BOTTOM) {
            return AxisSide::BOTTOM;
        } else {
            return AxisSide::TOP
        }
    }

    /// Returns the primary Y axis which is the [`AxisSide::LEFT`] one
    ///
    /// If not found, [`AxisSide::RIGHT`] axis is returned
    pub fn primary_y_axis(&self) -> AxisSide  {

        if let Some(axis) = self.axis(AxisSide::LEFT) {
            return AxisSide::LEFT;
        } else {
            return AxisSide::TOP
        }
    }

    /// Screen coordinates of a rectangle that contains the plotting area.
    ///
    /// The rectangle includes also the ticks if they point inwards
    pub fn screen_box(&self) -> Result<Box2D<f32>,PlotError> {
        Ok(Box2D{
            x_beg: self.primary_x_axis()?.screen_from,
            x_end: self.primary_x_axis()?.screen_to,
            y_beg: self.primary_y_axis()?.screen_from,
            y_end: self.primary_y_axis()?.screen_to
        })
    }

    /// Plot coordinates range as defined by the primary axes
    pub fn plot_box(&self) -> Result<Box2D<f32>,PlotError> {
        Ok(Box2D{
            x_beg: self.primary_x_axis()?.plot_from,
            x_end: self.primary_x_axis()?.plot_to,
            y_beg: self.primary_y_axis()?.plot_from,
            y_end: self.primary_y_axis()?.plot_to
        })
    }

    /// Set the plot coordinates range as defined by the primary axes
    pub fn set_plot_box(&mut self, range: &Box2D<f32>) -> Result<(),PlotError> {
        let primary_x = self.primary_x_axis()?.side;
        self.axis_mut(primary_x).ok_or(NoAxisDefined)?.set_plot_range(range.x_beg, range.x_end);
        let primary_y = self.primary_y_axis()?.side;
        self.axis_mut(primary_y).ok_or(NoAxisDefined)?.set_plot_range(range.y_beg, range.y_end);
        Ok(())
    }

    /// Choose which axes act as X and Y for coordinate transforms.
    /// Preference:
    ///   X: BOTTOM -> TOP
    ///   Y: LEFT   -> RIGHT
    fn resolve_xy_axes(&self) -> Option<(&Axis, &Axis)> {
        let x_axis = self.axis(AxisSide::BOTTOM)
            .or_else(|| self.axis(AxisSide::TOP));

        let y_axis = self.axis(AxisSide::LEFT)
            .or_else(|| self.axis(AxisSide::RIGHT));

        match (x_axis, y_axis) {
            (Some(xa), Some(ya)) => Some((xa, ya)),
            _ => None,
        }
    }

    /// Convert screen coordinates to plot/data coordinates.
    ///
    /// Uses [`AxisSide::BOTTOM`] & [`AxisSide::LEFT`] if present, otherwise [`AxisSide::TOP`] & [`AxisSide::RIGHT`].
    pub fn to_plot(&self, x: f32, y: f32) -> (f32, f32) {
        if let Some((x_axis, y_axis)) = self.resolve_xy_axes() {
            let px = x_axis.to_plot(x);
            let py = y_axis.to_plot(y);
            (px, py)
        } else {
            // Fallback: no axes available
            (x, y)
        }
    }

    /// Convert plot/data coordinates to screen coordinates.
    ///
    /// Uses [`AxisSide::BOTTOM`] & [`AxisSide::LEFT`] if present, otherwise [`AxisSide::TOP`] & [`AxisSide::RIGHT`].
    pub fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        if let Some((x_axis, y_axis)) = self.resolve_xy_axes() {
            let sx = x_axis.to_screen(x);
            let sy = y_axis.to_screen(y);
            (sx, sy)
        } else {
            // Fallback: no axes available
            (x, y)
        }
    }

    /// Creates an SVG group element that contains all graphical components representing all axes in this set
    pub fn create_element(&self) ->SvgElement {
        let mut axes = vec![];
        for ax in &self.axes {
            let mut axis = ax.create_element();
            if ax.side==AxisSide::BOTTOM || ax.side==AxisSide::TOP {
                axis.translate(0.0, self.primary_y_axis().expect("No primary Y axis found!").to_screen(ax.intercept));
            } else if ax.side==AxisSide::LEFT || ax.side==AxisSide::RIGHT  {
                axis.translate(self.primary_x_axis().expect("No primary X axis found!").to_screen(ax.intercept), 0.0);
            }
            axes.push(axis);
        }
        return SvgElement::group("axes", axes);
    }
}

/// Builder for constructing an [`AxisSet`] with a chosen layout and defaults.
///
/// [`AxisSetBuilder`] allows selecting which axes are present (e.g. left/bottom,
/// rectangular frame, dual axes) and configuring common properties that will
/// be propagated to all created [`Axis`] objects, such as tick count,
/// tick direction, font size, and arrowheads.
///
/// The builder is typically used indirectly via higher-level plot
/// construction, but it can also be used standalone when axes are needed
/// outside of a full plot.
///
/// # Example
///
/// ```
/// use visualife::plots::{AxisSetBuilder, TicsLocation};
/// let axes = AxisSetBuilder::new("LRTB", (50.0, 250.0, 450.0, 50.0))
///     .ntics(6)
///     .tics_location(TicsLocation::OUTER)
///     .arrowheads(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct AxisSetBuilder {
    screen_box: Box2D<f32>,
    plot_box: Box2D<f32>,
    sides: Vec<AxisSide>,
    xc: f32,
    yc: f32,
    // defaults applied to created axes
    n_ticks: usize,
    tics_location: TicsLocation,
    has_arrowhead: bool,
    font_size: f32,
}

impl AxisSetBuilder {
    /// Start a builder with empty axes drawn over a given rectangle
    ///
    pub fn new<R: Into<Box2D<f32>>>(sides: &str, screen_box: R) -> Self {
        let sides = Self::axes_sides(sides);
        let screen_box = screen_box.into();
        Self {
            screen_box,
            plot_box: Box2D{ x_beg: 0.0, x_end: 1.0, y_beg: 0.0, y_end: 1.0},
            sides,
            xc: 0.0,
            yc: 0.0,
            n_ticks: 5,
            tics_location: TicsLocation::OUTER,
            has_arrowhead: false,
            font_size: screen_box.shortest_length() / 20.0,
        }
    }

    /// Set the default number of major ticks for all axes created by this builder.
    pub fn ntics(mut self, n: usize) -> Self {
        self.n_ticks = n;
        self
    }

    /// Set the default tick location for all axes created by this builder.
    pub fn tics_location(mut self, loc: TicsLocation) -> Self {
        self.tics_location = loc;
        self
    }

    /// Enable/disable arrowheads for axes created by this builder.
    pub fn arrowheads(mut self, enabled: bool) -> Self {
        self.has_arrowhead = enabled;
        self
    }

    /// Set the main font size.
    ///
    /// This size define the size of ticks
    pub fn font_size(mut self, fsize: f32) -> Self {
        self.font_size = fsize;
        self
    }

    /// Define the intersection point where the X and Y axes cross each other
    pub fn center(mut self, xc: f32, yc: f32) -> Self {
        self.xc = xc;
        self.yc = yc;
        self
    }

    /// Define the range of data both on X and Y axis
    pub fn data_range<R: Into<Box2D<f32>>>(mut self, data_box: R) -> Self {
        self.plot_box = data_box.into();
        self
    }

    /// Build the AxisSet and create the requested axes with reasonable defaults.
    ///
    pub fn build(self) -> AxisSet {
        let mut axes: Vec<Axis> = Vec::with_capacity(self.sides.len());

        let shortest_len = self.screen_box.width().min(self.screen_box.height());
        let mut intercept = 0.0;
        for side in &self.sides {
            // Screen range along the varying dimension for the axis.
            let (screen_from, screen_to, plot_from, plot_to) = match side {
                AxisSide::TOP | AxisSide::BOTTOM =>
                    (self.screen_box.x_beg, self.screen_box.x_end, self.plot_box.x_beg, self.plot_box.x_end),
                AxisSide::LEFT | AxisSide::RIGHT =>
                    (self.screen_box.y_beg, self.screen_box.y_end, self.plot_box.y_beg, self.plot_box.y_end),
            };
            if side==&AxisSide::LEFT {
                if !self.sides.contains(&AxisSide::RIGHT) {
                    intercept = self.xc;
                } else {
                    intercept = self.plot_box.x_beg;
                }
            } else if side == &AxisSide::RIGHT {
                if !self.sides.contains(&AxisSide::LEFT) {
                    // only one vertical axis -> cross at center
                    intercept = self.xc;
                } else {
                    // both LEFT and RIGHT -> rectangle mode
                    intercept = self.plot_box.x_end;
                }
            } else if side == &AxisSide::BOTTOM {
                if !self.sides.contains(&AxisSide::TOP) {
                    // only one horizontal axis -> cross at center
                    intercept = self.yc;
                } else {
                    // both BOTTOM and TOP -> rectangle mode
                    intercept = self.plot_box.y_beg;
                }
            } else if side == &AxisSide::TOP {
                if !self.sides.contains(&AxisSide::BOTTOM) {
                    // only one horizontal axis -> cross at center
                    intercept = self.yc;
                } else {
                    // both BOTTOM and TOP -> rectangle mode
                    intercept = self.plot_box.y_end;
                }
            }

            // Create axis with mandatory screen range only; then apply defaults.
            // This assumes you have Axis::new(...) -> AxisBuilder as in your earlier design.
            let axis = AxisBuilder::new(side.clone(), screen_from, screen_to)
                .plot_range(plot_from, plot_to)
                .n_tics(self.n_ticks)
                .intercept(intercept)
                .tics_location(self.tics_location)
                .arrowhead(self.has_arrowhead)
                .stroke_width(shortest_len/400.0)
                .tics_width(shortest_len/40.0)
                .font_size(self.font_size)
                .build();

            axes.push(axis);
        }

        AxisSet { axes }
    }


    /// Decodes a compact string, e.g. "LB", "TLBR" into a set of axes.
    /// Allowed letters: T, L, B, R.
    fn axes_sides(sides_str: &str) -> Vec<AxisSide> {
        let mut sides: Vec<AxisSide> = vec![];
        for ch in sides_str.chars() {
            let side = match ch {
                'T' | 't' => Some(AxisSide::TOP),
                'L' | 'l' => Some(AxisSide::LEFT),
                'B' | 'b' => Some(AxisSide::BOTTOM),
                'R' | 'r' => Some(AxisSide::RIGHT),
                _ => None,
            };
            if let Some(s) = side { sides.push(s) }
        }
        sides
    }
}

impl Into<SvgElement> for AxisSet {
    /// [`AxisSet`] can be turned into [`SvgElement`] which facilitates their insertion into a drawing.
    ///
    /// # Example
    /// ```
    /// use visualife::plots::{AxisSetBuilder, AxisSide};
    /// use visualife::SvgDrawing;
    /// let axis = AxisSetBuilder::new("BL", (25.0, 225.0, 25.0, 225.0)).center(0.0, 0.0).build();
    /// let mut drawing = SvgDrawing::new(250.0, 250.0);
    /// drawing.add_element(axis);  // --- Here is where we actually use the Into<SvgElement>
    /// ```
    fn into(self) -> SvgElement { self.create_element() }
}
