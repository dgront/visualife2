use crate::basic_shapes::SvgElement;
use crate::heatmap::HeatmapError;
use crate::styling::{ColorMap, Style};
use crate::ElementID;

///
/// ```
/// # use std::fs;
/// use rand::Rng;
/// use visualife::heatmap::Heatmap;
/// use visualife::styling::ColorMap;
/// use visualife::styling::palettes::RED_BLUE;
/// use visualife::SvgDrawing;
/// # fn main()-> Result<(), String> {
/// let mut drawing = SvgDrawing::new(300.0, 300.0);
/// # let mut rng = rand::thread_rng();
/// let matrix: Vec<Vec<f64>> = (0..10).map(|_| (0..10).map(|_| rng.gen::<f64>()).collect()).collect();
/// let mut htm = Heatmap::from_matrix("heatmap", 20.0, 20.0, matrix);
/// htm.offset_x = 50.0;
/// htm.offset_y = 50.0;
/// drawing.add_element( htm.create_elements() );
/// let svg_str = drawing.to_svg();
/// fs::write("mapa.svg", &svg_str).map_err(|e| e.to_string())?;
/// # Ok(())
/// # }
///
/// ```
pub struct Heatmap {
    pub id: ElementID,
    pub box_width: f32,
    pub box_height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub cmap: ColorMap,
    data: Vec<Vec<f64>>,
    row_labels: Option<Vec<String>>,
    col_labels: Option<Vec<String>>,
}

impl Heatmap {
    pub fn from_matrix<I, R, T>(id: impl Into<ElementID>,box_width: f32,box_height: f32, data: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<f64>
    {
        let data: Vec<Vec<f64>> = data
                    .into_iter()
                    .map(|row| row.into_iter().map(Into::into).collect())
                    .collect();

        let (min, max) = data
            .iter()
            .flatten() // turns &Vec<Vec<f64>> into a single iterator over &f64
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &x| {
                (min.min(x), max.max(x))
            });

        Self {
            id: id.into(),
            box_width,
            box_height,
            offset_x: 0.0,
            offset_y: 0.0,
            cmap: ColorMap::from_range(&crate::styling::palettes::RED_BLUE, min, max).unwrap(),
            data,
            row_labels: None,
            col_labels: None,
        }
    }

    pub fn count_rows(&self) -> usize { self.data.len() }

    pub fn row_labels(&self) -> &Option<Vec<String>> { &self.row_labels }

    pub fn set_row_labels<L, S>(&mut self, labels: L) -> Result<(), HeatmapError>
    where
        L: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v: Vec<String> = labels.into_iter().map(Into::into).collect();
        if v.len() != self.data.len() {
            return Err(HeatmapError::IncorrectNumberOfLabels {
                n_labels_expected: self.data.len(),
                n_labels_found: v.len(),
            });
        }
        self.row_labels = Some(v);
        Ok(())
    }

    pub fn count_columns(&self) -> usize { self.data[0].len() }

    pub fn col_labels(&self) -> &Option<Vec<String>> {
        &self.col_labels
    }

    pub fn set_col_labels<L, S>(&mut self, labels: L) -> Result<(), HeatmapError>
    where
        L: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v: Vec<String> = labels.into_iter().map(Into::into).collect();
        if v.len() != self.data.len() {
            return Err(HeatmapError::IncorrectNumberOfLabels {
                n_labels_expected: self.data.len(),
                n_labels_found: v.len(),
            });
        }
        self.col_labels = Some(v);
        Ok(())
    }

    pub fn create_elements(&self) -> SvgElement {
        let mut boxes = vec![];
        let max_j = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..self.data.len() {
            let y = i as f32 * self.box_height + self.offset_y;
            for j in 0..max_j {
                let box_id = format!("{}:{i}:{j}", self.id);
                let x = j as f32 * self.box_width + self.offset_x;
                let r = SvgElement::rect(box_id, x, y, self.box_width, self.box_height)
                    .with_style(Style::new().fill(self.cmap.color(self.data[i][j])));
                boxes.push(r);
            }
        }
        let grp = SvgElement::group(format!("{}:boxes", self.id), boxes);

        grp
    }
}
