#[cfg(test)]
mod test_heatmap {
    use visualife::heatmap::Heatmap;

    #[test]
    fn build_from_array_like() {
        let hm1 = Heatmap::from_matrix("hm1", 10.0, 10.0, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(hm1.count_rows(), 2);

        let hm2 = Heatmap::from_matrix("hm2", 10.0, 10.0, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        assert_eq!(hm2.count_columns(), 3);

        let hm3 = Heatmap::from_matrix("hm3", 10.0, 10.0, [[1f32, 2f32], [3f32, 4f32], [3f32, 4f32]]);
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
        assert_eq!(hm.row_labels().as_ref().unwrap()[1],"B");
    }
}
