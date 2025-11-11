# Summary
An implementation of a [`Matrix`](https://en.wikipedia.org/wiki/Matrix_(mathematics)).
- Dimensions are checked at runtime
- no panics
- data is backed by `Box<[Box<[E]>]>` created from [`Vec::into_boxed_slice`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice)
- operations are only implemented on types that implement the necessary traits from [`num::traits`](https://docs.rs/num/latest/num/index.html#traits) or [`std::ops`](https://doc.rust-lang.org/std/ops/index.html#traits)
- Custom error type `MatrixError` enumerates all errors with metadata

## Matrix Operations
- [transpose](https://en.wikipedia.org/wiki/Transpose)
- [matrix multiply]()
- hadamard (element-wise) multiply
- scalar multiply
- add
- [minor](https://en.wikipedia.org/wiki/Minor_(linear_algebra))
- [cofactor](https://en.wikipedia.org/wiki/Minor_(linear_algebra))
- matrix of cofactors
- determinant
- inverse

## Regression
- Least Square Regression implemented for a set of `Data`
    - polynomials
    - TODO: Rational functions
    - TODO: Exponential/Logarithmic functions
