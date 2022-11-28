use eyre::Result;
use rsmt2d_rs::{codec, merkle_tree, rsmt2d, Matrix2D};

fn main() -> Result<()> {
    let merkle_tree = merkle_tree::new();
    let rsmt2d = rsmt2d::new(merkle_tree);

    // Instantiate original data.
    let data: Matrix2D = &vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];

    // Instantiate codec based on original data length.
    let codec = codec::new(data.len())?;

    // Compute extended data square.
    let _extended_data_square = rsmt2d.compute_extended_data_square(data, codec.as_ref())?;

    // TODO: remove some data
    let partial_data = &vec![vec![0, 0, 0, 0]];

    // Import partial data.
    let extended_data_square = rsmt2d.import_extended_data_square(partial_data, codec.as_ref())?;

    // Attempt to repair the extended data square.
    rsmt2d.repair(extended_data_square)?;

    Ok(())
}
