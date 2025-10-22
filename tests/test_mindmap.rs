#[cfg(test)]
mod test_mindmap {
    use visualife::{mindmap, SvgDrawing};
    use visualife::ElementID;

    #[test]
    fn two_nodes() {
        let drawing = SvgDrawing::new(300.0, 300.0);
        let mut mndmp = mindmap::Mindmap::new(drawing, "a_mindmap", 45.0);
        let n1_id = mndmp.place_node(ElementID::from("n1"), "Node 1", 100.0, 100.0).id.clone();
        let n2_id = mndmp.place_node(ElementID::from("n2"), "Node 2", 180.0, 180.0).id.clone();
        mndmp.connect_nodes(n1_id, n2_id);
        let svg = mndmp.create_elements();
        assert_eq!(svg.len(), 1);
    }

    #[test]
    fn grow_nodes() {
        let drawing = SvgDrawing::new(300.0, 300.0);
        let mut mndmp = mindmap::Mindmap::new(drawing, "a_mindmap", 50.0);
        let center_node_id = mndmp.place_node(ElementID::from("n0"), "Center node", 100.0, 100.0).id.clone();
        for i in 1..=7 {
            mndmp.grow_node(ElementID::from(&format!("n:{i}")), &format!("Node {i}"),
                            360.0/7.0 * i as f32, center_node_id.clone());
        }
        // let svg = mndmp.draw();
        // assert_eq!(svg.lines().count(), 14);
    }


    #[test]
    fn small_mindmap() {
        let drawing = SvgDrawing::new(300.0, 300.0);
        let mut mndmp = mindmap::Mindmap::new(drawing,"a_mindmap", 50.0);
        let n0 = mndmp.place_node("n0","Center", 250.0, 250.0);
        for i in 1..=2 {
            let ni_id = ElementID::from(&format!("n:{i}"));
            mndmp.grow_node(ni_id, &format!("Node {i}"), 360.0 / 7.0 * i as f32, "n0");
        }
        let _svg = mndmp.draw();
    }
}