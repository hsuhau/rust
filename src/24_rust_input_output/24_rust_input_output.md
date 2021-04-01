# Rust - Input Output

This chapter discusses how to accept values from the standard input (keyboard) and display values to the standard output (console). In this chapter, we will also discuss passing command line arguments.

## Reader and Writer Types

Rust’s standard library features for input and output are organized around two traits −

- Read
- Write

| Sr.No | Trait & Description                                          | Example     |
| ----- | ------------------------------------------------------------ | ----------- |
| 1     | **Read** Types that implement Read have methods for byte-oriented input. They’re called readers | Stdin,File  |
| 2     | **Write** Types that implement Write support both byte-oriented and UTF-8 text output. They’re called writers. | Stdout,File |

## Read Trait

**Readers** are components that your program can read bytes from. Examples include reading input from the keyboard, files, etc. The **read_line()** method of this trait can be used to read data, one line at a time, from a file or standard input stream.

| Sr.No | Trait | Method & Description                                         |
| ----- | ----- | ------------------------------------------------------------ |
| 1     | Read  | **read_line(&mut line)->Result**Reads a line of text and appends it to line, which is a String. The return value is an io::Result, the number of bytes read. |

### Illustration − Reading from the Console − stdin()

Rust programs might have to accept values from the user at runtime. The following example reads values from the standard input (Keyboard) and prints it to the console.

```rust
fn main(){
   let mut line = String::new();
   println!("Enter your name :");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}
```

The *stdin()* function returns a handle to the standard input stream of the current process, to which the *read_line* function can be applied. This function tries to read all the characters present in the input buffer when it encounters an end-of-line character.

### Output

```
Enter your name :
Mohtashim
Hello , Mohtashim
no of bytes read , 10
```

## Write Trait

**Writers** are components that your program can write bytes to. Examples include printing values to the console, writing to files, etc. The write() method of this trait can be used to write data to a file or standard output stream.

| Sr.No | Trait | Method & Description                                         |
| ----- | ----- | ------------------------------------------------------------ |
| 1     | Write | **write(&buf)->Result**Writes some of the bytes in the slice buf to the underlying stream. It returns an io::Result, the number of bytes written. |

### Illustration - Writing to the Console - stdout()

The *print!* or *println!* macros can be used to display text on the console. However, you can also use the *write()* standard library function to display some text to the standard output.

Let us consider an example to understand this.

```rust
use std::io::Write;
fn main() {
   let b1 = std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
   let b2 = std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
   std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();
}
```

### Output

```
Tutorials Point
bytes written 15
```

The *stdout()* standard library function returns a handle to the standard output stream of the current process, to which the **write** function can be applied. The write() method returns an enum, Result. The unwrap() is a helper method to extract the actual result from the enumeration. The unwrap method will send panic if an error occurs.

**NOTE** − File IO is discussed in the next chapter.

## CommandLine Arguments

CommandLine arguments are passed to a program before executing it. They are like parameters passed to functions. CommandLine parameters can be used to pass values to the *main()* function. The **std::env::args()** returns the commandline arguments.

### Illustration

The following example passes values as commandLine arguments to the main() function. The program is created in a file name *main.rs*.

```rust
//main.rs
fn main(){
   let cmd_line = std::env::args();
   println!("No of elements in arguments is :{}",cmd_line.len()); 
   //print total number of values passed
   for arg in cmd_line {
      println!("[{}]",arg); //print all values passed 
      as commandline arguments
   }
}
```

The program will generate a file *main.exe* once compiled. Multiple command line parameters should be separated by space. Execute main.exe from the terminal as *main.exe hello tutorialspoint*.

**NOTE** − *hello* and *tutorialspoint* are commandline arguments.

### Output

```
No of elements in arguments is :3
[main.exe]
[hello]
[tutorialspoint]
```

The output shows 3 arguments as the *main.exe* is the first argument.

### Illustration

The following program calculates the sum of values passed as commandline arguments. A list integer values separated by space is passed to program.

```rust
fn main(){
   let cmd_line = std::env::args();
   println!("No of elements in arguments is 
   :{}",cmd_line.len()); 
   // total number of elements passed

   let mut sum = 0;
   let mut has_read_first_arg = false;

   //iterate through all the arguments and calculate their sum

   for arg in cmd_line {
      if has_read_first_arg { //skip the first argument since it is the exe file name
         sum += arg.parse::<i32>().unwrap();
      }
      has_read_first_arg = true; 
      // set the flag to true to calculate sum for the subsequent arguments.
   }
   println!("sum is {}",sum);
}
```

On executing the program as main.exe 1 2 3 4, the output will be −

```
No of elements in arguments is :5
sum is 10
```