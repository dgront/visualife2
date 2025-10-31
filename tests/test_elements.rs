#[cfg(test)]
mod test_elements {
    use visualife::basic_shapes::SvgElement;
    use visualife::styling::{Style};

    #[test]
    fn test_circle() {
        let c = SvgElement::circle("c1", 100.0, 100.0, 10.0);
        assert_eq!(c.to_svg(), r#"<circle id="c1" cx="100" cy="100" r="10" />"#);
    }

    #[test]
    fn test_rectangle() {
        let mut r = SvgElement::rect("r1", 100.0, 100.0, 10.0, 10.0);
        assert_eq!(r.to_svg(), r#"<rect id="r1" x="100" y="100" width="10" height="10" />"#);
        r.set_style(Style::new().fill("#1A2C00"));
        assert_eq!(r.to_svg(), r#"<rect id="r1" x="100" y="100" width="10" height="10" style="fill:#1A2C00;" />"#);
    }

    #[test]
    fn test_path() {
        let mut p = SvgElement::path("p1", "M 100 100 L 300 100 L 200 300 Z");
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z" />"#);
        p.set_style(Style::new().stroke("#000000"));
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z" style="stroke:#000000;" />"#);
    }

    #[test]
    fn test_group() {
        let c1 = SvgElement::circle("my_circle_1", 100.0, 50.0, 10.0);
        let c2 = SvgElement::circle("my_circle_2", 100.0, 100.0, 10.0);
        let g = SvgElement::group("my_group", vec![c1, c2]);
        let svg = g.to_svg();
        let expected1 = r#"<g id="my_group" >
<circle id="my_circle_1" cx="100" cy="50" r="10" />
<circle id="my_circle_2" cx="100" cy="100" r="10" />
</g>"#;
        assert_eq!(svg, expected1);
        println!("{}", svg);
    }
}
