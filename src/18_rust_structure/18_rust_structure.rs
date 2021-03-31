/*
struct Name_of_structure {
   field1:data_type,
   field2:data_type,
   field3:data_type
}
*/

struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn main() {
    let emp = Employee {
        name: String::from("hsuhau"),
        company: String::from("TutorialsPoint"),
        age: 23,
    };
    println!("emp name is: {}, company is: {}, age is: {}", emp.name, emp.company, emp.age);


    // Modifying a struct instance
    let emp_mut = Employee {
        name: String::from("hsuhau"),
        company: String::from("TutorialsPoint"),
        age: 23,
    };
    println!("emp_mut name is: {}, company is: {}, age is: {}", emp_mut.name, emp_mut.company, emp_mut.age);
}

