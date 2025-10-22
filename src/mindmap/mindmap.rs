use std::collections::HashMap;
use crate::basic_shapes::{SvgElement};
use crate::mindmap::connector::connector;
use crate::mindmap::node::Node;
use crate::mindmap::polar_to_cartesian;
use crate::element_id::ElementID;
use crate::SvgDrawing;

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
    pub fn new(drawing: SvgDrawing, id: impl Into<ElementID>, max_node_radius: f32) -> Self {
        Mindmap {
            foot_angle_deg: 30.0,
            bar_width: max_node_radius / 5.0,
            id: id.into(),
            drawing,
            nodes: HashMap::new(),
            max_node_radius,
            connections: vec![],
            node_radius_shrink_factor: 0.8 }
    }

    pub fn place_node(&mut self, id: impl Into<ElementID>, label: &str, x: f32, y: f32) -> &mut Node {
        let id = id.into();
        let node = Node::new(id.clone(), label, x, y, self.max_node_radius);
        self.nodes.insert(id.clone(), node);
        self.nodes.get_mut(&id).unwrap()
    }


    pub fn grow_node(&mut self, id: impl Into<ElementID>, label: &str, angle_deg: f32, parent_id: impl Into<ElementID>) -> &mut Node {
        let real_parent_id = parent_id.into();
        let parent = self.nodes.get(&real_parent_id).unwrap();
        let (cx, cy) = polar_to_cartesian(parent.radius * 3.0, angle_deg, parent.cx, parent.cy);
        let id = id.into();
        let node = Node::new(id.clone(), label, cx, cy, parent.radius * self.node_radius_shrink_factor);
        self.nodes.insert(id.clone(), node);
        self.connect_nodes(real_parent_id, id.clone());
        self.nodes.get_mut(&id).unwrap()
    }


    pub fn connect_nodes(&mut self, from_id: impl Into<ElementID>, to_id: impl Into<ElementID>) {
        self.connections.push((from_id.into(), to_id.into()));
    }

    pub fn node(&self, id: &ElementID) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &ElementID) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn create_elements(&self) -> Vec<SvgElement> {
        // ---------- Create nodes and store them in a group
        let mut node_elements = Vec::new();
        for node in self.nodes.values() {
            node_elements.push(node.create_element());
        }
        let node_grp = SvgElement::group(self.id.new_with_prefix("n"), node_elements);

        // ---------- Create connectors and store them in a group
        let mut connector_elements = Vec::new();
        for (from_id, to_id) in &self.connections {
            let from_node = self.nodes.get(from_id).unwrap();
            let to_node = self.nodes.get(to_id).unwrap();
            let connector = connector(from_node, to_node, self.foot_angle_deg, self.bar_width);
            connector_elements.push(connector);
        }
        let connector_grp = SvgElement::group(self.id.new_with_prefix("c"), connector_elements);

        // ---------- Create the mindmap group
        let top_group = SvgElement::group(
            self.id.clone(),
            vec![node_grp, connector_grp],
        );

        vec![top_group]
    }

    pub fn draw(&mut self) -> String {
        for el in self.create_elements() {
            self.drawing.add_element(el)
        }
        self.drawing.to_svg()
    }
}
