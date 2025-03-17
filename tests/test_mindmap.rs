#[cfg(test)]
mod test_mindmap {
    use visualife::{mindmap, SvgDrawing, ToSvg};

    #[test]
    fn two_nodes() {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 45.0);
        mndmp.place_node("n1", "Node 1", 100.0, 100.0);
        mndmp.place_node("n2", "Node 2", 180.0, 180.0);
        mndmp.connect_nodes("n1", "n2");
        let svg = mndmp.to_svg();
        assert_eq!(svg.lines().count(), 13);
    }

    #[test]
    fn grow_nodes() {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 50.0);
        mndmp.place_node("n0", "Center node", 100.0, 100.0);
        for i in 1..=7 {
            mndmp.grow_node(&format!("n:{i}"), &format!("Node {i}"), 360.0/7.0 * i as f32, "n0");
        }
        let svg = mndmp.to_svg();
        assert_eq!(svg.lines().count(), 14);
    }


    #[test]
    fn small_mindmap() {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 50.0);
        mndmp.place_node("n0", "Center", 250.0, 250.0);
        for i in 1..=2 {
            mndmp.grow_node(&format!("n:{i}"), &format!("Node {i}"), 360.0 / 7.0 * i as f32, "n0");
        }

        let mut drawing = SvgDrawing::new(500.0, 500.0);
        drawing.add_element(Box::new(mndmp));
        drawing.draw();
    }
}