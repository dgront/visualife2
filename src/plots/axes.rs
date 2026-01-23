use std::fmt;
use std::fmt::Display;

use crate::basic_shapes::SvgElement;
use crate::plots::box2d::Box2D;
use crate::plots::linspace;
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
            AxisSide::TOP => write!(f, "x2"),
            AxisSide::BOTTOM => write!(f, "x1"),
            AxisSide::LEFT => write!(f, "y1"),
            AxisSide::RIGHT => write!(f, "y2")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TicsLocation {
    INNER,
    OUTER,
    NONE,
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
    plot_ticks: Vec<f32>,
    /// If ticks are not explicitly set, generate `n_ticks` evenly spaced ticks.
    n_ticks: usize,
    /// How tics should be drawn
    tics_location: TicsLocation,

    /// whether the axis ends with an arrowhead
    has_arrowhead: bool,
    /// drawing parameters
    stroke_width: f32,
    tics_width: f32
}

impl Axis {

    /// Set plot domain (data min/max).
    pub fn set_plot_range(&mut self, pmin: f32, pmax: f32) {
        self.plot_from = pmin;
        self.plot_to = pmax;
    }

    #[inline]
    pub fn side(&self) -> AxisSide { self.side }

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
        let t = (u - self.screen_from) / denom;
        self.plot_from + t * (self.plot_to - self.plot_from)
    }

    /// Map plot-space value to screen coordinate (along axis direction).
    pub fn to_screen(&self, x: f32) -> f32 {
        let denom = self.plot_to - self.plot_from;
        if denom == 0.0 { return self.screen_from; }
        let t = (x - self.plot_from) / denom;
        self.screen_from + t * (self.screen_to - self.screen_from)
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
        let dir = if (self.tics_location == TicsLocation::INNER) { 1.0 } else { -1.0 };
        if self.tics_location != TicsLocation::NONE {
            let mut tics: Vec<f32> = vec![];
            if self.plot_ticks.len()==0 && self.n_ticks > 0 {
                for t in linspace(self.n_ticks, self.plot_from, self.plot_to, true) {
                    tics.push(self.to_screen(t));
                }
            } else {
                for t in &self.plot_ticks {
                    tics.push(self.to_screen(*t));
                }
            }
            match self.side {
                AxisSide::TOP => {
                    for (i,t) in tics.iter().enumerate() {
                        lines.push(SvgElement::line(format!("{}t{}", self.side, i), *t, 0.0, *t, -self.tics_width * dir))
                    }
                }
                AxisSide::BOTTOM => {
                    for (i,t) in tics.iter().enumerate() {
                        lines.push(SvgElement::line(format!("{}t{}", self.side, i), *t, 0.0, *t, self.tics_width * dir))
                    }
                }
                AxisSide::LEFT => {
                    for (i,t) in tics.iter().enumerate() {
                        lines.push(SvgElement::line(format!("{}t{}", self.side, i), 0.0, *t, self.tics_width * dir, *t))
                    }
                }
                AxisSide::RIGHT => {
                    for (i,t) in tics.iter().enumerate() {
                        lines.push(SvgElement::line(format!("{}t{}", self.side, i), 0.0, *t, -self.tics_width * dir, *t))
                    }
                }
            }
        }
        let axis_grp = SvgElement::group(format!("{}", self.side), lines).with_style(
            Style::new().stroke("#000000").stroke_width(self.stroke_width));

        return axis_grp;
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
                let mut tri = SvgElement::polygon(
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

#[derive(Debug, Clone)]
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
                tics_width: w * 10.0
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
    pub fn plot_range_from_data<D>(mut self, data: D) -> Self
    where
        D: AsRef<[f32]>,
    {
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
    pub fn tics(mut self, tics: &[f32]) -> Self {
        self.axis.plot_ticks.clear();
        self.axis.plot_ticks.extend_from_slice(tics);
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

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.axis.stroke_width = width;
        self
    }

    pub fn tics_width(mut self, width: f32) -> Self {
        self.axis.tics_width = width;
        self
    }

    /// Finalize and return the Axis.
    pub fn build(self) -> Axis { self.axis }
}


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
    pub fn primary_x_axis(&self) -> &Axis {

        if let Some(axis) = self.axis(AxisSide::BOTTOM) {
            return axis;
        } else {
            return self.axis(AxisSide::TOP).unwrap();
        }
    }

    /// Returns the primary Y axis which is the [`AxisSide::LEFT`] one
    ///
    /// If not found, [`AxisSide::RIGHT`] axis is returned
    pub fn primary_y_axis(&self) -> &Axis {

        if let Some(axis) = self.axis(AxisSide::LEFT) {
            return axis;
        } else {
            return self.axis(AxisSide::RIGHT).unwrap();
        }
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
                axis.translate(0.0, self.primary_y_axis().to_screen(ax.intercept));
            } else if ax.side==AxisSide::LEFT || ax.side==AxisSide::RIGHT  {
                axis.translate(self.primary_x_axis().to_screen(ax.intercept), 0.0);
            }
            axes.push(axis);
        }
        return SvgElement::group("axes", axes);
    }
}

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
}

impl AxisSetBuilder {
    /// Start a builder with an empty axis set for the given plot rect.
    pub fn new<R: Into<Box2D<f32>>>(screen_box: R) -> Self {
        Self {
            screen_box: screen_box.into(),
            plot_box: Box2D{ x_beg: 0.0, x_end: 1.0, y_beg: 0.0, y_end: 1.0},
            sides: Vec::new(),
            xc: 0.0,
            yc: 0.0,
            n_ticks: 5,
            tics_location: TicsLocation::OUTER,
            has_arrowhead: false,
        }
    }

    /// Add axes from a compact string, e.g. "LB", "TLBR".
    /// Allowed letters: T, L, B, R.
    pub fn axes(mut self, sides: &str) -> Self {
        for ch in sides.chars() {
            let side = match ch {
                'T' | 't' => Some(AxisSide::TOP),
                'L' | 'l' => Some(AxisSide::LEFT),
                'B' | 'b' => Some(AxisSide::BOTTOM),
                'R' | 'r' => Some(AxisSide::RIGHT),
                _ => None,
            };
            if let Some(s) = side { self.sides.push(s) }
        }
        self
    }
    /// Set the default number of major ticks for all axes created by this builder.
    pub fn ntics(mut self, n: usize) -> Self {
        self.n_ticks = n.max(2);
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
                .build();

            axes.push(axis);
        }

        AxisSet { axes }
    }
}

