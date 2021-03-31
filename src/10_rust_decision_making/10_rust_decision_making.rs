// fn main(){
//     let num:i32 = 5;
//     if num > 0 {
//         println!("{} is positive", num);
//     } else if num < 0{
//         println!("{} is negative", num);
//     } else {
//         println!("{} is neither positive nor negative", num)
//     }
// }


fn main() {
    let state_code = "MH";
    let state = match state_code {
        "MH" => {
            println!("Found match for MH");
            "Maharashtra"
        }
        "KL" => "KERALA",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is: {}", state);
}