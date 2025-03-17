use crate::mindmap::node::Node;
use crate::mindmap::{cartesian_to_polar, FOOT_LENGTH_R_FRACTION, polar_to_cartesian};
use crate::shapes::{Group, Path};
use crate::ToSvg;

pub(crate) fn connector(node_a: &Node, node_b: &Node, foot_angle_deg: f32, bar_width: f32) -> Group {

    let foot_angle_rad = foot_angle_deg.to_radians();
    // ---------- polar coordinates of node a as seen from node b ----------
    let (r, theta_rad) = cartesian_to_polar(node_a.cx, node_a.cy, node_b.cx, node_b.cy);
    let foot_a = foot_path(&format!("c:{}:{}", &node_a.id, &node_b.id), node_a.cx, node_a.cy, node_a.radius, theta_rad, foot_angle_rad, bar_width);
    let foot_b = foot_path(&format!("c:{}:{}", &node_b.id, &node_a.id), node_b.cx, node_b.cy, node_b.radius, std::f32::consts::PI + theta_rad, foot_angle_rad, bar_width);

    let (x1a, y1a, x2a, y2a) = ankle_endpoints(node_a.cx, node_a.cy, node_a.radius, theta_rad, bar_width);
    let (x1b, y1b, x2b, y2b) = ankle_endpoints(node_b.cx, node_b.cy, node_b.radius, std::f32::consts::PI + theta_rad, bar_width);
    let edge= Path::new(&format!("b:{}:{}", &node_a.id, &node_b.id)).move_to(x1a, y1a).line_to(x2b, y2b).line_to(x1b, y1b).line_to(x2a, y2a).close();

    let mut g = Group::new(&format!("c:{}:{}", &node_a.id, &node_b.id));
    g.add_element(Box::new(foot_a));
    g.add_element(Box::new(foot_b));
    g.add_element(Box::new(edge));

    return g;
}

/// computes the two points that define the top edge of a foot.
///
/// Returns the x, y coordinates of the two points.
fn ankle_endpoints(cx: f32, cy: f32, r: f32, foot_direction_rad: f32, bar_width: f32) -> (f32, f32, f32, f32) {

    // ---------- ankle midpoint
    let (mx, my) = polar_to_cartesian(FOOT_LENGTH_R_FRACTION * r, foot_direction_rad, cx, cy);
    // ---------- leg vector
    let lx = foot_direction_rad.cos();
    let ly = foot_direction_rad.sin();

    // ----------- ankle endpoints
    return calculate_perpendicular_segment(lx, ly, mx, my, bar_width);
}

fn foot_path(id: &str, cx: f32, cy: f32, r: f32, foot_direction_rad: f32, foot_angle_rad: f32, bar_width: f32) -> Path {

    // ---------- foot endpoints
    let (a1xb, a1yb) = polar_to_cartesian(r, foot_direction_rad - 0.5 * foot_angle_rad, cx, cy);
    let (a1xe, a1ye) = polar_to_cartesian(r, foot_direction_rad + 0.5 * foot_angle_rad, cx, cy);

    // ---------- ankle midpoint
    let (x1, y1, x2, y2) = ankle_endpoints(cx, cy, r, foot_direction_rad, bar_width);

    // ----------- bezier control points
    let pi = std::f32::consts::PI;
    let (a1c1x, a1c1y) = polar_to_cartesian(0.15 * r, foot_direction_rad + pi / 2.5, a1xb, a1yb);
    let (a1c2x, a1c2y) = polar_to_cartesian(0.25 * r, foot_direction_rad + pi, x1, y1);

    let (a1c3x, a1c3y) = polar_to_cartesian(0.15 * r, foot_direction_rad - pi / 2.5, a1xe, a1ye);
    let (a1c4x, a1c4y) = polar_to_cartesian(0.25 * r, foot_direction_rad + pi, x2, y2);

    let foot_path = Path::new(id).move_to(x1, y1)
        .curve_to(a1c2x, a1c2y, a1c1x, a1c1y,  a1xb, a1yb)
        .elliptical_arc_to(r, r, 0.0, false, true, a1xe, a1ye)
        .curve_to(a1c3x, a1c3y, a1c4x, a1c4y, x2, y2);

    return foot_path;
}

fn  calculate_perpendicular_segment(vx: f32, vy: f32, px: f32, py: f32, l: f32) -> (f32, f32, f32, f32) {
    // ---------- Normalize the perpendicular vector(-vy, vx)
    let magnitude = (vx * vx + vy * vy).sqrt();
    let ux = -vy / magnitude;
    let uy = vx / magnitude;

    // ---------- Calculate half - length of the segment
    let half_length = l / 2.0;

    // ---------- Calculate the endpoints of the segment
    let x1 = px - half_length * ux;
    let y1 = py - half_length * uy;
    let x2 = px + half_length * ux;
    let y2 = py + half_length * uy;

    return (x1, y1, x2, y2)
}


#[cfg(test)]
mod test_connector {
    use crate::mindmap::connector::{ankle_endpoints, connector};
    use crate::mindmap::node::Node;
    use crate::mindmap::FOOT_LENGTH_R_FRACTION;
    use crate::ToSvg;

    #[test]
    fn test_ankle() {

        let pts = ankle_endpoints(0.0, 0.0, 10.0, 0.0, 2.0);
        assert_eq!(pts, (10.0*FOOT_LENGTH_R_FRACTION, -1.0, 10.0*FOOT_LENGTH_R_FRACTION, 1.0));
    }


    #[test]
    fn test_connector() {
        let na = Node::new("a", "A", 100.0, 100.0, 10.0);
        let nb = Node::new("b", "B", 100.0, 100.0, 10.0);
        let g = connector(&na, &nb, 0.0, 2.0);
        println!("{:?}", g.to_svg());
    }
}