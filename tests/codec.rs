#[cfg(test)]
mod tests {
    use eyre::Result;
    use rsmt2d_rs::{codec, Matrix2D};

    #[test]
    fn it_works() -> Result<()> {
        let data: Matrix2D = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ];
        let codec = codec::new(data.len())?;
        let input = data;
        let encoded_data = codec.encode(input)?;
        assert!(
            encoded_data
                == vec![
                    vec![16, 17, 18, 19],
                    vec![20, 21, 22, 23],
                    vec![24, 25, 26, 27],
                    vec![28, 29, 30, 31],
                ]
        );
        Ok(())
    }
}
