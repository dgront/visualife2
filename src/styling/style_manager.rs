use std::collections::HashMap;
use crate::ElementID;
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
    ///
    /// # Example
    /// ```
    /// use visualife::styling::{Style, StyleManager};
    /// let mut manager = StyleManager::new();
    /// let style_id = manager.define_style(Style::new());
    /// ```
    pub fn define_style(&mut self, style: Style) -> usize {
        self.styles.push(style);
        return self.styles.len() - 1;
    }

    /// Applies an already defined style to an element by their ``id``.
    pub fn style_element(&mut self, style_id: usize, element_id: impl Into<ElementID>) {
        self.element_styles.insert((element_id).into(), style_id);
    }

    /// Retrieves the `id` of the style for a given element, or `None` if unstyled.
    pub fn get_style_id(&self, element_id: &ElementID) -> Option<usize> {
        return match self.element_styles.get(&element_id) {
            None => { None }
            Some(id) => { Some(*id) }
        }
    }

    /// Provide access to the style registered under a given index
    pub fn get_style(&self, style_id: usize) -> &Style {
        &self.styles[style_id]
    }

    /// Provide mutable access to the style registered under a given index
    pub fn get_style_mut(&mut self, style_id: usize) -> &mut Style {
        &mut self.styles[style_id]
    }
}