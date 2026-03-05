fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            transposed[j][i] = col;
        }
    }
    transposed
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}
