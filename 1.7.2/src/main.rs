fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];

    for (i, row) in matrix.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            transposed[j][i] = *x;
        }
    }

    return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        let mut first_row = false;
        for x in row {
            if !first_row {
                print!(" ");
            }

            print!("{}", x);
            first_row = false;
        }
        println!("");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
