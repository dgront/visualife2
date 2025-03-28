use crate::styling::mix_colors;

const K: usize = 128;

/// A color interpolation map for mapping scalar values to colors.
///
/// `ColorMap` takes a palette of input colors (e.g., `["#ff0000", "#0000ff"]`) and
/// either a set of associated scalar stops or a continuous range. It precomputes
/// a fixed number (`K = 128`) of interpolated colors, which can be queried efficiently.
///
/// ### Construction
///
/// - `ColorMap::from_range(colors, from, to)`
/// - `ColorMap::from_stops(colors, stops)`
///
/// ### Example
///
/// ```
/// use visualife::styling::ColorMap;
/// let colors = ["#67001f", "#730421", "#7e0823", "#8a0c25"];
/// let cmap = ColorMap::from_range(&colors, 0.0, 1.0).unwrap();
///
/// let c = cmap.color(0.5);
/// assert!(c.starts_with('#'));
/// ```
///
/// ### Notes
/// - The `color(x)` function clamps `x` to the range [from, to].
/// - Interpolation is linear in RGB space using `mix_colors(...)`.
/// - Colors are returned as hex strings (e.g., "#8a0c25").
pub struct ColorMap {
    colors: Vec<String>, // Precomputed K interpolated colors
    from: f64,
    to: f64,
}

impl ColorMap {
    /// Constructs from explicit stops
    pub fn from_stops(colors: &[&str], stops: &[f64]) -> Result<Self, &'static str> {
        if colors.len() != stops.len() || colors.len() < 2 {
            return Err("colors and stops must have the same length and at least 2 points");
        }

        let from = *stops.first().unwrap();
        let to = *stops.last().unwrap();

        let mut result = Vec::with_capacity(K);
        for i in 0..K {
            let t = from + ((i as f64) / (K - 1) as f64) * (to - from);
            result.push(Self::interpolate(t, colors, stops)?);
        }

        Ok(Self { colors: result, from, to })
    }

    /// Constructs from a scalar range
    pub fn from_range(colors: &[&str], from: f64, to: f64) -> Result<Self, &'static str> {
        let n = colors.len();
        let mut stops = Vec::with_capacity(n);
        for i in 0..n {
            stops.push(from + (i as f64) * (to - from) / (n - 1) as f64);
        }
        Self::from_stops(colors, &stops)
    }

    /// Returns the color closest to `x`
    pub fn color(&self, x: f64) -> &str {
        let t = ((x - self.from) / (self.to - self.from)).clamp(0.0, 1.0);
        let i = (t * (K as f64 - 1.0)).round() as usize;
        &self.colors[i]
    }

    /// Internal: interpolates a color at position `x`
    fn interpolate(x: f64, colors: &[&str], stops: &[f64]) -> Result<String, &'static str> {
        for i in 0..stops.len() - 1 {
            let (x0, x1) = (stops[i], stops[i + 1]);
            if x0 <= x && x <= x1 {
                let f = (x - x0) / (x1 - x0);
                return mix_colors(colors[i], colors[i + 1], (1.0 - f) as f32);
            }
        }
        // Handle out-of-bound edges
        if x <= stops[0] {
            Ok(colors[0].to_string())
        } else if x >= stops[stops.len() - 1] {
            Ok(colors[colors.len() - 1].to_string())
        } else {
            Err("Failed to interpolate color")
        }
    }
}
