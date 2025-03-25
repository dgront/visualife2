use crate::basic_shapes::ElementID;
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
    pub fn new(id: ElementID, label: &str, cx: f32, cy: f32, radius: f32) -> Self {
        return Node { id, label: label.to_string(), cx, cy, radius }
    }

    pub fn create_elements(&self) -> Vec<SvgElement> {
        return vec![Circle { id: self.id.clone(), cx: self.cx, cy: self.cy, r: self.radius, }];
    }
}


#[cfg(test)]
mod test_node {
    use crate::basic_shapes::ElementID;
    use crate::mindmap::node::Node;
    use crate::ToSvg;

    #[test]
    fn node_to_svg() {
        let (x1, y1, r1)  = (100.0_f32, 100.0_f32, 10.0_f32);
        let na = Node::new(ElementID::from(1), "A", x1, y1, r1);
        assert_eq!(na.to_svg(), r#"<circle id="1" cx="100" cy="100" r="10" />"#);
    }
}