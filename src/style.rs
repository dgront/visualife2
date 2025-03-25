
pub fn rgb_to_hex(r: u16, g: u16, b: u16) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[derive(Debug, Clone)]
pub struct Style {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub fill_opacity: Option<f32>,
    pub stroke_opacity: Option<f32>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            fill: None,
            stroke: None,
            stroke_width: None,
            opacity: None,
            fill_opacity: None,
            stroke_opacity: None,
        }
    }


    pub fn fill(mut self, fill: &str) -> Self {
        self.fill = Some(fill.to_string());
        self
    }

    pub fn stroke(mut self, stroke: &str) -> Self {
        self.stroke = Some(stroke.to_string());
        self
    }

    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn fill_opacity(mut self, fill_opacity: f32) -> Self {
        self.fill_opacity = Some(fill_opacity);
        self
    }

    pub fn stroke_opacity(mut self, stroke_opacity: f32) -> Self {
        self.stroke_opacity = Some(stroke_opacity);
        self
    }

    /// Returns true if all fields are None.
    /// This is used to determine if a style is empty, and it doesn't need to be written to the SVG element
    ///
    /// # Examples
    /// ```
    /// use visualife::style::Style;
    /// let mut style = Style::new();
    /// assert!(style.is_empty());
    /// style.fill = Some("red".to_string());
    /// assert!(! style.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
            self.fill.is_none()
            && self.stroke.is_none()
            && self.stroke_width.is_none()
            && self.opacity.is_none()
            && self.fill_opacity.is_none()
            && self.stroke_opacity.is_none()
    }

    pub fn to_svg(&self) -> String {
        if self.is_empty() { return String::new(); }

        let mut style_string = String::from(" style=\"");

        if let Some(ref fill) = self.fill {
            style_string.push_str(&format!("fill:{};", fill));
        }

        if let Some(ref stroke) = self.stroke {
            style_string.push_str(&format!("stroke:{};", stroke));
        }

        if let Some(stroke_width) = self.stroke_width {
            style_string.push_str(&format!("stroke-width:{};", stroke_width));
        }

        if let Some(opacity) = self.opacity {
            style_string.push_str(&format!("opacity:{};", opacity));
        }

        if let Some(fill_opacity) = self.fill_opacity {
            style_string.push_str(&format!("fill-opacity:{};", fill_opacity));
        }

        if let Some(stroke_opacity) = self.stroke_opacity {
            style_string.push_str(&format!("stroke-opacity:{};", stroke_opacity));
        }
        style_string.push_str("\"");

        style_string
    }
}
