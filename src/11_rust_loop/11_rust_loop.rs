// fn main(){
//     for x in 1..11{
//         if x ==5 {
//             continue;
//         }
//         println!("x is {}", x);
//     }
// }

fn main() {
    let mut x = 0;
    loop {
        x += 1;
        println!("x is {}", x);
        if x == 15 {
            break;
        }
    }
}