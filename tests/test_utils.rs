mod test_utils {
    #[test]
    fn single_linspace() {
        use visualife::plots::linspace;
        assert!(linspace(0, 0.0, 1.0, false).is_empty());
        assert_eq!(linspace(1, 7.0, 9.0, false), vec![7.0]);

        let v = linspace(1, 0.0, 2.0, true);
        assert!((v[0] - 1.0).abs() < 1e-6); // (2-0)/(1+1)=1 => 0+1*1=1

        let v30 = linspace(30, -15.0, 15.0, false);
        assert!((v30[0]+15.0).abs() < 1e-4);
        assert!((v30[29]-15.0).abs() < 1e-4);
    }
}