use crate::{ElementID, SvgDrawing};
use crate::basic_shapes::SvgElement;
use crate::styling::{ColorMap, Style};

///
/// ```
/// use std::fs;
/// use rand::Rng;
/// use visualife::heatmap::Heatmap;
/// use visualife::styling::ColorMap;
/// use visualife::styling::palettes::RED_BLUE;
/// use visualife::SvgDrawing;
/// # fn main()-> Result<(), String> {
/// let drawing = SvgDrawing::new(300.0, 300.0);
/// # let mut rng = rand::thread_rng();
/// let matrix: Vec<Vec<f64>> = (0..10).map(|_| (0..10).map(|_| rng.gen::<f64>()).collect()).collect();
/// let cmap = ColorMap::from_range(&RED_BLUE, 0.0, 1.0).unwrap();
/// let htm = Heatmap::from_matrix(drawing, "heatmap", 20.0, 20.0, matrix, &cmap);
/// let svg_str = htm.to_svg();
/// fs::write("mapa.svg", &svg_str).map_err(|e| e.to_string())?;
/// # Ok(())
/// # }
///
/// ```
pub struct Heatmap {
    pub id: ElementID,
    pub offset_x: f32,
    pub offset_y: f32,
    pub(crate) drawing: SvgDrawing, // so the builder can set this field
    pub(crate) boxes: Vec<SvgElement>,
}

impl Heatmap {
    pub fn from_matrix(mut drawing: SvgDrawing, id: impl Into<ElementID>,
            box_width: f32, box_height: f32, data: Vec<Vec<f64>>, cmap: &ColorMap) -> Self {
        let boxes = Vec::new();

        let max_j = data.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..data.len() {
            let y = i as f32 * box_height;
            for j in 0..max_j {
                let box_id = format!("r:{i}:{j}");
                let x = j as f32 * box_width;
                let r = SvgElement::rect(box_id, x, y, box_width, box_height)
                    .with_style(Style::new().fill(cmap.color(data[i][j])));
                drawing.add_element(r);
            }
        }

        Self { id: id.into(), drawing, boxes, offset_x: 0.0, offset_y: 0.0 }
    }

    pub fn to_svg(&self) -> String { self.drawing.to_svg() }
}