fn main() {
    let tuple: (i32, f64, u8) = (-325, 4.9, 22);
    // println!("tuple is {}", tuple);
    println!("{:?}", tuple);
    println!("integer is {:?}", tuple.0);
    println!("float is {:?}", tuple.1);
    println!("unsigned integer is {:?}", tuple.2);

    let b: (i32, bool, f64) = (110, true, 10.9);
    print(b);
}

/**
 * pass the tuple as parameter
 */
fn print(x: (i32, bool, f64)) {
    println!("inside print method");
    println!("{:?}", x);
}

/**
 * Destructing assignment is a feature of rust wherein we unpack the values of a tuple. 
 * This is achieved by assigning a tuple to distinct variables.
 */
fn print1(x: (i32, bool, f64)) {
    println!("inside print1 method");
    let (age, is_male, cgpa) = x; // assigns a tuple to distinct variables
    println!("age is : {}, is_male is : {}, cgpa is : {}", age, is_male, cgpa);
}