use num::Num;

fn main() {}

fn matrix_multiply<E: Num + Copy, const LH: usize, const LW_RH: usize, const RW: usize>(
    lhs: [[E; LW_RH]; LH],
    rhs: [[E; RW]; LW_RH],
) -> [[E; RW]; LH] {
    let mut product = [[E::zero(); RW]; LH];

    for lhs_row_index in 0..LH {
        for rhs_col_index in 0..RW {
            
            let mut dot_product = E::zero();

            for element_index in 0..LW_RH {

                let lhs_element = lhs[lhs_row_index][element_index];
                let rhs_element = rhs[element_index][rhs_col_index];

                let element_product = lhs_element * rhs_element;

                dot_product = dot_product + element_product;
            }

            product[lhs_row_index][rhs_col_index] = dot_product;
        }
    }

    return product;
}

fn matrix_to_csv_string<E: Num + std::fmt::Display, const W: usize, const H: usize>(
    matrix: [[E; W]; H],
) -> String {
    let mut output = String::new();

    for row in matrix {
        for element in row {
            output.push_str(format!("{}, ", element).as_str());
        }
        output.push('\n');
    }

    return output;
}

#[test]
fn mat_mul() {
    let lhs = [
        [1, 2, 3], //
        [1, 2, 3],
        [4, 5, 6],
        [4, 5, 6],
    ];

    let rhs = [
        [7, 8, 7, 8], //
        [9, 10, 9, 10],
        [11, 12, 11, 12],
    ];

    let expected_product = [
        [58, 64, 58, 64], //
        [58, 64, 58, 64],
        [139, 154, 139, 154],
        [139, 154, 139, 154],
    ];

    let product = matrix_multiply(lhs, rhs);

    println!(
        "{}\n{}",
        matrix_to_csv_string(product),
        matrix_to_csv_string(expected_product)
    );

    assert!(product == expected_product);
}
