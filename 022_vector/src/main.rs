fn main() {
    let mut string_vec : Vec<String> = Vec::new();
    string_vec.push("Hello".to_string());
    string_vec.push("World".to_string());
    for i in 0..=1 {
        println!("{}, {}", string_vec[i], string_vec.capacity());
    }

    let mut num_vec : Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in 0..=4 {
        println!("{}", num_vec[i]);
    }

    let mut float_vec : Vec<f64> = Vec::with_capacity(20); // use this to reserve
    println!("capacity of float_vec = {}", float_vec.capacity());
}
