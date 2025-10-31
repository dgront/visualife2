mod testing_utilities; // Declare the module

#[cfg(test)]
mod test_palettes {
    use visualife::basic_shapes::{SvgElement};
    use visualife::styling::{darker, Style};
    use visualife::styling::palettes::*;
    use visualife::SvgDrawing;

    use crate::testing_utilities::load_expected_svgs;

    fn rect(i: usize, j: usize, color: &str) -> Result<SvgElement, String> {
        let stroke = darker(color, 0.2)?;
        let style = Style::new().fill(color).stroke(&stroke);
        let r = SvgElement::rect(format!("r{}", i),
                         i as f32 * 12.0 + 5.0, j as f32 * 12.0 + 5.0, 10.0, 20.0).with_style(style);
        return Ok(r);
    }

    #[test]
    fn test_categorical_palettes() -> Result<(), String> {

        let palettes: &[(&str, &[&str])] = &[
            ("accent.svg", &ACCENT),
            ("categorical_accent.svg", &CATEGORICAL_ACCENT),
            ("paired.svg", &PAIRED),
            ("pastel.svg", &PASTEL),
            ("tableau10.svg", &TABLEAU10),
            ("tableau20.svg", &TABLEAU20),
            ("viridis.svg", &VIRIDIS),
            ("okabe_ito.svg", &OKABE_ITO),
            ("ggplot2_default.svg", &GGPLOT2_DEFAULT),
            ("ibm_colors.svg", &IBM_COLORS),
            ("colorbrewer_set1.svg", &COLORBREWER_SET1),
            ("dark.svg", &DARK),
            ("dark2.svg", &DARK2),
        ];

        let extected_files: Vec<&str> = palettes.iter().map(|&(fname, _pal)| fname).collect();
        let expected = load_expected_svgs("./tests/expected_drawings/styling/", &extected_files)?;
        let mut i_pal = 0;
        for (_file_name, palette) in palettes {
            let n_colors = palette.len();
            let mut drawing = SvgDrawing::new((n_colors * 12 + 10) as f32, 30.0);
            let mut group_elements = vec![];
            for i in 0..palette.len() {
                let r = rect(i, 0, palette[i])?;
                group_elements.push(r);
            }
            let group = SvgElement::group("color bar", group_elements)
                .with_style(Style::new().stroke_width(1.5));
            drawing.add_element(group);
            let svg_str = drawing.to_svg();
            // fs::write(_file_name, &svg_str).map_err(|e| e.to_string())?;

            assert_eq!(svg_str, expected[i_pal]);
            i_pal += 1;
        }
        Ok(())
    }
}
