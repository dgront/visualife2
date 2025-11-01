use crate::basic_shapes::SvgElement;
use crate::element_id::ElementID;
use crate::mindmap::connector::connector;
use crate::mindmap::node::Node;
use crate::mindmap::polar_to_cartesian;

use indexmap::IndexMap;

pub struct Mindmap {
    pub foot_angle_deg: f32,
    pub bar_width: f32,
    pub id: ElementID,
    nodes: IndexMap<ElementID, Node>,
    connections: Vec<(ElementID, ElementID)>,
    max_node_radius: f32,
    node_radius_shrink_factor: f32,
}

impl Mindmap {
    pub fn new(id: impl Into<ElementID>, max_node_radius: f32) -> Self {
        Mindmap {
            foot_angle_deg: 30.0,
            bar_width: max_node_radius / 5.0,
            id: id.into(),
            nodes: IndexMap::new(),
            max_node_radius,
            connections: vec![],
            node_radius_shrink_factor: 0.8,
        }
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
        let (cx, cy) = polar_to_cartesian(parent.radius * 3.0, angle_deg.to_radians(), parent.cx, parent.cy);
        let id = id.into();
        let node = Node::new( id.clone(), label, cx, cy, parent.radius * self.node_radius_shrink_factor);
        self.nodes.insert(id.clone(), node);
        self.connect_nodes(real_parent_id, id.clone());
        self.nodes.get_mut(&id).unwrap()
    }

    pub fn connect_nodes(&mut self, from_id: impl Into<ElementID>, to_id: impl Into<ElementID>) {
        self.connections.push((from_id.into(), to_id.into()));
    }

    /// Non-mutable access to a [`Node`] of this [`Mindmap`]
    ///
    /// # Example
    /// ```
    /// # use visualife::ElementID;
    /// # use visualife::mindmap::{Mindmap, MindmapError};
    /// # fn main() -> Result<(), MindmapError> {
    /// let radius = 45.0;
    /// let mut mndmp = Mindmap::new("a_mindmap", radius);
    /// let node_id = ElementID::from("n1");
    /// mndmp.place_node(node_id.clone(), "Node 1", 30.0, 30.0);
    /// let node_ref = mndmp.node(&node_id).ok_or(MindmapError::node_not_found("n1"))?;
    /// assert_eq!(node_ref.radius, radius);
    /// # Ok(())
    /// # }
    /// ```
    pub fn node(&self, id: &ElementID) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Mutable access to a [`Node`] of this [`Mindmap`]
    ///
    /// This method allows to edit a node, e.g. to change its syle, as in the example below:
    /// # Example
    /// ```
    /// # use visualife::ElementID;
    /// # use visualife::mindmap::{Mindmap, MindmapError};
    /// # use visualife::styling::Style;
    /// # fn main() -> Result<(), MindmapError> {
    /// let radius = 45.0;
    /// let mut mndmp = Mindmap::new("a_mindmap", radius);
    /// let node_id = ElementID::from("n1");
    /// mndmp.place_node(node_id.clone(), "Node 1", 30.0, 30.0);
    /// let node_mut = mndmp.node_mut(&node_id).ok_or(MindmapError::node_not_found("n1"))?;
    /// node_mut.with_style( Style::new().fill("#AAAAAA").stroke("black"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn node_mut(&mut self, id: &ElementID) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn create_element(&self) -> SvgElement {
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
        SvgElement::group(self.id.clone(), vec![node_grp, connector_grp])
    }
}
