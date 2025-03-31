#[cfg(test)]
mod test_svg_drawing {
    use visualife::basic_shapes::{SvgElement};
    use visualife::basic_shapes::SvgElement::{Circle, Path, Rect};
    use visualife::styling::{Style, StyleManager};
    use visualife::SvgDrawing;

    #[test]
    fn test_drawing() {
        let mut drawing = SvgDrawing::new(100.0,100.0   );
        for i in 1..10 {
            let r = SvgElement::rect(format!("r{}", i), i as f32 * 8.0, 10.0, 10.0, 20.0);
            drawing.add_element(r);
            let style = Style::new().fill("red").stroke("blue").stroke_width(0.2);
            let style_id = drawing.styles_mut().add_style(style);
            drawing.styles_mut().style_element(style_id, format!("r{}", i));
        }
    }
}