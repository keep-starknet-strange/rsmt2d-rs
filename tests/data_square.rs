#[cfg(test)]
mod tests {
    use eyre::Result;
    use rsmt2d_rs::{data_square::DataSquare, merkle_tree, Matrix2D};

    #[test]
    fn it_works() -> Result<()> {
        let merkle_tree = merkle_tree::new();

        let data: Matrix2D = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![8, 9, 10, 11],
        ];
        let _data_square = DataSquare::new(data, merkle_tree.as_ref())?;

        Ok(())
    }

    #[test]
    fn when_data_is_not_square_then_it_fails() {
        let merkle_tree = merkle_tree::new();

        let data: Matrix2D = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result = DataSquare::new(data, merkle_tree.as_ref());

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Number of chunks must be a square number"));
    }
}
