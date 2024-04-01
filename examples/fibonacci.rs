fn main() {
    for i in 0..20 {
        println!("{}", fibonacci(i));
    }
}
fn fibonacci(n: usize) -> usize {
    return match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    };
}
