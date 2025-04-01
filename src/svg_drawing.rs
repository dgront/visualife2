use std::fmt::Write;

use crate::basic_shapes::{SvgElement};
use crate::{ElementID};
use crate::styling::{Style, StyleManager};

pub struct SvgDrawing {
    width: f32,
    height: f32,
    styles: StyleManager,
    elements: Vec<SvgElement>,
}

impl SvgDrawing {

    /// Create a new SVG drawing with the given width and height.
    pub fn new(width: f32, height: f32) -> Self {
        SvgDrawing { width, height, styles: StyleManager::new(), elements: vec![] }
    }

    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.width }

    pub fn styles(&self) -> &StyleManager { &self.styles }

    pub fn styles_mut(&mut self) -> &mut StyleManager { &mut self.styles }

    /// Renders the SVG drawing and returns it as a String.
    ///
    /// This method is made efficient for large outputs by using a preallocated buffer.
    ///
    /// # Example
    /// ```
    /// use visualife::basic_shapes::SvgElement;
    /// use visualife::styling::Style;
    /// use visualife::SvgDrawing;
    /// let mut drawing = SvgDrawing::new(200.0, 30.0);
    /// for i in 0..9 {
    ///     let circle = SvgElement::circle(format!("circle{}", i), 20.0 * i as f32 + 15.0, 15.0, 9.0)
    ///        .with_style(&mut drawing, Style::new());
    ///     drawing.add_element(circle);
    /// }
    /// let svg_string = drawing.to_svg();
    /// std::fs::write("output.svg", svg_string).unwrap();
    /// ```
    pub fn to_svg(&self) -> String {
        // Estimate capacity: header + K elements * average line size
        let mut svg = String::with_capacity(1024 + self.elements.len() * 256);

        writeln!(svg, "{}", self.svg_header()).unwrap();
        for element in &self.elements {
            writeln!(svg, "{}", element.to_svg(&self.styles)).unwrap();
        }
        svg.push_str("</svg>\n");

        svg
    }

    pub fn add_element(&mut self, el: SvgElement) {
        self.elements.push(el);
    }

    pub fn add_element_to_group(&mut self, el: SvgElement, group_id: ElementID) -> Result<(), String> {
        if Self::add_to_group_recursive(&mut self.elements, &group_id, el) {
            Ok(())
        } else {
            Err(format!("Group with ID '{}' not found", group_id))
        }
    }

    pub fn svg_header(&self) -> String {
        format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, self.width, self.height).to_string()
    }

    /// Defines a new style in a [`StyleManager`](StyleManager).
    ///
    /// See [`StyleManager::define_style()`](StyleManager::define_style()) for more information.
    pub fn define_style(&mut self, style: Style) -> usize {
        self.styles.define_style(style)
    }

    /// Applies an already defined style to an element by their ``id``.
    ///
    /// See [`StyleManager::style_element()`](StyleManager::style_element()) for more information.
    pub fn style_element(&mut self, style_id: usize, element_id: impl Into<ElementID>) {
        self.styles.style_element(style_id, element_id);
    }

    /// Retrieves the `id` of the style for a given element, or `None` if unstyled.
    ///
    /// See [`StyleManager::get_style_id()`](StyleManager::get_style_id()) for more information.
    pub fn get_style_id(&self, element_id: &ElementID) -> Option<usize> {
        self.styles.get_style_id(element_id)
    }

    /// Provide access to the style registered under a given index
    ///
    /// See [`StyleManager::get_style()`](StyleManager::get_style()) for more information.
    pub fn get_style(&self, style_id: usize) -> &Style {
        self.styles.get_style(style_id)
    }

    /// Provide mutable access to the style registered under a given index
    ///
    /// See [`StyleManager::get_style_mut()`](StyleManager::get_style_mut()) for more information.
    pub fn get_style_mut(&mut self, style_id: usize) -> &mut Style {
        self.styles.get_style_mut(style_id)
    }

    /// Recursively searches for the group and adds the element.
    fn add_to_group_recursive(elements: &mut Vec<SvgElement>, group_id: &ElementID, el: SvgElement) -> bool {
        for element in elements.iter_mut() {
            match element {
                SvgElement::Group { id, elements: group_elements } => {
                    if id == group_id {
                        group_elements.push(el);
                        return true;
                    } else if Self::add_to_group_recursive(group_elements, group_id, el.clone()) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}
