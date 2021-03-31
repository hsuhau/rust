fn main() {
    let v = vec![10, 20, 30];
    print_vector(v)；
    /*
    [Running] cd "/home/hsuhau/GitHub/hsuhau/rust/src/16_rust_borrowing/" && rustc 16_rust_brrowing.rs && "/home/hsuhau/GitHub/hsuhau/rust/src/16_rust_borrowing/"16_rust_brrowing
    error: unknown start of token: \u{ff1b}
    --> 16_rust_brrowing.rs:3:20
    |
    3 |     print_vector(v)；
    |                    ^^
    |
    help: Unicode character '；' (Fullwidth Semicolon) looks like ';' (Semicolon), but it is not
    |
    3 |     print_vector(v);
    |                    ^

    error: aborting due to previous error
    */
    // println!("{}", v[0]);
}

fn print_vector(x: Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}