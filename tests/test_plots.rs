mod test_plots {
    use visualife::plots::{AxisSetBuilder, Plot, TicsLocation};
    use visualife::SvgDrawing;

    #[test]
    fn plot_heatmap() -> std::io::Result<()> {
        let mut drawing = SvgDrawing::new(500.0, 500.0);

        let axes = AxisSetBuilder::new("LBTR",(50.0, 450.0, 50.0, 450.0))
            .data_range((-1.0, 1.0, -1.0, 1.0))
            .tics_location(TicsLocation::OUTER)
            .ntics(4).font_size(12.0).build();
        let mut plot = Plot::new("heatmap",axes,400.0,400.0);

        let data = make_grid(15.0, 30);
        plot.heatmap(data);
        drawing.add_element(plot.create_element());
        drawing.save_svg("htmp_plot.svg")?;

        return Ok(());
    }

    /// Build an N×N grid:
    /// x,y in [-15, 15], r = hypot(x,y), value = cos(r) * exp(-r/4).
    pub fn make_grid(v_max: f64, n: usize) -> Vec<Vec<f64>> {
        assert!(n >= 2, "N must be >= 2");
        let step = v_max * 2.0 / (n as f64 - 1.0);

        (0..n)
            .map(|i| {
                let y = -v_max + i as f64 * step;
                (0..n)
                    .map(|j| {
                        let x = -v_max + j as f64 * step;
                        let r = x.hypot(y);
                        r.cos() * (-r / 4.0).exp()
                    })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>()
    }
}

