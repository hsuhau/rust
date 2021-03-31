fn main() {
    let fees = 25_000;

    /*
    error[E0384]: cannot assign twice to immutable variable `fees`
    --> 6_rust_variables.rs:3:5
    |
    2 |     let fees = 25_000;
    |         ----
    |         |
    |         first assignment to `fees`
    |         help: make this binding mutable: `mut fees`
    3 |     fees = 35_000;
    |     ^^^^^^^^^^^^^ cannot assign twice to immutable variable

    error: aborting due to previous error; 1 warning emitted

    For more information about this error, try `rustc --explain E0384`.
     */
    // fees = 35_000;

    let mut salary: f64 = 35_000.00;

    salary = 25_000.0;

    println!("fees is {} and salary is {}", fees, salary);
}