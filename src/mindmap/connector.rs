use crate::basic_shapes::{PathBuilder, SvgElement};
use crate::mindmap::node::Node;
use crate::mindmap::{cartesian_to_polar, FOOT_LENGTH_R_FRACTION, polar_to_cartesian};
use crate::element_id::ElementID;
use crate::Point;

pub(crate) fn connector(node_a: &Node, node_b: &Node, foot_angle_deg: f32, bar_width: f32) -> SvgElement {

    let foot_angle_rad = foot_angle_deg.to_radians();
    // ---------- polar coordinates of node a as seen from node b ----------
    let (_r, theta_rad) = cartesian_to_polar(&node_a.center, &node_b.center);
    let id_str = format!("f{}:{}", &node_a.id, &node_b.id);
    let foot_a = foot_path(ElementID::from(&id_str), &node_a.center, node_a.radius, theta_rad, foot_angle_rad, bar_width);
    let id_str = format!("f{}:{}", &node_b.id, &node_a.id);
    let foot_b = foot_path(ElementID::from(&id_str), &node_b.center, node_b.radius, std::f32::consts::PI + theta_rad, foot_angle_rad, bar_width);

    let (x1a, y1a, x2a, y2a) = ankle_endpoints(&node_a.center, node_a.radius, theta_rad, bar_width);
    let (x1b, y1b, x2b, y2b) = ankle_endpoints(&node_b.center, node_b.radius, std::f32::consts::PI + theta_rad, bar_width);

    let id_str = format!("b{}:{}", &node_a.id, &node_b.id);
    let edge_path = PathBuilder::new(ElementID::from(&id_str)).move_to(x1a, y1a).line_to(x2b, y2b).line_to(x1b, y1b).line_to(x2a, y2a).close().to_path();

    let id = ElementID::from(&format!("c{}:{}", &node_a.id, &node_b.id));

    SvgElement::group(id, vec![foot_a, foot_b, edge_path])
}

/// computes the two points that define the top edge of a foot.
///
/// Returns the x, y coordinates of the two points.
fn ankle_endpoints(center: &Point, r: f32, foot_direction_rad: f32, bar_width: f32) -> (f32, f32, f32, f32) {

    // ---------- ankle midpoint
    let (mx, my) = polar_to_cartesian(FOOT_LENGTH_R_FRACTION * r, foot_direction_rad, center);
    // ---------- leg vector
    let lx = foot_direction_rad.cos();
    let ly = foot_direction_rad.sin();

    // ----------- ankle endpoints
    return calculate_perpendicular_segment(lx, ly, mx, my, bar_width);
}

fn foot_path(id: ElementID, center: &Point, r: f32, foot_direction_rad: f32, foot_angle_rad: f32, bar_width: f32) -> SvgElement {

    // ---------- foot endpoints
    let (a1xb, a1yb) = polar_to_cartesian(r, foot_direction_rad - 0.5 * foot_angle_rad, center);
    let (a1xe, a1ye) = polar_to_cartesian(r, foot_direction_rad + 0.5 * foot_angle_rad, center);

    // ---------- ankle midpoint
    let (x1, y1, x2, y2) = ankle_endpoints(center, r, foot_direction_rad, bar_width);

    // ----------- bezier control points
    let pi = std::f32::consts::PI;
    let (a1c1x, a1c1y) = polar_to_cartesian(0.15 * r, foot_direction_rad + pi / 2.5, &(a1xb, a1yb).into());
    let (a1c2x, a1c2y) = polar_to_cartesian(0.25 * r, foot_direction_rad + pi, &(x1, y1).into());

    let (a1c3x, a1c3y) = polar_to_cartesian(0.15 * r, foot_direction_rad - pi / 2.5, &(a1xe, a1ye).into());
    let (a1c4x, a1c4y) = polar_to_cartesian(0.25 * r, foot_direction_rad + pi, &(x2, y2).into());

    let foot_path = PathBuilder::new(id).move_to(x1, y1)
        .curve_to(a1c2x, a1c2y, a1c1x, a1c1y,  a1xb, a1yb)
        .elliptical_arc_to(r, r, 0.0, false, true, a1xe, a1ye)
        .curve_to(a1c3x, a1c3y, a1c4x, a1c4y, x2, y2);

    return foot_path.to_path();
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
    use crate::{ElementID, Point};
    use crate::mindmap::connector::{ankle_endpoints, connector};
    use crate::mindmap::node::Node;
    use crate::mindmap::FOOT_LENGTH_R_FRACTION;

    #[test]
    fn test_ankle() {
        let pts = ankle_endpoints(&Point::default(), 10.0, 0.0, 2.0);
        assert_eq!(pts, (10.0 * FOOT_LENGTH_R_FRACTION, -1.0, 10.0 * FOOT_LENGTH_R_FRACTION, 1.0));
    }

    #[test]
    fn test_connector() {
        let p100 = Point::new(100.0,100.0);
        let na = Node::new(ElementID::from(1), "A", p100, 10.0);
        let p100 = Point::new(100.0,100.0);
        let nb = Node::new(ElementID::from(2), "B", p100, 10.0);
        let _g = connector(&na, &nb, 0.0, 2.0);
        // println!("{:?}", _g.to_svg());
    }
}
