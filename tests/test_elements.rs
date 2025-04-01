#[cfg(test)]
mod test_elements {
    use visualife::basic_shapes::SvgElement;
    use visualife::styling::{Style, StyleManager};

    #[test]
    fn test_circle() {
        let mgr = StyleManager::new();
        let c = SvgElement::circle("c1", 100.0, 100.0, 10.0);
        assert_eq!(c.to_svg(&mgr), r#"<circle id="c1" cx="100" cy="100" r="10" />"#);
    }

    #[test]
    fn test_rectangle() {
        let mgr = StyleManager::new();
        let r = SvgElement::rect("r1", 100.0, 100.0, 10.0, 10.0);
        assert_eq!(r.to_svg(&mgr), r#"<rect id="r1" x="100" y="100" width="10" height="10" />"#);
    }

    #[test]
    fn test_path() {
        let mut mgr = StyleManager::new();
        let p = SvgElement::path("p1", "M 100 100 L 300 100 L 200 300 Z");
        assert_eq!(p.to_svg(&mgr), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z" />"#);
        let style_id = mgr.define_style(Style::new().stroke("#000000"));
        mgr.style_element(style_id, p.id().clone());
        assert_eq!(p.to_svg(&mgr), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z" style="stroke:#000000;" />"#);
    }

    #[test]
    fn test_group() {
        let c1 = SvgElement::circle("my_circle_1", 100.0, 50.0, 10.0);
        let c2 = SvgElement::circle("my_circle_2", 100.0, 100.0, 10.0);
        let g = SvgElement::group("my_group", vec![c1, c2]);
        let svg = g.to_svg(&StyleManager::new());
        let expected1 = r#"<g id="my_group" >
	<circle id="my_circle_1" cx="100" cy="50" r="10" />
	<circle id="my_circle_2" cx="100" cy="100" r="10" />
</g>"#;
        assert_eq!(svg, expected1);
        println!("{}", svg);
    }
}