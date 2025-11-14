mod testing_utilities; // Declare the module

#[cfg(test)]
mod test_heatmap {
    use data_matrix::DataMatrixBuilder;
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use visualife::heatmap::Heatmap;
    use visualife::SvgDrawing;
    use crate::testing_utilities::load_expected_svgs;

    #[test]
    fn build_from_array_like() {
        let hm1 = Heatmap::from_matrix("hm1", 10.0, 10.0, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(hm1.count_rows(), 2);

        let hm2 = Heatmap::from_matrix("hm2", 10.0, 10.0, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        assert_eq!(hm2.count_columns(), 3);

        let hm3 = Heatmap::from_matrix(
            "hm3",
            10.0,
            10.0,
            [[1f32, 2f32], [3f32, 4f32], [3f32, 4f32]],
        );
        assert_eq!(hm3.count_rows(), 3);
        assert_eq!(hm3.count_columns(), 2);

        let hm4 = Heatmap::from_matrix("hm4", 10.0, 10.0, [[1, 2, 1], [3, 4, 3]]); // integers also work
        assert_eq!(hm4.count_rows(), 2);
    }

    #[test]
    fn simple_map_with_labels() {
        let matrix: [[f64; 3]; 3] = [[1.0, 2.0, 3.0], [4.5, 5.5, 6.5], [7.1, 8.2, 9.3]];
        let mut hm = Heatmap::from_matrix("hm1", 10.0, 10.0, matrix);
        assert!(hm.set_row_labels(["A", "B", "C"]).is_ok());
        assert!(hm.set_col_labels(["1", "2", "3"]).is_ok());
        assert_eq!(hm.row_labels().as_ref().unwrap()[1], "B");
    }

    #[test]
    fn draw_heatmap_with_labels() -> anyhow::Result<()> {

        let mut drawing = SvgDrawing::new(300.0, 300.0);
        let mut rng = StdRng::seed_from_u64(0);
        let matrix: Vec<Vec<f64>> = (0..7)
            .map(|_| (0..7).map(|_| rng.random::<f64>()).collect())
            .collect();
        let mut htm = Heatmap::from_matrix("heatmap", 20.0, 20.0, matrix);
        htm.offset_x = 100.0;
        htm.offset_y = 50.0;
        htm.set_row_labels(["row A", "long name B", "row C", "row D", "row E", "row F", "row G"])?;
        htm.set_col_labels(["col 1", "col 2", "long name 3", "col 4", "col 5", "col 6", "col 7"])?;
        drawing.add_element(htm.create_element());
        let expected = load_expected_svgs("./tests/expected_drawings/heatmap/", &["labelled_map.svg"])?;
        // drawing.save_svg("labelled_map.svg")?;
        assert_eq!(drawing.to_svg(), expected[0]);
        Ok(())
    }

    #[test]
    fn cities_heatmap_from_datamatrix() -> Result<(), anyhow::Error> {

        let dmap = DataMatrixBuilder::new()
            .label_columns(0, 1)
            .data_column(2)
            .index_columns(3, 4)
            .skip_header(true)
            .symmetric(true)
            .from_file("./tests/test_inputs/cities_by_distance.csv")?;
        let mut htm = Heatmap::from_datamatrix("heatmap", 20.0, 20.0, &dmap);
        htm.offset_x = 110.0;
        htm.offset_y = 50.0;
        let mut drawing = SvgDrawing::new(460.0, 460.0);
        drawing.add_element(htm.create_element());
        // drawing.save_svg("cities.svg")?;
        let expected = load_expected_svgs("./tests/expected_drawings/heatmap/", &["cities.svg"])?;
        assert_eq!(drawing.to_svg(), expected[0]);
        Ok(())
    }
}
