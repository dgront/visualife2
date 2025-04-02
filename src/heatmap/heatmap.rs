use crate::{ElementID, SvgDrawing};
use crate::basic_shapes::SvgElement;
use crate::styling::{ColorMap, Style};

pub struct Heatmap {
    pub id: ElementID,
    drawing: SvgDrawing,
    boxes: Vec<SvgElement>,
}

impl Heatmap {
    pub fn from_data(mut drawing: SvgDrawing, id: impl Into<ElementID>,
            box_width: f32, box_height: f32, data: Vec<Vec<f64>>, cmap: &ColorMap) -> Self {
        let boxes = Vec::new();

        let max_j = data.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..data.len() {
            let y = i as f32 * box_height;
            for j in 0..max_j {
                let box_id = format!("r:{i}:{j}");
                let x = j as f32 * box_width;
                let r = SvgElement::rect(box_id, x, y, box_width, box_height)
                    .with_style(&mut drawing, Style::new().fill(cmap.color(data[i][j])));
                drawing.add_element(r);
            }
        }

        Self { id: id.into(), drawing, boxes, }
    }

    pub fn to_svg(&self) -> String { self.drawing.to_svg() }
}