use std::io;

fn main() {
    println!("Provide the numbers for array");

    let mut arr_string = String::new();

    io::stdin().read_line(&mut arr_string).expect("Invalid Input");

    let char_vector : Vec<char> = arr_string.trim().chars().collect();

    println!("Provide the rotation amount");

    let mut rot_left = String::new();

    io::stdin().read_line(&mut rot_left).expect("Invalid Input");

    let rotation_amount: usize = rot_left.trim().parse().expect("Invalid Input");

    let split_index = rotation_amount % char_vector.len();

    let rot_vector: Vec<char> = [&char_vector[split_index..], &char_vector[..split_index]]
        .concat()
        .iter()
        .cloned()
        .collect();

        println!("The rotated value will be {:?}", rot_vector);

}
