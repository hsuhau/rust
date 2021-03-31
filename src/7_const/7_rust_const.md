# Rust - Constant

Constants represent values that cannot be changed. If you declare a constant then there is no way its value changes. The
keyword for using constants is **const**. Constants must be explicitly typed. Following is the syntax to declare a
constant.

```rust
const VARIABLE_NAME:dataType = value;
```

## Rust Constant Naming Convention

The naming convention for Constants are similar to that of variables. All characters in a constant name are usually in
uppercase. Unlike declaring variables, the **let** keyword is not used to declare a constant.

We have used constants in Rust in the example below −

```rust
fn main() {
   const USER_LIMIT:i32 = 100;    // Declare a integer constant
   const PI:f32 = 3.14;           //Declare a float constant

   println!("user limit is {}",USER_LIMIT);  //Display value of the constant
   println!("pi value is {}",PI);            //Display value of the constant
}
```

## Constants v/s Variables

In this section, we will learn about the differentiating factors between constants and variables.

- Constants are declared using the **const** keyword while variables are declared using the **let** keyword.
- A variable declaration can optionally have a data type whereas a constant declaration must specify the data type. This
  means const USER_LIMIT=100 will result in an error.
- A variable declared using the **let** keyword is by default immutable. However, you have an option to mutate it using
  the **mut** keyword. Constants are immutable.
- Constants can be set only to a constant expression and not to the result of a function call or any other value that
  will be computed at runtime.
- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts
  of the code need to know about.

## Shadowing of Variables and Constants

Rust allows programmers to declare variables with the same name. In such a case, the new variable overrides the previous
variable.

Let us understand this with an example.

```rust
fn main() {
   let salary = 100.00;
   let salary = 1.50 ; 
   // reads first salary
   println!("The value of salary is :{}",salary);
}
```

The above code declares two variables by the name salary. The first declaration is assigned a 100.00 while the second
declaration is assigned value 1.50. The second variable shadows or hides the first variable while displaying output.

### Output

```
The value of salary is :1.50
```

Rust supports variables with different data types while shadowing.

Consider the following example.

The code declares two variables by the name ***uname\***. The first declaration is assigned a string value, whereas the
second declaration is assigned an integer. The len function returns the total number of characters in a string value.

```rust
fn main() {
   let uname = "Mohtashim";
   let uname = uname.len();
   println!("name changed to integer : {}",uname);
}
```

### Output

```
name changed to integer: 9
```

Unlike variables, constants cannot be shadowed. If variables in the above program are replaced with constants, the
compiler will throw an error.

```rust
fn main() {
   const NAME:&str = "Mohtashim";
   const NAME:usize = NAME.len(); 
   //Error : `NAME` already defined
   println!("name changed to integer : {}",NAME);
}
```