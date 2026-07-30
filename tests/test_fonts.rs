mod testing_utilities; // Declare the module

#[cfg(test)]
mod test_fonts {
    use std::io;
    use visualife::basic_shapes::{embed_font_file, embed_vl_font, SvgElement, VlFont};
    use visualife::styling::Style;
    use visualife::SvgDrawing;
    use crate::testing_utilities::load_expected_svgs;

    #[test]
    fn test_file_embeding() -> io::Result<()> {
        let font64 = embed_font_file("MyFont", "assets/dejavu-sans-webfont.woff2", None)?;

        let mut drawing = SvgDrawing::new(200.0,50.0);
        drawing.add_definition(font64);
        let txt = SvgElement::text("txt", 20.0, 20.0, "Brown fox")
            .with_style(Style::new().font_family("MyFont").fill("#000000"));
        drawing.add_element(txt);
        // drawing.save_svg("embeded_font.svg")?;

        let expected =
            load_expected_svgs("./tests/expected_drawings/basic_shapes/", &["embeded_font.svg"])?;
        assert_eq!(drawing.to_svg(), expected[0]);

        Ok(())
    }

    #[test]
    fn test_vl_embeding() -> io::Result<()> {
        let font64 = embed_vl_font(VlFont::DejaVuSansRegular, None)?;

        let mut drawing = SvgDrawing::new(200.0,50.0);
        drawing.add_definition(font64);
        let txt = SvgElement::text("txt", 20.0, 20.0, "Brown fox")
            .with_style(Style::new().font_family("MyFont").fill("#000000"));
        drawing.add_element(txt);
        // drawing.save_svg("embeded_vl_font.svg")?;

        let expected =
            load_expected_svgs("./tests/expected_drawings/basic_shapes/", &["embeded_vl_font.svg"])?;
        assert_eq!(drawing.to_svg(), expected[0]);

        Ok(())
    }
}