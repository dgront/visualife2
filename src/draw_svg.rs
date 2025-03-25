use crate::style::Style;

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

pub(crate) fn close_element(style: &Style, svg_string: &mut String) {

    if !style.is_empty() {
        svg_string.push_str(&format!(r#" style="{}""#, style.to_svg()));
    }
    svg_string.push_str(r#" />"#);
}


#[cfg(test)]
mod tests {
    use crate::draw_svg::close_element;
    use crate::style::Style;

    #[test]
    fn close_circle() {
        let mut svg_string = String::new();
        let mut style = Style::new();
        close_element(&style, &mut svg_string);
        assert_eq!(svg_string, r#" />"#);

        let mut svg_string = String::new();
        style.set_stroke_width(1.0);
        style.set_stroke("#000000");
        close_element(&style, &mut svg_string);
        assert_eq!(svg_string, r#" style="stroke:#000000;stroke-width:1;" />"#);
    }
}