use crate::basic_shapes::{ElementID, SvgElement};
use crate::styling::{StyleManager};

pub struct SvgDrawing {
    width: f32,
    height: f32,
    styles: StyleManager,
    elements: Vec<SvgElement>,
}

impl SvgDrawing {
    pub fn new(width: f32, height: f32) -> Self {
        SvgDrawing { width, height, styles: StyleManager::new(), elements: vec![] }
    }

    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.width }

    pub fn styles(&self) -> &StyleManager { &self.styles }

    pub fn styles_mut(&mut self) -> &mut StyleManager { &mut self.styles }

    pub fn draw(&mut self) {
        println!("{}", self.svg_header());
        for element in &self.elements {
            println!("{}", element.to_svg(&self.styles));
        }
        println!("</svg>");
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
