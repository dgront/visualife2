mod test_plots {
    use rand::rngs::StdRng;
    use rand::{SeedableRng};
    use rand_distr::{Distribution, Normal};
    use visualife::plots::{linspace, Plot, PlotError};
    use visualife::SvgDrawing;

    #[test]
    fn plot_heatmap() -> Result<(), PlotError> {
        let width = 500.0;
        let mut drawing = SvgDrawing::new(width, width);
        let m = 75.0;   // --- plot margin
        let mut plot = Plot::rectangular("heatmap", (0.0 + m, width - m, 0.0 + m, width - m));
        plot.set_nticks(5);
        let data = make_grid(15.0, 30);
        let x = linspace(30, -15.0, 15.0, false);
        let y = linspace(30, -15.0, 15.0, false);
        plot.heatmap(&x, &y, data)?;
        drawing.add_element(plot.create_element());
        let fname = "htmp_plot.svg";
        drawing.save_svg(fname)
            .map_err(|e| PlotError::CantWritePlotImage{ filename: fname.to_string(), reason: e.to_string()})?;

        return Ok(());
    }

    #[test]
    fn plot_scatter() -> Result<(), PlotError> {
        let mut plot = Plot::cartesian("sin", (50.0, 650.0, 50.0, 450.0));
        let x = linspace(30, -3.1415, 3.1415, false);
        let y: Vec<f32>  = x.iter().map(|x| x.sin()).collect();
        plot.scatter(&x, &y);
        plot.set_plot_box((-3.2, 3.2, -1.0, 1.0));

        let mut drawing = SvgDrawing::new(700.0, 500.0);
        drawing.add_element(plot.create_element());
        let fname = "scatter_plot.svg";

        drawing.save_svg(fname)
            .map_err(|e| PlotError::CantWritePlotImage{ filename: fname.to_string(), reason: e.to_string()})?;
        Ok(())
    }

    #[test]
    fn plot_scatter_box() -> Result<(), PlotError> {
        let mut rng = StdRng::seed_from_u64(0);

        let n = 500;
        let normal_x = Normal::new(0.0, 1.0).unwrap();
        let normal_y = Normal::new(0.0, 1.0).unwrap();
        let x: Vec<f32> = (0..n).map(|_| { normal_x.sample(&mut rng) as f32 }).collect();
        let y: Vec<f32> = (0..n).map(|_| { normal_y.sample(&mut rng) as f32 }).collect();

        let mut plot = Plot::rectangular("rnd", (75.0, 625.0, 75.0, 625.0));
        plot.set_plot_box((-4.0, 4.0, -4.0, 4.0));
        plot.set_nticks(3);
        plot.axes_mut().draw_grid = true;
        plot.scatter(&x, &y);

        let mut drawing = SvgDrawing::new(700.0, 700.0);
        drawing.add_element(plot.create_element());
        let fname = "scatter_box.svg";

        drawing.save_svg(fname)
            .map_err(|e| PlotError::CantWritePlotImage{ filename: fname.to_string(), reason: e.to_string()})?;
        Ok(())
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

