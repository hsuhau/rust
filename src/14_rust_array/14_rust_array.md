# Rust - Array

In this chapter, we will learn about an array and the various features associated with it. Before we learn about arrays,
let us see how an array is different from a variable.

Variables have the following limitations −

- Variables are scalar in nature. In other words, a variable declaration can only contain a single value at a time. This
  means that to store n values in a program n variable declaration will be needed. Hence, the use of variables is not
  feasible when one needs to store a larger collection of values.
- Variables in a program are allocated memory in random order, thereby making it difficult to retrieve/read the values
  in the order of their declaration.

An array is a homogeneous collection of values. Simply put, an array is a collection of values of the same data type.

## Features of an Array

The features of an array are as listed below −

- An array declaration allocates sequential memory blocks.
- Arrays are static. This means that an array once initialized cannot be resized.
- Each memory block represents an array element.
- Array elements are identified by a unique integer called the subscript/ index of the element.
- Populating the array elements is known as array initialization.
- Array element values can be updated or modified but cannot be deleted.

## Declaring and Initializing Arrays

Use the syntax given below to declare and initialize an array in Rust.

### Syntax

```
//Syntax1
let variable_name = [value1,value2,value3];

//Syntax2
let variable_name:[dataType;size] = [value1,value2,value3];

//Syntax3
let variable_name:[dataType;size] = [default_value_for_elements,size];
```

In the first syntax, type of the array is inferred from the data type of the array’s first element during
initialization.

### Illustration: Simple Array

The following example explicitly specifies the size and the data type of the array. The *{:?} syntax of the println!()*
function is used to print all values in the array. The *len()* function is used to compute the size of the array.

```rust
fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}
```

### Output

```
array is [10, 20, 30, 40]
array size is :4
```

### Illustration: Array without data type

The following program declares an array of 4 elements. The datatype is not explicitly specified during the variable
declaration. In this case, the array will be of type integer. The *len()* function is used to compute the size of the
array.

```rust
fn main(){
   let arr = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}
```

### Output

```
array is [10, 20, 30, 40]
array size is :4
```

### Illustration: Default values

The following example creates an array and initializes all its elements with a default value of *-1*.

```rust
fn main() {
   let arr:[i32;4] = [-1;4];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}
```

### Output

```
array is [-1, -1, -1, -1]
array size is :4
```

### Illustration: Array with for loop

The following example iterates through an array and prints the indexes and their corresponding values. The loop
retrieves values from index 0 to 4 (index of the last array element).

```rust
fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());

   for index in 0..4 {
      println!("index is: {} & value is : {}",index,arr[index]);
   }
}
```

### Output

```
array is [10, 20, 30, 40]
array size is :4
index is: 0 & value is : 10
index is: 1 & value is : 20
index is: 2 & value is : 30
index is: 3 & value is : 40
```

### Illustration: Using the iter() function

The iter() function fetches values of all elements in an array.

```rust
fn main(){

let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());

   for val in arr.iter(){
      println!("value is :{}",val);
   }
}
```

### Output

```
array is [10, 20, 30, 40]
array size is :4
value is :10
value is :20
value is :30
value is :40
```

### Illustration: Mutable array

The *mut* keyword can be used to declare a mutable array. The following example declares a mutable array and modifies
value of the second array element.

```rust
fn main(){
   let mut arr:[i32;4] = [10,20,30,40];
   arr[1] = 0;
   println!("{:?}",arr);
}
```

### Output

```
[10, 0, 30, 40]
```

## Passing Arrays as Parameters to Functions

An array can be passed by value or by reference to functions.

### Illustration: Pass by value

```rust
fn main() {
   let arr = [10,20,30];
   update(arr);

   print!("Inside main {:?}",arr);
}
fn update(mut arr:[i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}
```

### Output

```
Inside update [0, 0, 0]
Inside main [10, 20, 30]
```

### Illustration: Pass by reference

```rust
fn main() {
   let mut arr = [10,20,30];
   update(&mut arr);
   print!("Inside main {:?}",arr);
}
fn update(arr:&mut [i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}
```

### Output

```
Inside update [0, 0, 0]
Inside main [0, 0, 0]
```

## Array Declaration and Constants

Let us consider an example given below to understand array declaration and constants.

```rust
fn main() {
   let N: usize = 20;
   let arr = [0; N]; //Error: non-constant used with constant
   print!("{}",arr[10])
}
```

The compiler will result in an exception. This is because an array's length must be known at compile time. Here, the
value of the variable "N" will be determined at runtime. In other words, variables cannot be used to define the size of
an array.

However, the following program is valid −

```rust
fn main() {
   const N: usize = 20; 
   // pointer sized
   let arr = [0; N];

   print!("{}",arr[10])
}
```

The value of an identifier prefixed with the const keyword is defined at compile time and cannot be changed at runtime.
usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.