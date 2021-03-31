fn main() {
    // let sliced_value = &data_structure[start_index..end_index]

    let n1 = "Tutorials".to_string();
    let c1 = &n1[4..9];
    println!("c1 is {}", c1);

    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]);

    let mut data_mut = [10, 20, 30, 40, 50];
    use_slice_mut(&mut data_mut[1..4]);
    println!("data_mut is {:?}", data_mut);
}

/**
 * Illustration - Slicing an integer array
 */
fn use_slice(slice: &[i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("slice is {:?}", slice);
}

/**
 * Mutable Slices
 */
fn use_slice_mut(slice: &mut [i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("slice is {:?}", slice);
    slice[0] = 1010; // replace 20 to 1010
}
