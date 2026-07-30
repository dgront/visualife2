use crate::basic_shapes::SvgElement;
use crate::element_id::ElementID;
use crate::mindmap::connector::connector;
use crate::mindmap::node::Node;
use crate::mindmap::polar_to_cartesian;

use indexmap::IndexMap;
use crate::Point;

/// Represents the whole mindmap, with its nodes ans connectors
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

    /// Create a new node at a given location
    ///
    /// # Parameters
    /// - `id`: unique identifier for the node; any type implementing `Into<ElementID>`.
    /// - `label`: text rendered for the node
    /// - `x`, `y`: position in canvas/user units
    ///
    /// # Returns
    /// A mutable reference to the created node.
    ///
    /// # Examples
    ///
    /// **Place a single node and change its color:**
    ///
    /// ```rust
    /// # use visualife::mindmap::Mindmap;
    /// use visualife::Point;
    /// # use visualife::styling::Style;
    /// let mut mndmp = Mindmap::new("a_mindmap", 50.0);
    /// mndmp.place_node("root", "Root", Point::new(120.0, 80.0))
    ///     .with_style(Style::new().fill("#9A9A9A"));
    ///```
    ///
    /// **Place multiple nodes, each with its own style:**
    /// ```
    /// # use visualife::styling::{Style, darker, palettes};
    /// # use visualife::mindmap::{Mindmap, MindmapError};
    /// # fn place_nodes() -> Result<(), anyhow::Error> {
    /// let mut mndmp = Mindmap::new("a_mindmap", 30.0);
    /// for (i, color) in palettes::PASTEL.iter().enumerate() {
    ///     let x = ((i%3) as f32) * 70.0 + 40.0;
    ///     let y = ((i/3) as f32) * 70.0 + 40.0;
    ///     mndmp.place_node(format!("c{i}"), color, (x, y).into())
    ///         .with_style(Style::new().fill(color).stroke(&darker(color, 0.2)?).stroke_width(2.0))
    ///         .with_text_style(Style::new().fill(&darker(color, 0.8)?));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[doc = include_str!("../../tests/expected_drawings/mindmap/pastel_nodes.svg")]
    ///
    pub fn place_node(&mut self, id: impl Into<ElementID>, label: &str, center: Point) -> &mut Node {
        let id = id.into();
        let node = Node::new(id.clone(), label, center, self.max_node_radius);
        self.nodes.insert(id.clone(), node);
        self.nodes.get_mut(&id).unwrap()
    }

    /// Grows a new node in a given direction
    ///
    /// Creates a new node growing at a given angle from the parent one. Connects the two nodes with the connector
    /// of the default length.
    ///
    /// ```
    /// # use visualife::mindmap::Mindmap;
    /// # use visualife::styling::{Style, darker};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let mut mndmp = Mindmap::new("a_mindmap", 50.0);
    /// let center_node_id = mndmp.place_node("n0", "Center node", (80.0, 80.0).into()).id.clone();
    /// let n_new_nodes = 5;
    /// let mut fill = String::from("#FFFFFF");
    /// for i in 0..n_new_nodes {
    ///     let angle = (90.0 / ((n_new_nodes - 1) as f32) * i as f32);
    ///     // --- grow a new node
    ///     let new_node = mndmp.grow_node(&format!("n:{i}"),&format!("{angle}°"), angle, center_node_id.clone());
    ///     // --- create a new style and assign to the new node
    ///     fill = darker(fill.as_str(), 0.1)?;
    ///     let style = Style::new()
    ///         .fill(fill.as_str())
    ///         .stroke_dasharray([15.0, 5.0])
    ///         .stroke_width(3.0)
    ///         .stroke("black");
    ///     new_node.with_style(style);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[doc = include_str!("../../tests/expected_drawings/mindmap/grow_nodes.svg")]
    ///
    pub fn grow_node(&mut self, id: impl Into<ElementID>, label: &str, angle_deg: f32, parent_id: impl Into<ElementID>) -> &mut Node {
        let real_parent_id = parent_id.into();
        let parent = self.nodes.get(&real_parent_id).unwrap();
        let center = polar_to_cartesian(parent.radius * 3.0, angle_deg.to_radians(), &parent.center);
        let id = id.into();
        let node = Node::new( id.clone(), label, center.into(), parent.radius * self.node_radius_shrink_factor);
        self.nodes.insert(id.clone(), node);
        self.connect_nodes(real_parent_id, id.clone());
        self.nodes.get_mut(&id).unwrap()
    }

    /// Connect two nodes that have been already created
    ///
    /// ```
    /// # use visualife::mindmap::{Mindmap, MindmapError};
    /// let mut mndmp = Mindmap::new("a_mindmap", 45.0);
    /// let node1_id = mndmp.place_node("n1", "Node 1", (100.0, 100.0).into()).id.clone();
    /// let node2_id = mndmp.place_node("n2", "Node 2", (180.0, 180.0).into()).id.clone();
    /// mndmp.connect_nodes(node1_id, node2_id);
    /// ```
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
    /// # let mut mndmp = Mindmap::new("a_mindmap", 45.0);
    /// # let node_id = ElementID::from("n1");
    /// # mndmp.place_node(node_id.clone(), "Node 1", (30.0, 30.0).into());
    /// let node_ref = mndmp.node(&node_id).ok_or(MindmapError::node_not_found("n1"))?;
    /// assert_eq!(node_ref.radius, 45.0);
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
    /// # use visualife::Point;
    /// let mut mndmp = Mindmap::new("a_mindmap", 45.0);
    /// # let node_id = ElementID::from("n1");
    /// # mndmp.place_node(node_id.clone(), "Node 1", Point::new(30.0, 30.0));
    /// let node_mut = mndmp.node_mut(&node_id).ok_or(MindmapError::node_not_found("n1"))?;
    /// node_mut.with_style( Style::new().fill("#AAAAAA").stroke("black"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn node_mut(&mut self, id: &ElementID) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Creates an SVG group element that contains all graphical components representing this mindmap
    ///
    /// Once you created your mindmap with all the desired nodes and connectors,
    /// create an SVG `<g>` (group) element and insert it into a [`SvgDrawing`](crate::SvgDrawing)
    ///
    /// ```
    /// # use visualife::mindmap::Mindmap;
    /// # use visualife::SvgDrawing;
    /// # use visualife::Point;
    /// # use std::fs;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // --- create a mindmap
    /// let mut mndmp = Mindmap::new("a_mindmap", 45.0);
    /// // --- add some nodes
    /// mndmp.place_node("n1", "Node 1", Point::new(100.0, 100.0));
    /// // --- create SvgDrawing
    /// let mut drawing = SvgDrawing::new(220.0, 220.0);
    /// // --- create the element for the mindmap and place it in the drawing
    /// drawing.add_element(mndmp.create_element());
    /// // --- save to file
    /// drawing.save_svg("figure.svg")?;
    /// # fs::remove_file("figure.svg").unwrap(); // cleanup
    /// # Ok(())
    /// # }
    /// ```
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

impl From<&Mindmap> for SvgElement {
    /// Creates an SVG group that contains all graphical elements for this mindmap
    fn from(m: &Mindmap) -> Self { m.create_element() }
}
