A simple test project for 2d matrix operations

# Features
- Elements are store on the heap as `Box<[Box<[Element]>]>`
  - TODO: double check that the elements created using <br>
        ```
            vec![
                vec![Element::zero(); WIDTH].into_boxed_slice();
                HEIGHT
            ].into_boxed_slice()
        ``` <br>
    are contiguous in memory
- Matrix Operations
  - Addition
    - Of same size matrices
  - Multiplication
    - Matrix multiplication (Sum of dot products between lhs rows and rhs columns)
    - Hadamard multiplication (Element-wise multiplication)
  - Determinant
    - Minor
    - Cofactor
  - Multiplicative Inverse
  - A linear regresssion trait
