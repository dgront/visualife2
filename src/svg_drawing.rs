use crate::basic_shapes::SvgElement;
use crate::{StyleManager, ToSvg};

pub struct SvgDrawing {
    width: f32,
    height: f32,
    elements: Vec<SvgElement>
}

impl SvgDrawing {
    pub fn new(width: f32, height: f32) -> Self {
        SvgDrawing { width, height, elements: vec![] }
    }

    pub fn draw(&mut self, style_manager: &StyleManager) {
        println!("{}", self.svg_header());
        for element in &self.elements {
            println!("{}", element.to_svg(style_manager));
        }
        println!("</svg>");
    }

    pub fn add_element(&mut self, el: SvgElement) {
        self.elements.push(el);
    }

    pub fn svg_header(&self) -> String {
        format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, self.width, self.height).to_string()
    }
}
