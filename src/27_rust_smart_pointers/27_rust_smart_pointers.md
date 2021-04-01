# Rust - Smart Pointers

Rust allocates everything on the stack by default. You can store things on the heap by wrapping them in smart pointers like *Box*. Types like Vec and String implicitly help heap allocation. Smart pointers implement traits listed in the table below. These traits of the smart pointers differentiate them from an ordinary struct −

| Sr.No | Trait name | Package & Description                                        |
| ----- | ---------- | ------------------------------------------------------------ |
| 1     | Deref      | **std::ops::Deref** Used for immutable dereferencing operations, like *v. |
| 2     | Drop       | **std::ops::Drop** Used to run some code when a value goes out of scope. This is sometimes called a *destructor* |

In this chapter, we will learn about the **Box** smart pointer. We will also learn how to create a custom smart pointer like Box.

## Box

The Box smart pointer also called a box allows you to store data on the heap rather than the stack. The stack contains the pointer to the heap data. A Box does not have performance overhead, other than storing their data on the heap.

Let us see how to use a box to store an i32 value on the heap.

```rust
fn main() {
   let var_i32 = 5; 
   //stack
   let b = Box::new(var_i32); 
   //heap
   println!("b = {}", b);
}
```

### Output

```
b = 5
```

In order to access a value pointed by a variable, use dereferencing. The * is used as a dereference operator. Let us see how to use dereference with Box.

```rust
fn main() {
   let x = 5; 
   //value type variable
   let y = Box::new(x); 
   //y points to a new value 5 in the heap

   println!("{}",5==x);
   println!("{}",5==*y); 
   //dereferencing y
}
```

The variable x is a value-type with the value 5. So, the expression *5==x* will return true. Variable y points to the heap. To access the value in heap, we need to dereference using **y. \*y* returns value 5. So, the expression *5==\*y* returns true.

### Output

```
true
true
```

### Illustration - Deref Trait

The Deref trait, provided by the standard library, requires us to implement one method named *deref*, that borrows *self* and returns a reference to the inner data. The following example creates a structure *MyBox*, which is a generic type. It implements the trait *Deref*. This trait helps us access heap values wrapped by *y* using **y*.

```rust
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> { 
   // Generic structure with static method new
   fn new(x:T)-> MyBox<T> {
      MyBox(x)
   }
}
impl<T> Deref for MyBox<T> {
   type Target = T;
   fn deref(&self) -> &T {
      &self.0 //returns data
   }
}
fn main() {
   let x = 5;
   let y = MyBox::new(x); 
   // calling static method
   
   println!("5==x is {}",5==x);
   println!("5==*y is {}",5==*y); 
   // dereferencing y
   println!("x==*y is {}",x==*y);
   //dereferencing y
}
```

### Output

```
5==x is true
5==*y is true
x==*y is true
```

### Illustration - Drop Trait

The Drop trait contains the *drop()* method. This method is called when a structure that implemented this trait goes out of scope. In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. In Rust, you can achieve automatic memory deallocation using Drop trait.

```rust
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
   fn new(x:T)->MyBox<T>{
      MyBox(x)
   }
}
impl<T> Deref for MyBox<T> {
   type Target = T;
      fn deref(&self) -< &T {
      &self.0
   }
}
impl<T> Drop for MyBox<T>{
   fn drop(&mut self){
      println!("dropping MyBox object from memory ");
   }
}
fn main() {
   let x = 50;
   MyBox::new(x);
   MyBox::new("Hello");
}
```

In the above example, the drop method will be called twice as we are creating two objects in the heap.

```
dropping MyBox object from memory
dropping MyBox object from memory
```