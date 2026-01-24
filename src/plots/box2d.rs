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

