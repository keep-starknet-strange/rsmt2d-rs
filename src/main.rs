use eyre::Result;
use rsmt2d_rs::{codec, merkle_tree, rsmt2d, Matrix2D};

fn main() -> Result<()> {
    let merkle_tree = merkle_tree::new();
    let rsmt2d = rsmt2d::new(merkle_tree);
    let data: Matrix2D = &vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
    let codec = codec::new(data.len())?;
    let extended_data_square = rsmt2d.compute_extended_data_square(data, codec.as_ref())?;

    rsmt2d.import_extended_data_square(extended_data_square, codec.as_ref())?;

    Ok(())
}
