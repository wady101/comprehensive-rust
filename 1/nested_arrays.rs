fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // let mut new_matrix: [[i32; 3]; 3];
    // we're partially initialising it here so we can't do it.
    let mut new_matrix = [[0; 3]; 3];
    for row in matrix
    {
    dbg!(row[0]);
    }
    for row in 0..3
    {
        for col in 0..3
        {
            new_matrix[col][row] = matrix[row][col]
        }
    }
    new_matrix
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}
