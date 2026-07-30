
use visualife::{mindmap, SvgDrawing};
use visualife::styling::darker;
use visualife::styling::palettes::TABLEAU10;
use visualife::styling::Style;

fn main() {
    let draw_width = 1000.0;
    let colors = &TABLEAU10;
    let mut mndmp = mindmap::Mindmap::new("a_mindmap", 50.0);
    mndmp.place_node("n0", "Center", (250.0, 250.0).into())
        .with_style(Style::new().fill("white").stroke("black"))
        .with_text_style(Style::new().font_size("16.0"));
    for i in 1..=5 {
        let node_id_str = format!("n:{i}");
        let style = Style::new()
            .fill(&colors[i % colors.len()])
            .stroke_width(2.0)
            .stroke(&darker(&colors[i % colors.len()], 0.2).unwrap());
        mndmp.grow_node(&node_id_str, &format!("Node {i}"), 360.0 / 7.0 * i as f32, "n0")
            .with_style(style);
    }

    let mut drawing = SvgDrawing::new(draw_width, draw_width);
    drawing.add_element(mndmp.create_element());

    println!("{}", drawing.to_svg());
}
