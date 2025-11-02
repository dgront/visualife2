mod testing_utilities; // Declare the module

#[cfg(test)]
mod test_mindmap {
    use crate::testing_utilities::load_expected_svgs;
    use visualife::styling::{Style, darker, palettes};
    use visualife::ElementID;
    use visualife::SvgDrawing;
    use visualife::mindmap::{Mindmap, MindmapError};

    #[test]
    fn test_node_access() -> Result<(), MindmapError> {
        let radius = 45.0;
        let mut mndmp = Mindmap::new("a_mindmap", radius);
        let node_id = ElementID::from("n1");
        mndmp.place_node(node_id.clone(), "Node 1", 30.0, 30.0);
        let node_ref = mndmp.node(&node_id).ok_or(MindmapError::node_not_found("n1"))?;
        assert_eq!(node_ref.radius, radius);
        Ok(())
    }

    #[test]
    fn place_nodes() -> Result<(), anyhow::Error> {

        let mut mndmp = Mindmap::new("a_mindmap", 30.0);
        for (i, color) in palettes::PASTEL.iter().enumerate() {
            let x = ((i%3) as f32) * 70.0 + 40.0;
            let y = ((i/3) as f32) * 70.0 + 40.0;
            mndmp.place_node(format!("c{i}"), color, x, y)
                .with_style(Style::new().fill(color).stroke(&darker(color, 0.2)?).stroke_width(2.0))
                .with_text_style(Style::new().fill(&darker(color, 0.8)?));
        }
        let mut drawing = SvgDrawing::new(220.0, 220.0);
        drawing.add_element(mndmp.create_element());
        let expected =
            load_expected_svgs("./tests/expected_drawings/mindmap/", &["pastel_nodes.svg"])?;
        // drawing.save_svg("pastel_nodes.svg")?;
        assert_eq!(drawing.to_svg(), expected[0]);

        Ok(())
    }

    #[test]
    fn two_nodes() -> std::io::Result<()> {
        let mut mndmp = Mindmap::new("a_mindmap", 45.0);
        let node1_id = mndmp.place_node(ElementID::from("n1"), "Node 1", 100.0, 100.0).id.clone();
        let node2_id = mndmp.place_node(ElementID::from("n2"), "Node 2", 180.0, 180.0).id.clone();
        mndmp.connect_nodes(node1_id, node2_id);
        let svg_el = mndmp.create_element();
        let mut drawing = SvgDrawing::new(300.0, 300.0);
        drawing.add_element(svg_el);
        // drawing.save_svg("two_nodes.svg")?;
        let expected =
            load_expected_svgs("./tests/expected_drawings/mindmap/", &["two_nodes.svg"])?;
        assert_eq!(drawing.to_svg(), expected[0]);
        Ok(())
    }

    #[test]
    fn grow_nodes() -> Result<(), anyhow::Error> {
        let mut mndmp = Mindmap::new("a_mindmap", 50.0);
        let center_node_id = mndmp.place_node("n0", "Center node", 80.0, 80.0).id.clone();
        let n_new_nodes = 5;
        let mut fill = String::from("#FFFFFF");
        for i in 0..n_new_nodes {
            let angle = (90.0 / ((n_new_nodes - 1) as f32) * i as f32);
            let n = mndmp.grow_node(&format!("n:{i}"),&format!("{angle}°"),angle,center_node_id.clone());
            fill = darker(fill.as_str(), 0.1)?;
            let style = Style::new()
                .fill(fill.as_str())
                .stroke_dasharray([15.0, 5.0])
                .stroke_width(3.0)
                .stroke("black");
            n.with_style(style);
        }
        let mut drawing = SvgDrawing::new(300.0, 300.0);
        drawing.add_element(mndmp.create_element());
        let expected =
            load_expected_svgs("./tests/expected_drawings/mindmap/", &["grow_nodes.svg"])?;
        // drawing.save_svg("grow_nodes.svg")?;
        assert_eq!(drawing.to_svg(), expected[0]);
        Ok(())
    }

    #[test]
    fn small_mindmap() -> Result<(), anyhow::Error> {
        let mut mndmp = Mindmap::new("a_mindmap", 50.0);
        mndmp.place_node("n0", "Center", 250.0, 250.0);
        for i in 1..=2 {
            let ni_id = ElementID::from(&format!("n:{i}"));
            mndmp.grow_node(ni_id, &format!("Node {i}"), 360.0 / 7.0 * i as f32, "n0");
        }
        let mut drawing = SvgDrawing::new(300.0, 300.0);
        drawing.add_element(mndmp.create_element());
        drawing.save_svg("small_mindmap.svg")?;
        Ok(())
    }
}
