use std::fs;
use crate::{ElementID, VisualifeError};
use crate::{basic_shapes::SvgElement};

pub struct SvgDrawing {
    width: f32,
    height: f32,
    elements: Vec<SvgElement>,
    defs: Vec<String>,
}

impl SvgDrawing {

    /// Create a new SVG drawing with the given width and height.
    pub fn new(width: f32, height: f32) -> Self {
        SvgDrawing { width, height, elements: vec![], defs: vec![] }
    }

    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.width }

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
    ///        .with_style(Style::new().fill("skyblue").stroke("navy"));
    ///     drawing.add_element(circle);
    /// }
    /// let svg_string = drawing.to_svg();
    /// std::fs::write("output.svg", svg_string).unwrap();
    /// ```
    pub fn to_svg(&self) -> String {
        // Estimate average line length
        let avg_len_per_element = 120;
        let header_len = 256;

        // Preallocate total buffer size
        let estimated_capacity = header_len + self.elements.len() * avg_len_per_element;
        let mut out = String::with_capacity(estimated_capacity);

        // Write header
        out.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height, self.width, self.height
        ));
        out.push_str("\n");

        // --- add definitions
        out.push_str("<defs>\n");
        for def in &self.defs {
            out.push_str(&def);
            out.push('\n');
        }
        out.push_str("</defs>\n");

        // Add elements
        for el in &self.elements {
            out.push_str("\t");
            out.push_str(&el.to_svg());
            out.push('\n');
        }

        // Close SVG
        out.push_str("</svg>\n");

        out
    }

    pub fn save_svg(&self, fname: &str) -> std::io::Result<()> {
        fs::write(fname, self.to_svg())?;
        Ok(())
    }

    /// Add a graphical element to this drawing.
    ///
    /// ```
    /// # use visualife::SvgDrawing;
    /// # use visualife::basic_shapes::SvgElement;
    /// let mut drawing = SvgDrawing::new(100.0, 100.0);
    /// drawing.add_element(SvgElement::circle("circle_1", 50.0, 50.0, 80.0));
    /// # let svg = drawing.to_svg();
    /// ```
    pub fn add_element<E: Into<SvgElement>>(&mut self, e: E) {
        self.elements.push(e.into());
    }

    pub fn add_definition(&mut self, defin: String) {
            self.defs.push(defin);
    }

    // ---- used by Python API!
    /// Attempts to add an element to a group.
    ///
    /// The group must be already created with ID `group_id`; the methid results in error
    /// when the group can't be found.
    pub fn add_element_to_group<E: Into<SvgElement>>(&mut self, element_to_add: E, group_id: &ElementID) -> Result<(), VisualifeError> {

        let ee = element_to_add.into();
        for elem in self.elements.iter_mut() {
            if elem.add_to_group(group_id, ee.clone()) {
                return Ok(());
            }
        }
        return Err(VisualifeError::NoSuchGroup { group_id: group_id.to_string() });
    }
}
