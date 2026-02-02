use std::fmt;
use std::str::FromStr;
use crate::basic_shapes::SvgElement;

/// Marker symbols for scatter plots
///
/// Supported markers:
/// - `"o"` / `"c"` : empty circle
/// - `"O"` / `"C"` : filled circle
/// - `"s"` : empty square
/// - `"s"` : filled square
/// - `"."` : point
/// - `"+"` : plus
/// - `"x"` : cross
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Circle,
    FilledCircle,
    Square,
    FilledSquare,
    Point,
    Plus,
    Cross,
    None
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
            _ => { // --- this should never happen: caller should check id the marker is None before calling
                return SvgElement::circle(format!("{id}_c"), cx, cy, 0.0);
            }
        }
    }

    /// Assigns marker type by index
    ///
    /// This is used to automatically assign a marker for a new scatter data series
    pub(crate) fn by_index(index: usize) -> Self {
        match index {
            0 => MarkerType::Circle,
            1 => MarkerType::FilledCircle,
            2 => MarkerType::Square,
            3 => MarkerType::FilledSquare,
            4 => MarkerType::Point,
            5 => MarkerType::Plus,
            6 => MarkerType::Cross,
            _ => MarkerType::Circle
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
            _ => Ok(None),
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
            None => "",
        };

        write!(f, "{}", s)
    }
}

/// Marker symbols for scatter plots
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Solid,
    None
}
