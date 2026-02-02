use crate::basic_shapes::SvgElement;
use crate::ElementID;
use crate::heatmap::Heatmap;
use crate::plots::{AxisIntercept, AxisSet, Box2D, MarkerType, matrix_shape, nice_plot_box, PLOT_FONT_FAMILY, PLOT_FONT_WEIGHT, PlotError, point_outside_box, TickDirection, update_plot_box};
use crate::styling::{darker, Style};

use crate::styling::palettes::{ACCENT};
use crate::utils::min_max;


pub struct Plot {
    pub id: ElementID,
    axes: AxisSet,
    axes2: Option<AxisSet>,
    screen_x0: f32,
    screen_y0: f32,
    svg_elements: Vec<SvgElement>,
    scatter_series: Vec<DataSeries>,
    line_series: Vec<DataSeries>
}

impl Plot {
    pub fn new(id: impl Into<ElementID>, axes: AxisSet) -> Self {
        Plot {
            id: id.into(),
            axes, axes2: None,
            screen_x0: 0.0, screen_y0: 0.0,
            svg_elements: vec![],
            scatter_series: vec![],
            line_series: vec![]
        }
    }

    /// Cartesian axes that cross at 0,0
    pub fn cartesian<R: Into<Box2D<f32>>>(id: impl Into<ElementID>, screen_box: R) -> Self {
        let mut axes = AxisSet::new(screen_box);
        axes.set_intercept_point(0.0, 0.0);
        axes.has_arrowhead = true;
        axes.set_intercept_point(0.0, 0.0);
        axes.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(7);

        return Plot::new(id, axes);
    }

    pub fn rectangular<R: Into<Box2D<f32>>>(id: impl Into<ElementID>, screen_box: R) -> Self {
        let screen_box = screen_box.into();
        let mut axes = AxisSet::new(screen_box.clone());
        axes.has_arrowhead = false;
        axes.set_intercept(AxisIntercept::AutoStart, AxisIntercept::AutoStart);
        axes.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(7);
        axes.y.ticks_location = TickDirection::UpLeft;
        axes.y.label_location = TickDirection::UpLeft;
        axes.x.ticks_location = TickDirection::DownRight;
        axes.x.label_location = TickDirection::DownRight;

        let mut plot = Plot::new(id, axes);

        let mut axes2 = AxisSet::new(screen_box.clone());
        axes2.set_intercept(AxisIntercept::AutoEnd, AxisIntercept::AutoEnd);
        axes2.has_arrowhead = false;
        axes2.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes2.x.set_nticks(7);
        axes2.y.set_nticks(7);
        axes2.y.ticks_location = TickDirection::DownRight;
        axes2.y.label_location = TickDirection::DownRight;
        axes2.x.ticks_location = TickDirection::UpLeft;
        axes2.x.label_location = TickDirection::UpLeft;

        plot.axes2 = Some(axes2);

        return plot;
    }

    pub fn set_plot_box<R: Into<Box2D<f32>>>(&mut self, range: R)  {
        let data_box = range.into();
        if let Some(ax2) = &mut self.axes2 { ax2.set_plot_box(data_box.clone())}
        self.axes.set_plot_box(data_box);
    }

    pub fn set_nticks(&mut self, n_ticks: usize)  {
        if let Some(ax2) = &mut self.axes2 {
            ax2.x.set_nticks(n_ticks);
            ax2.y.set_nticks(n_ticks);
        }
        self.axes.x.set_nticks(n_ticks);
        self.axes.y.set_nticks(n_ticks);
    }


    pub fn axes_mut(&mut self) -> &mut AxisSet { &mut self.axes }

    pub fn axes2_mut(&mut self) -> &mut Option<AxisSet> { &mut self.axes2 }

    pub fn translate(&mut self, offset_x: f32, offset_y: f32) {
        self.screen_x0 = offset_x;
        self.screen_y0 = offset_y;
    }

    /// Adds a series of data points to this plot
    ///
    /// # Example
    /// ```
    /// use visualife::plots::{linspace, Plot};
    /// use visualife::SvgDrawing;
    /// let mut plot = Plot::rectangular("scat", (75.0, 525.0, 75.0, 525.0));
    /// let x = linspace(30, -3.1415, 3.1415, false);
    /// let ysin: Vec<f32>  = x.iter().map(|x| x.sin()).collect();
    /// let ycos: Vec<f32>  = x.iter().map(|x| x.cos()).collect();
    /// plot.set_nticks(3);
    /// let data = plot.scatter(&x, &ysin);
    /// data.marker_size = 8.0;
    /// let data = plot.scatter(&x, &ycos);
    /// data.marker_size = 5.0;
    /// let mut drawing = SvgDrawing::new(600.0, 600.0);
    /// drawing.add_element(&plot);
    /// drawing.save_svg("scatter.svg").unwrap()
    /// ```
    pub fn scatter(&mut self, x: &[f32], y: &[f32]) -> &mut DataSeries {
        assert_eq!(x.len(), y.len(), "x and y must have the same length");

        let pbox = self.axes.plot_box();
        if point_outside_box(x, y, &pbox) {
            let mut box2d = update_plot_box(x,y,&pbox);
            box2d = nice_plot_box(&box2d);
            self.axes.set_plot_box(update_plot_box(x,y,&pbox));
        }
        if let Some(ax2) = &mut self.axes2 {
            let pbox = ax2.plot_box();
            if point_outside_box(x, y, &pbox) {
                let mut box2d = update_plot_box(x,y,&pbox);
                box2d = nice_plot_box(&box2d);
                ax2.set_plot_box(update_plot_box(x,y,&pbox));
            }
        }
        let data = x.iter()
            .copied()
            .zip(y.iter().copied())
            .collect();
        let n = self.scatter_series.len();
        self.scatter_series.push(
            DataSeries{
                data: data,
                marker: MarkerType::by_index(n),
                marker_size: 7.0,
                color: ACCENT[ n % ACCENT.len() ],
                label: format!("scatter {}", n+1)
            });
        return &mut self.scatter_series[n];
    }

    /// Plots a heatmap from a rectangular 2D dataset.
    ///
    /// Given rectangular `data` matrix is plotted as a heatmap; `x` and `y` vectors are used only
    /// to set up axes properly.
    pub fn heatmap<I, R, T>(&mut self, x: &[f32], y: &[f32], data: I) -> Result<(), PlotError>
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<f64> {

        let screen = self.axes.screen_box();
        let (min_x, max_x) = min_max(x);
        // let (min_x, max_x) = nice_plot_range(min_x, max_x);
        let (min_y, max_y) = min_max(y);
        // let (min_y, max_y) = nice_plot_range(min_y, max_y);
        self.set_plot_box((min_x, max_x, min_y, max_y));
        let data: Vec<Vec<f64>> = data
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect();

        let (n_rows, n_cols) = matrix_shape(&data);
        let htmp = Heatmap::from_matrix(format!("{}htmp", &self.id),
            screen.width() / n_cols as f32, screen.height() / n_rows as f32, data);
        let mut htmp_svg = htmp.create_element();
        htmp_svg.translate(screen.x_beg, screen.y_beg);
        self.svg_elements.push(htmp_svg);

        return Ok(());
    }

    /// Creates an SVG group element that holds all the graphical elements of this plot
    pub fn create_element(&self) -> SvgElement {
        let mut plot_components = vec![];

        // --- draw axes
        plot_components.push(self.axes.create_element());
        if let Some(ax2) = &self.axes2 {
            plot_components.push(ax2.create_element());
        }

        // --- draw explicit SVG content
        for c in &self.svg_elements {
            plot_components.push(c.clone());
        }
        // --- draw scatter series
        for ser in &self.scatter_series {
            let mut markers = vec![];
            for (i,(dx,dy)) in ser.data.iter().enumerate() {
                let (sx,sy) = self.axes.to_screen(*dx, *dy);
                markers.push(ser.marker.draw(&format!("s1{}", i), sx, sy, ser.marker_size));
            }

            let series = SvgElement::group("s1", markers)
                .with_style(Style::new().stroke(&darker(ser.color, 0.1).unwrap()).fill(ser.color));
            plot_components.push(series);
        }

        // --- group for the whole plot
        let mut plot = SvgElement::group(format!("{}", self.id), plot_components)
            .with_style(Style::new().font_family(PLOT_FONT_FAMILY).font_weight(PLOT_FONT_WEIGHT));
        plot.translate(self.screen_x0, self.screen_y0);

        return plot;
    }
}

impl From<&Plot> for SvgElement {
    /// Creates an SVG group that contains all graphical elements for this [`Plot`]

    fn from(plot: &Plot) -> Self { plot.create_element() }
}

/// Data series to be plotted
pub struct DataSeries {
    pub data: Vec<(f32,f32)>,
    pub marker: MarkerType,
    pub marker_size: f32,
    pub color: &'static str,
    pub label: String
}
