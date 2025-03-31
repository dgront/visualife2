
use visualife::{mindmap, SvgDrawing};
use visualife::{ElementID};
use visualife::styling::darker;
use visualife::styling::Style;

fn main() {
    let draw_width = 1000.0;
    let drawing = SvgDrawing::new(draw_width, draw_width);
    let colors = vec!["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf"];
    let mut mndmp = mindmap::Mindmap::new(drawing, ElementID::from("a_mindmap"), 50.0);
    let n0_id = mndmp.place_node(ElementID::from("n0"), "Center", 250.0, 250.0);
    for i in 1..=5 {
        let node_id_str = format!("n:{i}");
        let el_id = mndmp.grow_node(&node_id_str, &format!("Node {i}"), 360.0 / 7.0 * i as f32, &n0_id);
        let style = Style::new()
            .fill(&colors[i % colors.len()])
            .stroke_width(2.0)
            .stroke(&darker(&colors[i % colors.len()], 0.1).unwrap());
        let style_id = mndmp.define_style(style);
        mndmp.style_node(style_id, el_id.clone());  // works both with node_id_str and el_id !
    }

    mndmp.draw();
}