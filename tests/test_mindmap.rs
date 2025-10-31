mod testing_utilities; // Declare the module

#[cfg(test)]
mod test_mindmap {
    use visualife::{mindmap, SvgDrawing};
    use visualife::ElementID;
    use visualife::styling::{Style, darker};
    use crate::testing_utilities::load_expected_svgs;

    #[test]
    fn two_nodes() {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 45.0);
        let n1_id = mndmp.place_node(ElementID::from("n1"), "Node 1", 100.0, 100.0).id.clone();
        let n2_id = mndmp.place_node(ElementID::from("n2"), "Node 2", 180.0, 180.0).id.clone();
        mndmp.connect_nodes(n1_id, n2_id);
        let svg_el = mndmp.create_element();
        // assert_eq!(svg_el.len(), 1);
        // let drawing = SvgDrawing::new(300.0, 300.0);
    }

    #[test]
    fn grow_nodes() -> Result<(), String> {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 50.0);
        let center_node_id = mndmp.place_node(ElementID::from("n0"), "Center node", 80.0, 80.0).id.clone();
        let n_new_nodes = 5;
        let mut fill = String::from("#FFFFFF");
        for i in 0..n_new_nodes {
            let angle = (90.0/((n_new_nodes-1) as f32) * i as f32);
            let n = mndmp.grow_node(ElementID::from(&format!("n:{i}")), &format!("{angle}°"),
                            angle, center_node_id.clone());
            fill = darker(fill.as_str(), 0.1)?;
            let style = Style::new().fill(fill.as_str())
                .stroke_dasharray([15.0,5.0]).stroke_width(3.0).stroke("black");
            n.with_style(style);
        }
        let mut drawing = SvgDrawing::new(300.0, 300.0);
        drawing.add_element(mndmp.create_element());
        let expected = load_expected_svgs("./tests/expected_drawings/mindmap/", &["grow_nodes.svg"])?;
        // drawing.save_svg("grow_nodes.svg")?;
        assert_eq!(drawing.to_svg(), expected[0]);
        Ok(())
    }


    #[test]
    fn small_mindmap() {
        let mut mndmp = mindmap::Mindmap::new("a_mindmap", 50.0);
        let n0 = mndmp.place_node("n0","Center", 250.0, 250.0);
        for i in 1..=2 {
            let ni_id = ElementID::from(&format!("n:{i}"));
            mndmp.grow_node(ni_id, &format!("Node {i}"), 360.0 / 7.0 * i as f32, "n0");
        }
        let mut drawing = SvgDrawing::new(300.0, 300.0);
        drawing.add_element(mndmp.create_element());

        drawing.to_svg();
    }
}
