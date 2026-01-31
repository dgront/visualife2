mod test_axes {
    use std::io;
    use visualife::basic_shapes::{embed_vl_font, VlFont};
    use visualife::plots::{AxisIntercept, AxisSet, TickDirection};
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
        axes.y.ticks_location = TickDirection::UpLeft;
        axes.y.label_location = TickDirection::UpLeft;
        axes.x.ticks_location = TickDirection::DownRight;
        axes.x.label_location = TickDirection::DownRight;
        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_element(axes);
        drawing.save_svg("axes_xy.svg").unwrap();
    }

    #[test]
    fn draw_axes_rectangular() -> io::Result<()> {
        let mut axes = AxisSet::new((25.0,225.0, 25.0, 125.0));
        axes.has_arrowhead = false;
        axes.set_intercept(AxisIntercept::AutoStart, AxisIntercept::AutoStart);
        axes.set_plot_box((0.0, 10.0, 0.0, 100.0));
        axes.x.set_nticks(7);
        axes.y.set_nticks(4);
        axes.y.ticks_location = TickDirection::UpLeft;
        axes.y.label_location = TickDirection::UpLeft;
        axes.x.ticks_location = TickDirection::DownRight;
        axes.x.label_location = TickDirection::DownRight;

        let mut axes2 = AxisSet::new((25.0,225.0, 25.0, 125.0));
        axes2.has_arrowhead = false;
        axes2.set_intercept(AxisIntercept::AutoEnd, AxisIntercept::AutoEnd);
        axes2.set_plot_box((0.0, 10.0, 0.0, 100.0));
        axes2.x.set_nticks(7);
        axes2.y.set_nticks(4);
        axes2.y.show_ticks_labels = false;
        axes2.x.show_ticks_labels = false;
        axes2.y.ticks_location = TickDirection::DownRight;
        axes2.y.label_location = TickDirection::DownRight;
        axes2.x.ticks_location = TickDirection::UpLeft;
        axes2.x.label_location = TickDirection::UpLeft;

        let mut drawing = SvgDrawing::new(250.0, 150.0);
        drawing.add_definition(embed_vl_font(VlFont::DejaVuSansRegular, None)?);
        drawing.add_element(axes);
        drawing.add_element(axes2);
        drawing.save_svg("axes_rect.svg")?;

        Ok(())
    }
}