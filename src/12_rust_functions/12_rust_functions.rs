fn fn_hello() {
    println!("hello from function fn_hello");
}

fn get_pi() -> f64 {
    return 22.0 / 7.0;
}

fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no * 0;
    println!("param_no is : {}", param_no);
}

fn main() {
    fn_hello;
    println!("pi is : {}", get_pi());
    let no: i32 = 5;
    mutate_no_to_zero(no);
    println!("no is : {}", no);
}