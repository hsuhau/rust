# Rust - Functions

Functions are the building blocks of readable, maintainable, and reusable code. A function is a set of statements to
perform a specific task. Functions organize the program into logical blocks of code. Once defined, functions may be
called to access code. This makes the code reusable. Moreover, functions make it easy to read and maintain the program’s
code.

A function declaration tells the compiler about a function's name, return type, and parameters. A function definition
provides the actual body of the function.

| Sr.No | Function & Description                                       |
| ----- | ------------------------------------------------------------ |
| 1     | **Defining a function** TA function definition specifies what and how a specific task would be done. |
| 2     | **Calling or invoking a Function** A function must be called so as to execute it. |
| 3     | **Returning Functions** Functions may also return value along with control, back to the caller. |
| 4     | **Parameterized Function** Parameters are a mechanism to pass values to functions. |

## Defining a Function

A function definition specifies what and how a specific task would be done. Before using a function, it must be defined.
The function body contains code that should be executed by the function. The rules for naming a function are similar to
that of a variable. Functions are defined using the **fn** keyword. The syntax for defining a standard function is given
below

### Syntax

```rust
fn function_name(param1,param2..paramN) {
   // function body
}
```

A function declaration can optionally contain parameters/arguments. Parameters are used to pass values to functions.

### Example - Simple function definition

```rust
//Defining a function
fn fn_hello(){
   println!("hello from function fn_hello ");
}
```

## Invoking a Function

A function must be called so as to execute it. This process is termed as **function invocation**. Values for parameters
should be passed when a function is invoked. The function that invokes another function is called the **caller
function.**

### Syntax

```
function_name(val1,val2,valN)
```

### Example: Invoking a Function

```rust
fn main(){
   //calling a function
   fn_hello();
}
```

Here, the *main()* is the caller function.

### Illustration

The following example defines a function ***fn_hello()\***. The function prints a message to the console. The ***main()
\*** function invokes the *fn_hello()* function.

```rust
fn main(){
   //calling a function
   fn_hello();
}
//Defining a function
fn fn_hello(){
   println!("hello from function fn_hello ");
}
```

### Output

```
hello from function fn_hello
```

## Returning Value from a Function

Functions may also return a value along with control, back to the caller. Such functions are called returning functions.

### Syntax

Either of the following syntax can be used to define a function with return type.

### With return statement

```rust
// Syntax1
fn function_name() -> return_type {
   //statements
   return value;
}
```

### Shorthand syntax without return statement

```rust
//Syntax2
fn function_name() -> return_type {
   value //no semicolon means this value is returned
}
```

### lllustration

```rust
fn main(){
   println!("pi value is {}",get_pi());
}
fn get_pi()->f64 {
   22.0/7.0
}
```

### Output

```
pi value is 3.142857142857143
```

## Function with Parameters

Parameters are a mechanism to pass values to functions. Parameters form a part of the function’s signature. The
parameter values are passed to the function during its invocation. Unless explicitly specified, the number of values
passed to a function must match the number of parameters defined.

Parameters can be passed to a function using one of the following techniques −

### Pass by Value

When a method is invoked, a new storage location is created for each value parameter. The values of the actual
parameters are copied into them. Hence, the changes made to the parameter inside the invoked method have no effect on
the argument.

The following example declares a variable no, which is initially 5. The variable is passed as parameter (by value) to
the ***mutate_no_to_zero()\***functionnction, which changes the value to zero. After the function call when control
returns back to main method the value will be the same.

```rust
fn main(){
   let no:i32 = 5;
   mutate_no_to_zero(no);
   println!("The value of no is:{}",no);
}

fn mutate_no_to_zero(mut param_no: i32) {
   param_no = param_no*0;
   println!("param_no value is :{}",param_no);
}
```

Output

```
param_no value is :0
The value of no is:5
```

### Pass by Reference

When you pass parameters by reference, unlike value parameters, a new storage location is not created for these
parameters. The reference parameters represent the same memory location as the actual parameters that are supplied to
the method. Parameter values can be passed by reference by prefixing the variable name with an **&** .

In the example given below, we have a variable *no*, which is initially 5. A reference to the variable no is passed to
the ***mutate_no_to_zero()\*** function. The function operates on the original variable. After the function call, when
control returns back to main method, the value of the original variable will be the zero.

```rust
fn main() {
   let mut no:i32 = 5;
   mutate_no_to_zero(&mut no);
   println!("The value of no is:{}",no);
}
fn mutate_no_to_zero(param_no:&mut i32){
   *param_no = 0; //de reference
}
```

The * operator is used to access value stored in the memory location that the variable **param_no** points to. This is
also known as dereferencing.

The output will be −

```
The value of no is 0.
```

### Passing string to a function

The *main()* function passes a string object to the *display()* function.

```rust
fn main(){
   let name:String = String::from("TutorialsPoint");
   display(name); 
   //cannot access name after display
}
fn display(param_name:String){
   println!("param_name value is :{}",param_name);
}
```

### Output

```
param_name value is :TutorialsPoint
```