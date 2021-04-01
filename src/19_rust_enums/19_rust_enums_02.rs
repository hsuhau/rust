enum CarType {
    hatch,
    sedan,
    suv
}

fn print_size(car:CarType){
    match car {
        hatch => println!("Small sized car"),
        sedan => println!("Medium sized car"),
        suv => println!("Large sized car")
    }
}

fn main() {
    print_size(CarType::hatch);
    print_size(CarType::sedan);
    print_size(CarType::suv);
}
