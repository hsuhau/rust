/*
[Running] cd "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/" && rustc 15_rust_ownership.rs && "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/"15_rust_ownership
warning: unused variable: `v2`
 --> 15_rust_ownership.rs:6:8
  |
6 |    let v2 = v; 
  |        ^^ help: if this is intentional, prefix it with an underscore: `_v2`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `v`
  --> 15_rust_ownership.rs:13:20
   |
2  |    let v = vec![1,2,3]; 
   |        - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
...
6  |    let v2 = v; 
   |             - value moved here
...
13 |    println!("{:?}",v);
   |                    ^ value borrowed here after move

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
*/

/* fn main(){
   let v = vec![1,2,3]; 
   // vector v owns the object in heap

   //only a single variable owns the heap memory at any given time
   let v2 = v; 
   // here two variables owns heap value,
   //two pointers to the same content is not allowed in rust

   //Rust is very smart in terms of memory access ,so it detects a race condition
   //as two variables point to same heap

   println!("{:?}",v);
} */

/*
[Running] cd "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/" && rustc 15_rust_ownership.rs && "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/"15_rust_ownership
error[E0382]: borrow of moved value: `v2`
  --> 15_rust_ownership.rs:48:28
   |
46 |    let v2 = v;              // moves ownership to v2
   |        -- move occurs because `v2` has type `Vec<i32>`, which does not implement the `Copy` trait
47 |    display(v2);             // v2 is moved to display and v2 is invalidated
   |            -- value moved here
48 |    println!("In main {:?}",v2);    //v2 is No longer usable here
   |                            ^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/

/* fn main(){
   let v = vec![1,2,3];     // vector v owns the object in heap
   let v2 = v;              // moves ownership to v2
   display(v2);             // v2 is moved to display and v2 is invalidated
   println!("In main {:?}",v2);    //v2 is No longer usable here
}
fn display(v:Vec<i32>){
   println!("inside display {:?}",v);
} */

/*
[Running] cd "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/" && rustc 15_rust_ownership.rs && "/home/hsuhau/GitHub/hsuhau/rust/src/15_rust_ownership/"15_rust_ownership
error[E0308]: mismatched types
  --> 15_rust_ownership.rs:76:25
   |
76 | fn display(v:Vec<i32>)->Vec<i32> { 
   |    -------              ^^^^^^^^ expected struct `Vec`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   |
   = note: expected struct `Vec<i32>`
           found unit type `()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
/* fn main(){
   let v = vec![1,2,3];       // vector v owns the object in heap
   let v2 = v;                // moves ownership to v2
   let v2_return = display(v2);    
   println!("In main {:?}",v2_return);
}
fn display(v:Vec<i32>)->Vec<i32> { 
   // returning same vector
   println!("inside display {:?}",v);
} */

// todo:don't understand
fn main() {
    let u1 = 10;
    let u2 = u1;  // u1 value copied(not moved) to u2

    println!("u1 = {}", u1);
}