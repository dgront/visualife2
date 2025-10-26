use crate::{ElementID, SvgDrawing};
use crate::basic_shapes::SvgElement;
use crate::styling::{ColorMap, Style};
use crate::heatmap::Heatmap;

///
/// ```
/// use rand::Rng;
/// use visualife::heatmap::HeatmapBuilder;
/// use visualife::styling::ColorMap;
/// use visualife::styling::palettes::RED_BLUE;
/// use visualife::SvgDrawing;
/// # let mut rng = rand::thread_rng();
/// let matrix: Vec<Vec<f64>> = (0..10).map(|_| (0..10).map(|_| rng.gen::<f64>()).collect()).collect();
/// let cmap = ColorMap::from_range(&RED_BLUE, 0.0, 1.0).unwrap();
/// let htmp = HeatmapBuilder::new(SvgDrawing::new(15.0,15.0), "hmap").box_size(10.0, 10.0).build_from_matrix(matrix);
/// ```
///
///
pub struct HeatmapBuilder {
    id: ElementID,
    box_width: f32,
    box_height: f32,
    offset_x: f32,
    offset_y: f32,
    cmap: ColorMap,
    x_labels: Option<Vec<String>>,
    y_labels: Option<Vec<String>>,
    x_title: Option<String>,
    y_title: Option<String>,
    pub(crate) drawing: SvgDrawing,
}

impl HeatmapBuilder {
    pub fn new(drawing: SvgDrawing, id: impl Into<ElementID>, ) -> Self {
        Self {
            id: id.into(),
            box_width: 20.0,
            box_height: 20.0,
            offset_x: 0.0,
            offset_y: 0.0,
            cmap: ColorMap::from_range(&crate::styling::palettes::RED_BLUE, 0.0, 1.0).unwrap(),
            x_labels: None,
            y_labels: None,
            x_title: None,
            y_title: None,
            drawing
        }
    }

    pub fn box_size(mut self, width: f32, height: f32) -> Self {
        self.box_width = width;
        self.box_height = height;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.offset_x = dx;
        self.offset_y = dy;
        self
    }

    pub fn cmap(mut self, cmap: ColorMap) -> Self {
        self.cmap = cmap;
        self
    }

    pub fn x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = Some(labels);
        self
    }

    pub fn y_labels(mut self, labels: Vec<String>) -> Self {
        self.y_labels = Some(labels);
        self
    }

    pub fn x_title(mut self, title: impl Into<String>) -> Self {
        self.x_title = Some(title.into());
        self
    }

    pub fn y_title(mut self, title: impl Into<String>) -> Self {
        self.y_title = Some(title.into());
        self
    }


    pub fn build_from_matrix(self, data: Vec<Vec<f64>>) -> Heatmap {

        let mut boxes = Vec::new();
        let max_cols = data.iter().map(Vec::len).max().unwrap_or(0);

        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                let color = self.cmap.color(val);
                let style = Style::new().fill(&color);

                let x = self.offset_x + j as f32 * self.box_width;
                let y = self.offset_y + i as f32 * self.box_height;
                let box_id = format!("r:{i}:{j}");

                let el = SvgElement::rect(box_id, x, y, self.box_width, self.box_height)
                    .with_style(style);

                // drawing.add_element(el.clone());
                boxes.push(el);
            }
        }

        // (Future: render titles and labels here)

        Heatmap {
            id: self.id,
            drawing: self.drawing,
            boxes,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
        }
    }
}
