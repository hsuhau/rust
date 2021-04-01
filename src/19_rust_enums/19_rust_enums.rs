enum GenderCategory {
    Male,
    Female,
}

struct Person {
    name: String,
    gender: GenderCategory,
}

fn main() {
    let p1 = Person {
        name: String::from("hsuhau"),
        gender: GenderCategory::Male,
    };
    let p2 = Person {
        name: String::from("hh"),
        gender: GenderCategory::Female,
    };

    println!("p1 is : {:?}", p1);
    println!("p2 is : {:?}", p2);
}
