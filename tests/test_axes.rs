mod test_axes {
    use visualife::plots::{AxisIntercept, AxisSet, Box2D, TickDirection};
    use visualife::SvgDrawing;


    #[test]
    fn draw_axes_xy_centered() {
        let mut axes = AxisSet::new((25.0,225.0, 25.0, 125.0));
        axes.has_arrowhead = true;
        axes.set_intercept_point(0.0, 0.0);
        axes.set_plot_box((-1.0, 1.0, -1.0, 1.0));
        axes.x.set_nticks(5);
        axes.y.set_nticks(5);
        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_element(axes);
        drawing.save_svg("axes_xy_cent.svg").unwrap();
    }

    #[test]
    fn draw_axes_xy() {
        let mut axes = AxisSet::new((25.0,225.0, 25.0, 125.0));
        axes.has_arrowhead = true;
        axes.set_intercept(AxisIntercept::AutoStart, AxisIntercept::AutoStart);
        axes.set_plot_box((0.0, 10.0, 0.0, 100.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(4);
        axes.y.tics_location = TickDirection::UpLeft;
        axes.y.label_location = TickDirection::UpLeft;
        axes.x.tics_location = TickDirection::DownRight;
        axes.x.label_location = TickDirection::DownRight;
        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_element(axes);
        drawing.save_svg("axes_xy.svg").unwrap();
    }
}