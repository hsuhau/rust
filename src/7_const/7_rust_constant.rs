fn main() {
    const USER_LIMIT: i32 = 100;
    const PI: f32 = 3.14;

    println!("user limit is : {}", USER_LIMIT);
    // println!("pi value is : {}", PI)

    let salary = 100.00;
    let salary = 4.00;
    // read first salary
    println!("salary is {}", salary);

    let uname = "hsuhau";
    let uname = uname.len();
    println!("uname changed to integer : {}", uname);


    /*
    error[E0428]: the name `NAME` is defined multiple times
    --> 7_rust_constant.rs:18:5
    |
    17 |     const NAME:&str = "HHH";
    |     ------------------------ previous definition of the value `NAME` here
    18 |     const NAME:usize = NAME.len();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NAME` redefined here
    |
    = note: `NAME` must be defined only once in the value namespace of this block

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0428`.
    */
    // unlike variables, constants cannot be shadowed. If variables in the above program are replaced with constants, the compiler will throw an error.
    // const NAME:&str = "HHH";
    // const NAME:usize = NAME.len();
    // println!("name changed to integer : {}", NAME);
}