use crate::basic_shapes::SvgElement;
use crate::heatmap::HeatmapError;
use crate::styling::{ColorMap, Style};
use crate::ElementID;

/// Represents a color map based on a rectangular matrix of real values.
///
/// Here is the code that generates the image:
#[doc = include_str!("../../tests/expected_drawings/heatmap/labelled_map.svg")]
///
/// ```
/// use visualife::SvgDrawing;
/// use visualife::heatmap::Heatmap;
/// # use rand::{SeedableRng, Rng};
/// # use rand::rngs::StdRng;
/// # fn main() -> anyhow::Result<()> {
/// let mut drawing = SvgDrawing::new(300.0, 300.0);
/// let mut rng = StdRng::seed_from_u64(0);
/// let matrix: Vec<Vec<f64>> = (0..7)
///     .map(|_| (0..7).map(|_| rng.random::<f64>()).collect())
///     .collect();
/// let mut htm = Heatmap::from_matrix("heatmap", 20.0, 20.0, matrix);
/// htm.offset_x = 100.0;
/// htm.offset_y = 50.0;
/// htm.set_row_labels(["row A", "long name B", "row C", "row D", "row E", "row F", "row G"])?;
/// htm.set_col_labels(["col 1", "col 2", "long name 3", "col 4", "col 5", "col 6", "col 7"])?;
/// drawing.add_element(htm.create_element());
/// # Ok(())
/// # }
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

    pub fn create_element(&self) -> SvgElement {

        let mut heatmap_groups = Vec::with_capacity(3);

        // --- Group of boxes
        let mut boxes = vec![];
        let n_rows = self.data.len();
        let n_cols = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..n_rows {
            let y = i as f32 * self.box_height + self.offset_y;
            for j in 0..n_cols {
                let box_id = format!("{}:{i}:{j}", self.id);
                let x = j as f32 * self.box_width + self.offset_x;
                let r = SvgElement::rect(box_id, x, y, self.box_width, self.box_height)
                    .with_style(Style::new().fill(self.cmap.color(self.data[i][j])));
                boxes.push(r);
            }
        }
        let boxes_grp = SvgElement::group(format!("{}:boxes", self.id), boxes);
        heatmap_groups.push(boxes_grp);

        let total_h = n_rows as f32 * self.box_height;
        let pad = self.box_height.min(self.box_width) * 0.25; // --- Small padding for labels

        // --- 2) Row labels (left side, right-aligned, vertically centered per row) ===
        if let Some(labels) = &self.row_labels {
            let mut rlbls = Vec::with_capacity(n_rows);
            for i in 0..n_rows {
                let y_center = self.offset_y + i as f32 * self.box_height + 0.5 * self.box_height;
                let x_left = self.offset_x - pad;

                let text_id = format!("{}:r{}", self.id, i);
                let t = SvgElement::text(text_id, x_left, y_center, &labels[i])
                    .with_style(
                        Style::new()
                            .text_anchor("end")           // align text end to x_left
                            .dominant_baseline("middle"), // center on cell vertically
                    );
                rlbls.push(t);
            }
            heatmap_groups.push(SvgElement::group(format!("{}:rows", self.id), rlbls));
        }

        // --- Column labels (bottom, vertical)
        if let Some(labels) = &self.col_labels {
            let mut clbls = Vec::with_capacity(n_cols);
            for j in 0..n_cols {
                let x_center = self.offset_x + j as f32 * self.box_width + 0.5 * self.box_width;
                let y_bottom = self.offset_y + total_h + pad;
                let text_id = format!("{}:c{:}", self.id, j);
                let t = SvgElement::text(text_id, x_center, y_bottom, &labels[j])
                    .with_style(Style::new().text_anchor("end").dominant_baseline("central"))
                    .with_transform(format!("rotate(-90 {} {})", x_center, y_bottom));
                clbls.push(t);
            }
            heatmap_groups.push(SvgElement::group(format!("{}:cols", self.id), clbls));
        }

        SvgElement::group(format!("{}:heatmap", self.id), heatmap_groups)
    }
}
