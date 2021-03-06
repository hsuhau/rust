# Rust - Collections

Rust's standard collection library provides efficient implementations of the most common general-purpose programming data structures. This chapter discusses the implementation of the commonly used collections − Vector, HashMap and HashSet.

## Vector

A Vector is a resizable array. It stores values in contiguous memory blocks. The predefined structure Vec can be used to create vectors. Some important features of a Vector are −

- A Vector can grow or shrink at runtime.
- A Vector is a homogeneous collection.
- A Vector stores data as sequence of elements in a particular order. Every element in a Vector is assigned a unique index number. The index starts from 0 and goes up to n-1 where, n is the size of the collection. For example, in a collection of 5 elements, the first element will be at index 0 and the last element will be at index 4.
- A Vector will only append values to (or near) the end. In other words, a Vector can be used to implement a stack.
- Memory for a Vector is allocated in the heap.

### Syntax - Creating a Vector

```
let mut instance_name = Vec::new();
```

The static method *new()* of the *Vec*structure is used to create a vector instance.

Alternatively, a vector can also be created using the vec! macro. The syntax is as given below −

```
let vector_name = vec![val1,val2,val3]
```

The following table lists some commonly used functions of the Vec structure.

| Sr.No | Method     | Signature & Description                                      |
| ----- | ---------- | ------------------------------------------------------------ |
| 1     | new()      | **pub fn new()->Vect** Constructs a new, empty Vec. The vector will not allocate until elements are pushed onto it. |
| 2     | push()     | **pub fn push(&mut self, value: T)** Appends an element to the back of a collection. |
| 3     | remove()   | **pub fn remove(&mut self, index: usize) -> T** Removes and returns the element at position index within the vector, shifting all elements after it to the left. |
| 4     | contains() | **pub fn contains(&self, x: &T) -> bool** Returns true if the slice contains an element with the given value. |
| 5     | len()      | **pub fn len(&self) -> usize** Returns the number of elements in the vector, also referred to as its 'length'. |

### Illustration: Creating a Vector - new()

To create a vector, we use the static method *new*−

```rust
fn main() {
   let mut v = Vec::new();
   v.push(20);
   v.push(30);
   v.push(40);

   println!("size of vector is :{}",v.len());
   println!("{:?}",v);
}
```

The above example creates a Vector using the static method *new()* that is defined in structure *Vec*. The *push(val)* function appends the value passed as parameter to the collection. The len() function returns the length of the vector.

### Output

```
size of vector is :3
[20, 30, 40]
```

### Illustration: Creating a Vector - vec! Macro

The following code creates a vector using the vec! macro. The data type of the vector is inferred the first value that is assigned to it.

```rust
fn main() {
   let v = vec![1,2,3];
   println!("{:?}",v);
}
```

### Output

```
[1, 2, 3]
```

As mentioned earlier, a vector can only contain values of the same data type. The following snippet will throw a *error[E0308]: mismatched types* error.

```rust
fn main() {
   let v = vec![1,2,3,"hello"];
   println!("{:?}",v);
}
```

### Illustration: push()

Appends an element to the end of a collection.

```rust
fn main() {
   let mut v = Vec::new();
   v.push(20);
   v.push(30);
   v.push(40);
   
   println!("{:?}",v);
}
```

### Output

```
[20, 30, 40]
```

### Illustration: remove()

Removes and returns the element at position index within the vector, shifting all elements after it to the left.

```rust
fn main() {
   let mut v = vec![10,20,30];
   v.remove(1);
   println!("{:?}",v);
}
```

### Output

```
[10, 30]
```

### Illustration - contains()

Returns true if the slice contains an element with the given value −

```rust
fn main() {
   let v = vec![10,20,30];
   if v.contains(&10) {
      println!("found 10");
   }
   println!("{:?}",v);
}
```

### Output

```
found 10
[10, 20, 30]
```

### Illustration: len()

Returns the number of elements in the vector, also referred to as its 'length'.

```rust
fn main() {
   let v = vec![1,2,3];
   println!("size of vector is :{}",v.len());
}
```

### Output

```
size of vector is :3
```

### Accessing values from a Vector

Individual elements in a vector can be accessed using their corresponding index numbers. The following example creates a vector ad prints the value of the first element.

```rust
fn main() {
   let mut v = Vec::new();
   v.push(20);
   v.push(30);

   println!("{:?}",v[0]);
}
Output: `20`
```

Values in a vector can also be fetched using reference to the collection.

```rust
fn main() {
   let mut v = Vec::new();
   v.push(20);
   v.push(30);
   v.push(40);
   v.push(500);

   for i in &v {
      println!("{}",i);
   }
   println!("{:?}",v);
}
```

### Output

```
20
30
40
500
[20, 30, 40, 500]
```

## HashMap

A map is a collection of key-value pairs (called entries). No two entries in a map can have the same key. In short, a map is a lookup table. A HashMap stores the keys and values in a hash table. The entries are stored in an arbitrary order. The key is used to search for values in the HashMap. The HashMap structure is defined in the **std::collections** module. This module should be explicitly imported to access the HashMap structure.

### Syntax: Creating a HashMap

```
let mut instance_name = HashMap::new();
```

The static method *new()* of the *HashMap* structure is used to create a HashMap object. This method creates an empty HashMap.

The commonly used functions of HashMap are discussed below −

| Sr.No | Method       | Signature & Description                                      |
| ----- | ------------ | ------------------------------------------------------------ |
| 1     | insert()     | **pub fn insert(&mut self, k: K, v: V) -> Option**Inserts a key/value pair, if no key then None is returned. After update, old value is returned. |
| 2     | len()        | **pub fn len(&self) -> usize**Returns the number of elements in the map. |
| 3     | get()        | **pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq**Returns a reference to the value corresponding to the key. |
| 4     | iter()       | **pub fn iter(&self) -> Iter<K, V>**An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V). |
| 5     | contains_key | **pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool**Returns true if the map contains a value for the specified key. |
| 6     | remove()     | **pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>**Removes a key from the map, returning the stored key and value if the key was previously in the map. |

### Illustration: insert()

Inserts a key/value pair into the HashMap.

```rust
use std::collections::HashMap;
fn main(){
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("{:?}",stateCodes);
}
```

The above program creates a HashMap and initializes it with 2 key-value pairs.

### Output

```
{"KL": "Kerala", "MH": "Maharashtra"}
```

### Illustration: len()

Returns the number of elements in the map

```rust
use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("size of map is {}",stateCodes.len());
}
```

The above example creates a HashMap and prints the total number of elements in it.

### Output

```
size of map is 2
```

### Illustration - get()

Returns a reference to the value corresponding to the key. The following example retrieves the value for key *KL* in the HashMap.

```rust
use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("size of map is {}",stateCodes.len());
   println!("{:?}",stateCodes);

   match stateCodes.get(&"KL") {
      Some(value)=> {
         println!("Value for key KL is {}",value);
      }
      None => {
         println!("nothing found");
      }
   }
}
```

### Output

```
size of map is 2
{"KL": "Kerala", "MH": "Maharashtra"}
Value for key KL is Kerala
```

### Illustration − iter()

Returns an iterator containing reference to all key-value pairs in an arbitrary order.

```rust
use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");

   for (key, val) in stateCodes.iter() {
      println!("key: {} val: {}", key, val);
   }
}
```

### Output

```
key: MH val: Maharashtra
key: KL val: Kerala
```

### Illustration: contains_key()

Returns true if the map contains a value for the specified key.

```rust
use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   stateCodes.insert("GJ","Gujarat");

   if stateCodes.contains_key(&"GJ") {
      println!("found key");
   }
}
```

### Output

```
found key
```

### Illustration: remove()

Removes a key from the map.

```rust
use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   stateCodes.insert("GJ","Gujarat");

   println!("length of the hashmap {}",stateCodes.len());
   stateCodes.remove(&"GJ");
   println!("length of the hashmap after remove() {}",stateCodes.len());
}
```

### Output

```
length of the hashmap 3
length of the hashmap after remove() 2
```

## HashSet

HashSet is a set of unique values of type T. Adding and removing values is fast, and it is fast to ask whether a given value is in the set or not. The HashSet structure is defined in the std::collections module. This module should be explicitly imported to access the HashSet structure.

### Syntax: Creating a HashSet

```
let mut hash_set_name = HashSet::new();
```

The static method, *new*, of HashSet structure is used to create a HashSet. This method creates an empty HashSet.

The following table lists some of the commonly used methods of the HashSet structure.

| Sr.No | Method       | Signature & Description                                      |
| ----- | ------------ | ------------------------------------------------------------ |
| 1     | insert()     | **pub fn insert(&mut self, value: T) -> bool**Adds a value to the set. If the set did not have this value present, true is returned else false. |
| 2     | len()        | **pub fn len(&self) -> usize**Returns the number of elements in the set. |
| 3     | get()        | **pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow,Q: Hash + Eq,**Returns a reference to the value in the set, if any that is equal to the given value. |
| 4     | iter()       | **pub fn iter(&self) -> Iter**Returns an iterator visiting all elements in arbitrary order. The iterator element type is &'a T. |
| 5     | contains_key | **pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool**Returns true if the set contains a value. |
| 6     | remove()     | **pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool**Removes a value from the set. Returns true if the value was present in the set. |

### Illustration - insert()

Adds a value to the set. A HashSet does not add duplicate values to the collection.

```rust
use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();

   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");//duplicates not added

   println!("{:?}",names);
}
```

### Output

```
{"TutorialsPoint", "Kannan", "Mohtashim"}
```

### Illustration: len()

Returns the number of elements in the set.

```rust
use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   println!("size of the set is {}",names.len());
}
```

### Output

```
size of the set is 3
```

### Illustration - iter()

Retruns an iterator visiting all elements in arbitrary order.

```rust
use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");

   for name in names.iter() {
      println!("{}",name);
   }
}
```

### Output

```
TutorialsPoint
Mohtashim
Kannan
```

### Illustration: get()

Returns a reference to the value in the set, if any, which is equal to the given value.

```rust
use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");

   match names.get(&"Mohtashim"){
      Some(value)=>{
         println!("found {}",value);
      }
      None =>{
         println!("not found");
      }
   }
   println!("{:?}",names);
}
```

### Output

```
found Mohtashim
{"Kannan", "Mohtashim", "TutorialsPoint"}
```

### Illustration - contains()

Returns true if the set contains a value.

```rust
use std::collections::HashSet;

fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");

   if names.contains(&"Kannan") {
      println!("found name");
   }  
}
```

### Output

```
found name
```

### Illustration: remove()

Removes a value from the set.

```rust
use std::collections::HashSet;

fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   println!("length of the Hashset: {}",names.len());
   names.remove(&"Kannan");
   println!("length of the Hashset after remove() : {}",names.len());
}
```

### Output

```
length of the Hashset: 3
length of the Hashset after remove() : 2
```