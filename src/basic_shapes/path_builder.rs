use std::fmt::{Display, Formatter};
use crate::basic_shapes::SvgElement;
use crate::element_id::ElementID;

// #[derive(Debug, Clone, PartialEq)]
enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    HorizontalTo(f32),
    VerticalTo(f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    SmoothCurveTo(f32, f32, f32, f32),
    QuadraticBezierCurveTo(f32, f32, f32, f32),
    SmoothQuadraticBezierCurveTo(f32, f32),
    EllipticalArcTo(f32, f32, f32, bool, bool, f32, f32),
    MoveBy(f32, f32),
    LineBy(f32, f32),
    HorizontalBy(f32),
    VerticalBy(f32),
    CurveBy(f32, f32, f32, f32, f32, f32),
    SmoothCurveBy(f32, f32, f32, f32),
    QuadraticBezierCurveBy(f32, f32, f32, f32),
    EllipticalArcBy(f32, f32, f32, bool, bool, f32, f32),
    Close,
}

impl Display for PathCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathCommand::MoveTo(x, y) => write!(f, "M {} {}", x, y),
            PathCommand::LineTo(x, y) => write!(f, "L {} {}", x, y),
            PathCommand::HorizontalTo(x) => write!(f, "H {}", x),
            PathCommand::VerticalTo(y) => write!(f, "V {}", y),
            PathCommand::CurveTo(x1, y1, x2, y2, x, y) => write!(f, "C {} {} {} {} {} {}", x1, y1, x2, y2, x, y),
            PathCommand::SmoothCurveTo(x2, y2, x, y) => write!(f, "S {} {} {} {}", x2, y2, x, y),
            PathCommand::QuadraticBezierCurveTo(x1, y1, x, y) => write!(f, "Q {} {} {} {}", x1, y1, x, y),
            PathCommand::SmoothQuadraticBezierCurveTo(x, y) => write!(f, "T {} {}", x, y),
            PathCommand::EllipticalArcTo(rx, ry, x_axis_rotation, large_arc_flag , sweep_flag, x, y)
            => write!(f, "A {} {} {} {} {} {} {}", rx, ry, x_axis_rotation, (*large_arc_flag as i32), (*sweep_flag as i32), x, y),
            PathCommand::Close => { write!(f, "Z") },
            PathCommand::MoveBy(x, y) => write!(f, "m {} {}", x, y),
            PathCommand::LineBy(x, y) => write!(f, "l {} {}", x, y),
            PathCommand::HorizontalBy(x) => write!(f, "h {}", x),
            PathCommand::VerticalBy(y) => write!(f, "v {}", y),
            PathCommand::CurveBy(x1, y1, x2, y2, x, y) => write!(f, "c {} {} {} {} {} {}", x1, y1, x2, y2, x, y),
            PathCommand::SmoothCurveBy(x2, y2, x, y) => write!(f, "s {} {} {} {}", x2, y2, x, y),
            PathCommand::QuadraticBezierCurveBy(x1, y1, x, y) => write!(f, "q {} {} {} {}", x1, y1, x, y),
            PathCommand::EllipticalArcBy(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y)
            => write!(f, "a {} {} {} {} {} {} {}", rx, ry, x_axis_rotation, (*large_arc_flag as i32), (*sweep_flag as i32), x, y),
        }
    }
}

/// Builder for constructing an SVG Path.
pub struct PathBuilder {
    id: ElementID,
    commands: Vec<PathCommand>,
}

impl PathBuilder {
    /// Creates a new, empty path.
    pub fn new(id: ElementID) -> Self {
        Self { id, commands: Vec::new() }
    }

    /// Adds a move-to command (`M x y`).
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(x, y));
        self
    }

    /// Adds a line-to command (`L x y`).
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::LineTo(x, y));
        self
    }

    /// Adds a horizontal line-to command (`H x`).
    pub fn horizontal_to(mut self, x: f32) -> Self {
        self.commands.push(PathCommand::HorizontalTo(x));
        self
    }

    /// Adds a vertical line-to command (`V y`).
    pub fn vertical_to(mut self, y: f32) -> Self {
        self.commands.push(PathCommand::VerticalTo(y));
        self
    }

    /// Adds a curve-to command (`C x1 y1 x2 y2 x y`).
    pub fn curve_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::CurveTo(x1, y1, x2, y2, x, y));
        self
    }

    /// Adds a close-path command (`Z`).
    pub fn close(mut self) -> Self {
        self.commands.push(PathCommand::Close);
        self
    }

    /// Adds a vertical line-to command (`V y`).
    pub fn elliptical_arc_to(mut self, rx: f32, ry: f32, x_axis_rotation: f32, large_arc_flag: bool, sweep_flag: bool, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::EllipticalArcTo(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y));
        self
    }

    /// Adds a smooth curve-to command (`S x2 y2 x y`).
    pub fn smooth_curve_to(mut self, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::SmoothCurveTo(x2, y2, x, y));
        self
    }

    /// Adds a quadratic Bezier curve-to command (`Q x1 y1 x y`).
    pub fn quadratic_bezier_curve_to(mut self, x1: f32, y1: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::QuadraticBezierCurveTo(x1, y1, x, y));
        self
    }

    /// Adds a smooth quadratic Bezier curve-to command (`T x y`).
    pub fn smooth_quadratic_bezier_curve_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::SmoothQuadraticBezierCurveTo(x, y));
        self
    }

    /// Adds a relative move command (`m dx dy`).
    pub fn move_by(mut self, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::MoveBy(dx, dy));
        self
    }

    /// Adds a relative line command (`l dx dy`).
    pub fn line_by(mut self, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::LineBy(dx, dy));
        self
    }

    /// Adds a relative horizontal line command (`h dx`).
    pub fn horizontal_by(mut self, dx: f32) -> Self {
        self.commands.push(PathCommand::HorizontalBy(dx));
        self
    }

    /// Adds a relative vertical line command (`v dy`).
    pub fn vertical_by(mut self, dy: f32) -> Self {
        self.commands.push(PathCommand::VerticalBy(dy));
        self
    }

    /// Adds a relative curve command (`c dx1 dy1 dx2 dy2 dx dy`).
    pub fn curve_by(mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::CurveBy(dx1, dy1, dx2, dy2, dx, dy));
        self
    }

    /// Adds a relative smooth curve command (`s dx2 dy2 dx dy`).
    pub fn smooth_curve_by(mut self, dx2: f32, dy2: f32, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::SmoothCurveBy(dx2, dy2, dx, dy));
        self
    }

    /// Adds a relative quadratic Bezier curve command (`q dx1 dy1 dx dy`).
    pub fn quadratic_bezier_curve_by(mut self, dx1: f32, dy1: f32, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::QuadraticBezierCurveBy(dx1, dy1, dx, dy));
        self
    }

    /// Adds a relative elliptical arc command (`a rx ry x_axis_rotation large_arc_flag sweep_flag dx dy`).
    pub fn elliptical_arc_by(mut self, rx: f32, ry: f32, x_axis_rotation: f32, large_arc_flag: bool, sweep_flag: bool, dx: f32, dy: f32) -> Self {
        self.commands.push(PathCommand::EllipticalArcBy(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy));
        self
    }

    /// Builds the final Path string.
    pub fn build(self) -> String {
        self.commands.iter().map(|cmd| cmd.to_string()).collect::<Vec<String>>().join(" ")
    }

    /// Converts to an `SvgElement::Path`.
    pub fn to_path(self) -> SvgElement {
        let d = self.commands
            .iter()
            .map(|cmd| cmd.to_string())  // Convert each command to string
            .collect::<Vec<String>>()    // Collect into a Vec<String>
            .join(" ");

        return SvgElement::Path { id: self.id, d, };
    }
}