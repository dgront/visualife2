use crate::basic_shapes::SvgElement;
use crate::{StyleManager};

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

    pub fn svg_header(&self) -> String {
        format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, self.width, self.height).to_string()
    }
}
