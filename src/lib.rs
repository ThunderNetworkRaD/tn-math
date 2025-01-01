#[cfg(feature = "algebra")]
pub mod algebra;

    #[test]
    fn sum() {
        let result = sum!(3, -1, 2);
        assert_eq!(result, 4);
    }
}
