#[cfg(test)]
mod test_mindmap {
    use visualife::{mindmap, SvgDrawing, ToSvg};
    use visualife::basic_shapes::ElementID;

    #[test]
    fn two_nodes() {
        let mut mndmp = mindmap::Mindmap::new(ElementID::from("a_mindmap"), 45.0);
        let n1_id = mndmp.place_node(ElementID::from("n1"), "Node 1", 100.0, 100.0);
        let n2_id = mndmp.place_node(ElementID::from("n2"), "Node 2", 180.0, 180.0);
        mndmp.connect_nodes(n2_id, n1_id);
        let svg = mndmp.create_elements();
        assert_eq!(svg.lines().count(), 13);
    }

    #[test]
    fn grow_nodes() {
        let mut mndmp = mindmap::Mindmap::new(ElementID::from("a_mindmap"), 50.0);
        let n0_id = mndmp.place_node(ElementID::from("n0"), "Center node", 100.0, 100.0);
        for i in 1..=7 {
            mndmp.grow_node(ElementID::from(&format!("n:{i}")), &format!("Node {i}"), 360.0/7.0 * i as f32, &n0_id);
        }
        let svg = mndmp.to_svg();
        assert_eq!(svg.lines().count(), 14);
    }


    #[test]
    fn small_mindmap() {
        let mut mndmp = mindmap::Mindmap::new(ElementID::from("a_mindmap"), 50.0);
        let n0_id = mndmp.place_node(ElementID::from("n0"), "Center", 250.0, 250.0);
        for i in 1..=2 {
            mndmp.grow_node(ElementID::from(&format!("n:{i}")), &format!("Node {i}"), 360.0 / 7.0 * i as f32, &n0_id);
        }

        let mut drawing = SvgDrawing::new(500.0, 500.0);
        drawing.add_element(Box::new(mndmp));
        drawing.draw();
    }
}