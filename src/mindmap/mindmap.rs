use std::collections::HashMap;
use crate::basic_shapes::{ElementID, SvgElement};
use crate::mindmap::connector::connector;
use crate::mindmap::node::Node;
use crate::mindmap::polar_to_cartesian;
use crate::basic_shapes::SvgElement::{Group};
use crate::style::Style;
use crate::SvgDrawing;

// #[derive(Clone)]
pub struct Mindmap {
    pub foot_angle_deg: f32,
    pub bar_width: f32,
    pub id: ElementID,
    drawing: SvgDrawing,
    nodes: HashMap<ElementID, Node>,
    connections: Vec<(ElementID,ElementID)>,
    max_node_radius: f32,
    node_radius_shrink_factor: f32,
}

impl Mindmap {
    pub fn new(drawing: SvgDrawing, id: ElementID, max_node_radius: f32) -> Self {
        Mindmap {
            foot_angle_deg: 30.0,
            bar_width: max_node_radius / 5.0,
            id,
            drawing,
            nodes: HashMap::new(),
            max_node_radius,
            connections: vec![],
            node_radius_shrink_factor: 0.8 }
    }

    pub fn place_node(&mut self, id: ElementID, label: &str, x: f32, y: f32) -> ElementID {
        let el = Node::new(id.clone(), label, x, y, self.max_node_radius);
        self.nodes.insert(id.clone(), el);
        return id;
    }

    pub fn grow_node(&mut self, id: ElementID, label: &str, angle_deg: f32, parent_node_id: &ElementID) -> ElementID {

        let parent = self.nodes.get(parent_node_id).unwrap(); // todo: handle error
        let (cx, cy) = polar_to_cartesian(parent.radius*3.0, angle_deg, parent.cx, parent.cy);
        let el = Node::new(id.clone(), label, cx, cy, parent.radius * self.node_radius_shrink_factor);
        self.nodes.insert(id.clone(), el);

        self.connect_nodes(parent_node_id.clone(), id.clone());

        return id;
    }

    pub fn connect_nodes(&mut self, from_id: ElementID, to_id: ElementID) {
        self.connections.push((from_id, to_id));
    }

    pub fn style_node(&mut self, style_id: u32, element_id: &ElementID) {
        self.drawing.styles_mut().style_element(style_id, element_id);
    }

    /// Defines a new style.
    pub fn define_style(&mut self, style: Style) -> u32 {
        self.drawing.styles_mut().add_style(style)
    }

    pub fn create_elements(&self) -> Vec<SvgElement> {
        // ---------- Create nodes and store them in a group
        let mut elements = vec![];
        for node in self.nodes.values() {
            elements.append(&mut node.create_elements());
        }
        let node_grp = Group{ id: self.id.new_with_prefix("n"), elements };

        // ---------- Create connectors and store them in a group
        let mut elements = vec![];
        for (from_id, to_id) in &self.connections {
            let from_node = self.nodes.get(from_id).unwrap();
            let to_node = self.nodes.get(to_id).unwrap();
            let connector = connector(from_node, to_node, self.foot_angle_deg, self.bar_width);
            elements.push(connector);
        }
        let connector_grp = Group{ id: self.id.new_with_prefix("n"), elements };

        // ---------- Create the mindmap group
        let elements = vec![node_grp, connector_grp];
        let mindmap_group = Group{ id: self.id.clone(), elements};

        return vec![mindmap_group];
    }

    pub fn draw(&mut self) {
        for el in self.create_elements() {
            self.drawing.add_element(el)
        }
        self.drawing.draw();
    }
}
