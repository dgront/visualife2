use std::ops::Sub;

/// A simple 2D bounding box / range container.
///
/// Note: `x_beg` may be greater than `x_end` (same for Y) to represent inverted axes.
/// Use `width()` / `height()` to obtain non-negative extents.
///
/// # Examples
///
/// Construct directly:
/// ```
/// use visualife::plots::Box2D;
/// let b = Box2D { x_beg: 0.0_f32, x_end: 10.0, y_beg: -2.0, y_end: 3.0 };
/// assert_eq!(b.x_beg, 0.0);
/// assert_eq!(b.y_end, 3.0);
/// ```
///
/// A [`Box2D`] can also be conveniently created from a tuple of four:
/// ```
/// use visualife::plots::Box2D;
/// let b: Box2D<f32> = (0.0, 10.0, -2.0, 3.0).into();
/// assert_eq!(b.width(), 10.0);
/// assert_eq!(b.height(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2D<T> {
    pub x_beg: T,
    pub x_end: T,
    pub y_beg: T,
    pub y_end: T,
}

impl<T> Box2D<T>
where
    T: Copy + PartialOrd + Sub<Output = T>
{
    /// Width of the [`Box2D`]
    #[inline]
    pub fn width(&self) -> T {
        if self.x_end >= self.x_beg {
            self.x_end - self.x_beg
        } else {
            self.x_beg - self.x_end
        }
    }

    /// Height of the [`Box2D`]
    #[inline]
    pub fn height(&self) -> T {
        if self.y_end >= self.y_beg {
            self.y_end - self.y_beg
        } else {
            self.y_beg - self.y_end
        }
    }
}

impl Box2D<f32> {
    /// Length of the shorter side of the [`Box2D`]
    #[inline]
    pub fn shortest_length(&self) -> f32 { self.width().min(self.height()) }
}

impl Box2D<f64> {
    /// Length of the shorter side of the [`Box2D`]
    #[inline]
    pub fn shortest_length(&self) -> f64 { self.width().min(self.height()) }
}

impl From<(f32, f32, f32, f32)> for Box2D<f32> {
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self {
            x_beg: t.0,
            x_end: t.1,
            y_beg: t.2,
            y_end: t.3,
        }
    }
}

/// Invert the range so the left side is smaller than the right side.
fn normalize_pair(a: f32, b: f32) -> (f32, f32, bool) {
    if a <= b {
        (a, b, false) // (min, max, not inverted)
    } else {
        (b, a, true)  // (min, max, inverted)
    }
}

pub fn point_outside_box(x: &[f32], y: &[f32], plot_box: &Box2D<f32>) -> bool {

    let (xmin, xmax, _flag) = normalize_pair(plot_box.x_beg, plot_box.x_end);
    let (ymin, ymax, _flag) = normalize_pair(plot_box.y_beg, plot_box.y_end);

    x.iter()
        .zip(y.iter())
        .any(|(&xi, &yi)| xi < xmin || xi > xmax || yi < ymin || yi > ymax)
}

pub fn update_plot_box(x: &[f32], y: &[f32], plot_box: &Box2D<f32>) -> Box2D<f32> {

    let (mut xmin, mut xmax, x_inv) = normalize_pair(plot_box.x_beg, plot_box.x_end);
    let (mut ymin, mut ymax, y_inv) = normalize_pair(plot_box.y_beg, plot_box.y_end);

    for (&xi, &yi) in x.iter().zip(y.iter()) {
        if xi < xmin { xmin = xi; }
        if xi > xmax { xmax = xi; }
        if yi < ymin { ymin = yi; }
        if yi > ymax { ymax = yi; }
    }

    let (x_beg, x_end) = restore_pair(xmin, xmax, x_inv);
    let (y_beg, y_end) = restore_pair(ymin, ymax, y_inv);

    Box2D { x_beg, x_end, y_beg, y_end }
}

/// Restore the original order of a data range
///
/// This function reverts the change introduced by [`normalize_pair()`].
/// If the `inverted` flag is `false`, the order is not changed.
fn restore_pair(min: f32, max: f32, inverted: bool) -> (f32, f32) {
    if inverted {
        (max, min)
    } else {
        (min, max)
    }
}
