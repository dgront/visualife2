use data_matrix::DataMatrix;

use crate::basic_shapes::{SvgElement};
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

    /// Creates a heatmap from data given as a 2D Vec of numerical values.
    ///
    /// # Example
    /// ```
    /// # use visualife::heatmap::Heatmap;
    /// # use rand::{SeedableRng, Rng};
    /// # use rand::rngs::StdRng;
    /// let mut rng = StdRng::seed_from_u64(0);
    /// let vec2d: Vec<Vec<f64>> = (0..5)
    ///     .map(|_| (0..7).map(|_| rng.random::<f64>()).collect())
    ///     .collect();
    /// let mut htm = Heatmap::from_vec2d("heatmap", 20.0, 20.0, vec2d);
    /// # assert_eq!(htm.count_rows(), 5);
    /// # assert_eq!(htm.count_columns(), 7);
    /// ```
    pub fn from_vec2d(id: impl Into<ElementID>, box_width: f32,box_height: f32, data: Vec<Vec<f64>>) -> Self {

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

    /// Creates a heatmap from data given as a matrix of numerical values.
    ///
    /// # Example
    /// ```
    /// # use visualife::heatmap::Heatmap;
    /// # use rand::{SeedableRng, Rng};
    /// # use rand::rngs::StdRng;
    /// let mut rng = StdRng::seed_from_u64(0);
    /// let matrix: Vec<Vec<f64>> = (0..5)
    ///     .map(|_| (0..7).map(|_| rng.random::<f64>()).collect())
    ///     .collect();
    /// let mut htm = Heatmap::from_matrix("heatmap", 20.0, 20.0, matrix);
    /// # assert_eq!(htm.count_rows(), 5);
    /// # assert_eq!(htm.count_columns(), 7);
    /// ```
    pub fn from_matrix<I, R, T>(id: impl Into<ElementID>, box_width: f32,box_height: f32, data: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<f64>
    {
        let data: Vec<Vec<f64>> = data
                    .into_iter()
                    .map(|row| row.into_iter().map(Into::into).collect())
                    .collect();

        return Heatmap::from_vec2d(id, box_width, box_height, data);
    }

    /// Creates a heatmap from [`DataMatrix`] struct
    ///
    /// # Example
    /// ```
    /// use visualife::heatmap::Heatmap;
    /// use visualife::SvgDrawing;
    /// use data_matrix::DataMatrixBuilder;
    /// # use data_matrix::Error;
    /// # fn main() -> Result<(), Error> {
    /// let dm = DataMatrixBuilder::new()
    ///     .label_columns(0, 1)
    ///     .index_columns(3, 4)
    ///     .data_column(2)
    ///     .symmetric(true)
    ///     .skip_header(true)
    ///     .separator(',')
    ///     .from_file("tests/test_inputs/cities_by_distance.csv")?;
    /// let mut htm = Heatmap::from_datamatrix("heatmap", 20.0, 20.0, &dm);
    /// htm.offset_x = 80.0;
    /// let mut drawing = SvgDrawing::new(800.0, 800.0);
    /// drawing.add_element(htm.create_element());
    /// drawing.save_svg("cities.svg");
    /// # assert_eq!(htm.count_rows(), 15);
    /// # assert_eq!(htm.count_columns(), 15);
    /// # assert!(htm.row_labels().is_some());
    /// # assert!(htm.col_labels().is_some());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_datamatrix(id: impl Into<ElementID>,box_width: f32,box_height: f32, data_matrix: &DataMatrix) -> Self {
        let (min, max) = data_matrix.data()
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
            data: data_matrix.data().clone(),
            row_labels: Some(data_matrix.row_labels().to_vec()),
            col_labels: Some(data_matrix.col_labels().to_vec()),
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

    /// Creates an SVG group element that contains all graphical components representing this heatmap
    pub fn create_element(&self) -> SvgElement {

        let mut heatmap_groups = Vec::with_capacity(3);

        let x_offset_new = self.offset_x;

        // --- Group of boxes
        let mut boxes = vec![];
        let n_rows = self.data.len();
        let n_cols = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..n_rows {
            let y = i as f32 * self.box_height + self.offset_y;
            for j in 0..n_cols {
                let box_id = format!("{}:{i}:{j}", self.id);
                let x = j as f32 * self.box_width + x_offset_new;
                let r = SvgElement::rect(box_id, x, y, self.box_width, self.box_height)
                    .with_style(Style::new().fill(self.cmap.color(self.data[i][j])));
                boxes.push(r);
            }
        }
        let boxes_grp = SvgElement::group(format!("{}:boxes", self.id), boxes);
        heatmap_groups.push(boxes_grp);

        let total_h = n_rows as f32 * self.box_height;
        let pad = self.box_height.min(self.box_width) * 0.25; // --- Small padding for labels

        // --- Row labels (left side, right-aligned, vertically centered per row)
        if let Some(labels) = &self.row_labels {
            let mut rlbls = Vec::with_capacity(n_rows);
            for i in 0..n_rows {
                let y_center = self.offset_y + i as f32 * self.box_height + 0.5 * self.box_height;
                let x_left = x_offset_new - pad;

                let text_id = format!("{}:r{}", self.id, i);
                let t = SvgElement::text(text_id, x_left, y_center, &labels[i])
                    .with_style(Style::new().dominant_baseline("middle"), // center on cell vertically
                    );
                rlbls.push(t);
            }
            let rows_style = Style::new().text_anchor("end").font_size("15");
            heatmap_groups.push(SvgElement::group(format!("{}:rows", self.id), rlbls).with_style(rows_style));
            // --- modify the offset_x field if it's still 0.0 to make room for the labels
            // let font_size = 15.0;
            // self.offset_x = labels.iter().map(|l| estimate_text_width(l, font_size)).reduce(f32::max).unwrap_or(0.0);
        }

        // --- Column labels (bottom, vertical)
        if let Some(labels) = &self.col_labels {
            let mut clbls = Vec::with_capacity(n_cols);
            for j in 0..n_cols {
                let x_center = x_offset_new + j as f32 * self.box_width + 0.5 * self.box_width;
                let y_bottom = self.offset_y + total_h + pad;
                let text_id = format!("{}:c{:}", self.id, j);
                let t = SvgElement::text(text_id, x_center, y_bottom, &labels[j])
                    .with_style(Style::new().dominant_baseline("central"))
                    .with_transform(format!("rotate(-90 {} {})", x_center, y_bottom));
                clbls.push(t);
            }
            let cols_style = Style::new().text_anchor("end").font_size("15");
            heatmap_groups.push(SvgElement::group(format!("{}:cols", self.id), clbls).with_style(cols_style));
        }

        SvgElement::group(format!("{}:heatmap", self.id), heatmap_groups)
    }
}

impl From<&Heatmap> for SvgElement {
    /// Creates an SVG group that contains all graphical elements for this heatmap

    fn from(h: &Heatmap) -> Self { h.create_element() }
}
