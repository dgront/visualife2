use std::collections::HashMap;
use crate::basic_shapes::ElementID;
use crate::styling::Style;

/// Manages styles and style bindings for SVG elements.
pub struct StyleManager {
    styles: Vec<Style>,    // Maps style index to SvgStyle
    element_styles: HashMap<ElementID, usize>, // Maps element ID to Style ID
}

impl StyleManager {
    /// Creates a new StyleManager.
    pub fn new() -> Self { Self { styles: vec![], element_styles: HashMap::new(), } }

    /// Defines a new style.
    pub fn add_style(&mut self, style: Style) -> u32 {
        self.styles.push(style);
        return self.styles.len() as u32 - 1;
    }

    /// Binds a style to an element by ID.
    pub fn style_element(&mut self, style_id: u32, element_id: &ElementID) {
        self.element_styles.insert(element_id.clone(), style_id as usize);
    }

    /// Retrieves the style for a given element, or `None` if unstyled.
    pub fn get_style(&self, element_id: &ElementID) -> Option<&Style> {
        return match self.element_styles.get(&element_id) {
            None => { None }
            Some(id) => { Some(&self.styles[*id]) }
        }
    }
}