#[cfg(test)]
mod tests {
    use eyre::Result;
    use rsmt2d_rs::{data_square::DataSquare, Matrix2D};

    #[test]
    fn it_works() -> Result<()> {
        let data: Matrix2D = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ];
        let data_square = DataSquare::new(data)?;
        assert!(data_square.chunk_size == 4);
        assert!(data_square.width == 2);
        assert!(
            data_square.square_row
                == vec![
                    vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]],
                    vec![vec![8, 9, 10, 11], vec![12, 13, 14, 15]],
                ]
        );
        assert!(
            data_square.square_col
                == vec![
                    vec![vec![0, 1, 2, 3], vec![8, 9, 10, 11]],
                    vec![vec![4, 5, 6, 7], vec![12, 13, 14, 15],]
                ]
        );

        Ok(())
    }

    #[test]
    fn when_data_is_empty_then_it_fails() {
        let data: Matrix2D = vec![];
        let result = DataSquare::new(data);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(err.to_string(), "Data must not be empty."),
        }
    }

    #[test]
    fn when_data_is_not_square_then_it_fails() {
        let data: Matrix2D = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result = DataSquare::new(data);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert!(err
                .to_string()
                .contains("Number of chunks must be a square number.")),
        };
    }

    #[test]
    fn erasure_extend_square_works() -> Result<()> {
        let data: Matrix2D = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ];
        let mut data_square = DataSquare::new(data).unwrap();
        data_square.erasure_extend_square()?;
        println!("{:?}", data_square.square_row);
        println!("{:?}", data_square.square_col);
        /*assert!(
            data_square.square_row
                == vec![
                    [[0, 1, 2, 3], [4, 5, 6, 7], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[8, 9, 10, 11], [12, 13, 14, 15], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
                ]
        );
        assert!(
            data_square.square_col
                == vec![
                    [[0, 1, 2, 3], [8, 9, 10, 11], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[4, 5, 6, 7], [12, 13, 14, 15], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
                ]
        );*/
        Ok(())
    }
}
