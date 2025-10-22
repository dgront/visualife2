

/// Style defines how an SVG element looks like
///
/// # Examples
/// ```
/// use visualife::styling::Style;
/// let style = Style::new().fill("#a6cee3").stroke("#1f78b4").stroke_width(0.25).opacity(0.5);
/// let style_str = style.to_svg();
/// ```
#[derive(Debug, Clone)]
pub struct Style {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub fill_opacity: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub text_anchor: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub text_decoration: Option<String>,
    pub dominant_baseline: Option<String>,
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
            text_anchor: None,
            font_family: None,
            font_size: None,
            font_weight: None,
            text_decoration: None,
            dominant_baseline: None,
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
    /// Sets the `text-anchor` property (e.g., `"start"`, `"middle"`, `"end"`).
    pub fn text_anchor(mut self, value: &str) -> Self {
        self.text_anchor = Some(value.to_string());
        self
    }

    /// Sets the `font-family` property (e.g., `"Arial"`, `"sans-serif"`).
    pub fn font_family(mut self, value: &str) -> Self {
        self.font_family = Some(value.to_string());
        self
    }

    /// Sets the `font-size` property (e.g., `"12px"`, `"1.2em"`).
    pub fn font_size(mut self, value: &str) -> Self {
        self.font_size = Some(value.to_string());
        self
    }

    /// Sets the `font-weight` property (e.g., `"normal"`, `"bold"`, `"700"`).
    pub fn font_weight(mut self, value: &str) -> Self {
        self.font_weight = Some(value.to_string());
        self
    }

    /// Sets the `text-decoration` property (e.g., `"underline"`, `"line-through"`).
    pub fn text_decoration(mut self, value: &str) -> Self {
        self.text_decoration = Some(value.to_string());
        self
    }

    /// Sets the `dominant-baseline` property (e.g., `"middle"`, `"hanging"`, `"baseline"`).
    pub fn dominant_baseline(mut self, value: &str) -> Self {
        self.dominant_baseline = Some(value.to_string());
        self
    }

    /// Returns true if all fields are None.
    /// This is used to determine if a style is empty, and it doesn't need to be written to the SVG element
    ///
    /// # Examples
    /// ```
    /// use visualife::styling::Style;
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
            && self.text_anchor.is_none()
            && self.font_family.is_none()
            && self.font_size.is_none()
            && self.font_weight.is_none()
            && self.text_decoration.is_none()
            && self.dominant_baseline.is_none()
    }

    pub fn to_svg(&self) -> String {
        if self.is_empty() { return String::new(); }

        let mut style_string = String::from("style=\"");

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
            style_string.push_str(&format!("opacity:{:.3};", opacity));
        }

        if let Some(fill_opacity) = self.fill_opacity {
            style_string.push_str(&format!("fill-opacity:{:.3};", fill_opacity));
        }

        if let Some(stroke_opacity) = self.stroke_opacity {
            style_string.push_str(&format!("stroke-opacity:{:.3};", stroke_opacity));
        }
        if let Some(ref text_anchor) = self.text_anchor {
            style_string.push_str(&format!("text-anchor:{};", text_anchor));
        }
        if let Some(ref font_family) = self.font_family {
            style_string.push_str(&format!("font-family:{};", font_family));
        }
        if let Some(ref font_size) = self.font_size {
            style_string.push_str(&format!("font-size:{};", font_size));
        }
        if let Some(ref font_weight) = self.font_weight {
            style_string.push_str(&format!("font-weight:{};", font_weight));
        }
        if let Some(ref text_decoration) = self.text_decoration {
            style_string.push_str(&format!("text-decoration:{};", text_decoration));
        }
        if let Some(ref dominant_baseline) = self.dominant_baseline {
            style_string.push_str(&format!("dominant-baseline:{};", dominant_baseline));
        }
        style_string.push_str("\"");

        style_string
    }
}
