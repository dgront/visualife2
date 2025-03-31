use crate::{ElementID, SvgDrawing};
use crate::basic_shapes::SvgElement;
use crate::basic_shapes::SvgElement::Circle;

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub id: ElementID,
    pub label: String,
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
}

impl Node {
    pub fn new(id: impl Into<ElementID>, label: &str, cx: f32, cy: f32, radius: f32) -> Self {
        return Node { id: id.into(), label: label.to_string(), cx, cy, radius }
    }

    pub fn create_elements(&self) -> SvgElement {

        let circle = SvgElement::circle(self.id.new_with_prefix("c"), self.cx, self.cy, self.radius);
        let text = SvgElement::text(self.id.new_with_prefix("t"), self.cx, self.cy, self.label.as_str());
        return SvgElement::group(self.id.clone(), vec![circle, text]);
    }
}

#[cfg(test)]
mod test_node {
    use crate::mindmap::node::Node;
    use crate::normalize_whitespace;
    use crate::styling::{Style, StyleManager};

    #[test]
    fn node_to_svg() {
        let (x1, y1, r1)  = (100.0_f32, 100.0_f32, 10.0_f32);
        let nodeA = Node::new("n1", "A", x1, y1, r1);
        let mut mgr = StyleManager::new();
        let expected = r#"<g id="n1" >
    <circle id="cn1" cx="100" cy="100" r="10" />
    <text x="100" y="100">A</text>
</g>"#;
        assert_eq!(normalize_whitespace(&nodeA.create_elements().to_svg(&mgr)), normalize_whitespace(&expected));
    }
}