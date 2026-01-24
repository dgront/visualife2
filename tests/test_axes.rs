mod test_axes {
    use visualife::plots::{AxisBuilder, AxisSetBuilder, AxisSide, Box2D};
    use visualife::SvgDrawing;

    #[test]
    fn single_axes() {
        let x = AxisBuilder::new(AxisSide::BOTTOM, 0.0, 100.0)
            .plot_range(2.0, 5.0).build();
        assert_eq!(x.plot_range().0,2.0);
        assert_eq!(x.plot_range().1,5.0);
        let y = AxisBuilder::new(AxisSide::LEFT, 0.0, 100.0)
            .plot_range_from_data(&[1.01, 2.5, 8.9]).build();
        assert_eq!(y.plot_range().0,0.0);
        assert_eq!(y.plot_range().1,10.0);
    }

    #[test]
    fn draw_axes_xy() {
        let axes = AxisSetBuilder::new((25.0,225.0, 25.0, 125.0))
            .axes("LB").arrowheads(true).ntics(5)
            .data_range((-1.0, 1.0, -1.0, 1.0)).center(0.0, 0.0).build();
        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_element(axes.create_element());
        drawing.save_svg("axes_xy.svg").unwrap();
    }

    #[test]
    fn draw_axes_box() {
        let axes = AxisSetBuilder::new((25.0,225.0, 25.0, 125.0))
            .axes("LBTR").arrowheads(false).ntics(7)
            .data_range((-1.0, 1.0, -1.0, 1.0)).center(0.0, 0.0).build();
        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_element(axes.create_element());
        drawing.save_svg("axes_box.svg").unwrap();
    }
}