# Rust - Data Types

The Type System represents the different types of values supported by the language. The Type System checks validity of
the supplied values, before they are stored or manipulated by the program. This ensures that the code behaves as
expected. The Type System further allows for richer code hinting and automated documentation too.

Rust is a statically typed language. Every value in Rust is of a certain data type. The compiler can automatically infer
data type of the variable based on the value assigned to it.

## Declare a Variable

Use the ***let*** keyword to declare a variable.

``` rust
fn main() {
   let company_string = "TutorialsPoint";  // string type
   let rating_float = 4.5;                 // float type
   let is_growing_boolean = true;          // boolean type
   let icon_char = '♥';                    //unicode character type

   println!("company name is:{}",company_string);
   println!("company rating on 5 is:{}",rating_float);
   println!("company is growing :{}",is_growing_boolean);
   println!("company icon is:{}",icon_char);
}
```

In the above example, data type of the variables will be inferred from the values assigned to them. For example, Rust
will assign string data type to the variable *company_string*, float data type to *rating_float*, etc.

The *println!* macro takes two arguments −

- A special syntax *{ }*, which is the placeholder
- The variable name or a constant

The placeholder will be replaced by the variable’s value

The output of the above code snippet will be −

```
company name is: TutorialsPoint
company rating on 5 is:4.5
company is growing: true
company icon is: ♥
```

## Scalar Types

A scalar type represents a single value. For example, 10,3.14,'c'. Rust has four primary scalar types.

- Integer
- Floating-point
- Booleans
- Characters

We will learn about each type in our subsequent sections.

## Integer

An integer is a number without a fractional component. Simply put, the integer data type is used to represent whole
numbers.

Integers can be further classified as Signed and Unsigned. Signed integers can store both negative and positive values.
Unsigned integers can only store positive values. A detailed description if integer types is given below −

| Sr.No. | Size    | Signed | Unsigned |
| ------ | ------- | ------ | -------- |
| 1      | 8 bit   | i8     | u8       |
| 2      | 16 bit  | i16    | u16      |
| 3      | 32 bit  | i32    | u32      |
| 4      | 64 bit  | i64    | u64      |
| 5      | 128 bit | i128   | u128     |
| 6      | Arch    | isize  | usize    |

The size of an integer can be *arch*. This means the size of the data type will be derived from the *architecture* of
the machine. An integer the size of which is *arch* will be 32 bits on an x86 machine and 64 bits on an x64 machine. An
arch integer is primarily used when indexing some sort of collection.

### Illustration

``` rust
fn main() {
   let result = 10;    // i32 by default
   let age:u32 = 20;
   let sum:i32 = 5-15;
   let mark:isize = 10;
   let count:usize = 30;
   println!("result value is {}",result);
   println!("sum is {} and age is {}",sum,age);
   println!("mark is {} and count is {}",mark,count);
}
```

The output will be as given below −

```
result value is 10
sum is -10 and age is 20
mark is 10 and count is 30
```

The above code will return a compilation error if you replace the value of *age* with a floating-point value.

### Integer Range

Each signed variant can store numbers from *-(2^(n-1) to 2^(n-1) -1*, where n is the number of bits that variant uses.
For example, i8 can store numbers from *-(2^7) to 2^7 -1* − here we replaced *n* with 8.

Each unsigned variant can store numbers from *0* to *(2^n)-1*. For example, u8 can store numbers from *0* to *2^7*,
which is equal to 0 to 255.

### Integer Overflow

An integer overflow occurs when the value assigned to an integer variable exceeds the Rust defined range for the data
type. Let us understand this with an example −

``` rust
fn main() {
   let age:u8 = 255;

   // 0 to 255 only allowed for u8
   let weight:u8 = 256;   //overflow value is 0
   let height:u8 = 257;   //overflow value is 1
   let score:u8 = 258;    //overflow value is 2

   println!("age is {} ",age);
   println!("weight is {}",weight);
   println!("height is {}",height);
   println!("score is {}",score);
}
```

The valid range of unsigned u8 variable is 0 to 255. In the above example, the variables are assigned values greater
than 255 (upper limit for an integer variable in Rust). On execution, the above code will return a warning − ***warning
− literal out of range for u8\*** for weight, height and score variables. The overflow values after 255 will start from
0, 1, 2, etc. The final output without warning is as shown below −

```
age is 255
weight is 0
height is 1
score is 2
```

## Float

Float data type in Rust can be classified as **f32** and **f64**. The f32 type is a single-precision float, and f64 has
double precision. The default type is f64. Consider the following example to understand more about the float data type.

``` rust
fn main() {
   let result = 10.00;        //f64 by default
   let interest:f32 = 8.35;
   let cost:f64 = 15000.600;  //double precision
   
   println!("result value is {}",result);
   println!("interest is {}",interest);
   println!("cost is {}",cost);
}
```

The output will be as shown below −

```
interest is 8.35
cost is 15000.6
```

### Automatic Type Casting

Automatic type casting is not allowed in Rust. Consider the following code snippet. An integer value is assigned to the
float variable ***interest\***.

``` rust
fn main() {
   let interest:f32 = 8;   // integer assigned to float variable
   println!("interest is {}",interest);
}
```

The compiler throws a ***mismatched types error\*** as given below.

```
error[E0308]: mismatched types
   --> main.rs:2:22
   |
 2 | let interest:f32=8;
   |    ^ expected f32, found integral variable
   |
   = note: expected type `f32`
      found type `{integer}`
error: aborting due to previous error(s)
```

### Number Separator

For easy readability of large numbers, we can use a visual separator _ underscore to separate digits. That is 50,000 can
be written as 50_000 . This is shown in the below example.

``` rust
fn main() {
   let float_with_separator = 11_000.555_001;
   println!("float value {}",float_with_separator);
   
   let int_with_separator = 50_000;
   println!("int value {}",int_with_separator);
}
```

The output is given below −

```
float value 11000.555001
int value 50000
```

### Boolean

Boolean types have two possible values – *true* or *false*. Use the **bool** keyword to declare a boolean variable.

### Illustration

``` rust
fn main() {
   let isfun:bool = true;
   println!("Is Rust Programming Fun ? {}",isfun);
}
```

The output of the above code will be −

```
Is Rust Programming Fun ? true
```

### Character

The character data type in Rust supports numbers, alphabets, Unicode and special characters. Use the **char** keyword to
declare a variable of character data type. Rust’s char type represents a Unicode Scalar Value, which means it can
represent a lot more than just ASCII. Unicode Scalar Values range from **U+0000** to **U+D7FF** and **U+E000** to **
U+10FFFF** inclusive.

Let us consider an example to understand more about the Character data type.

``` rust
fn main() {
   let special_character = '@'; //default
   let alphabet:char = 'A';
   let emoji:char = '😁';
   
   println!("special character is {}",special_character);
   println!("alphabet is {}",alphabet);
   println!("emoji is {}",emoji);
}
```

The output of the above code will be −

```
special character is @
alphabet is A
emoji is 😁
```