#[cfg(test)]
mod test_svg_drawing {
    use visualife::basic_shapes::{ElementID, SvgElement};
    use visualife::basic_shapes::SvgElement::{Circle, Path, Rect};
    use visualife::styling::{Style, StyleManager};
    use visualife::SvgDrawing;

    #[test]
    fn test_drawing() {
        let mut drawing = SvgDrawing::new(100.0,100.0   );
        for i in 1..10 {
            let el_id = drawing.add_element(Rect{ id: ElementID::from(format!("r{}",i)),
                x: i as f32 * 8.0, y: 10.0, width: 10.0,  height: 20.0 });
            let style = Style::new().fill("red").stroke("blue").stroke_width(0.2);
            let style_id = drawing.styles_mut().add_style(style);
            drawing.styles_mut().style_element(style_id, &el_id);
        }
    }
}