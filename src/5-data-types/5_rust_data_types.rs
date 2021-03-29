fn main(){

    // Declare a Variable
    let company_string = "TutorialsPoint";
    let rating_float = 4.5;
    let is_growing_boolean = true;
    let icon_char = '❤';
    println!("company name is : {}", company_string);
    println!("company rating on 5 is : {}", rating_float);
    println!("company is growing : {}", is_growing_boolean);
    println!("company icon is : {}", icon_char);

    // Integer
    /**
     * Sr.No.	Size	Signed	Unsigned
     *  1	8 bit	i8	u8
        2	16 bit	i16	u16
        3	32 bit	i32	u32
        4	64 bit	i64	u64
        5	128 bit	i128	u128
        6	Arch	isize	usize
     */
    let result = 10;
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("result is : {}", result);
    println!("age is : {} and sum is : {}", age, sum);
    println!("mark is: {} and count is :{}", mark, count);

    // Integer Overflow
    let age:u8 = 255;
    let height:u8 = 256;
    let weight:u8 = 257;
    let score:u8 = 258;

    println!("age is : {}", age);
    println!("height is : {}", height);
    println!("weight is : {}", weight);
    println!("score is : {}", score);
}