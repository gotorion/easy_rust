fn main() {
    let my_tuple = (
        "Here is a tuple",
        12,
        // vec![1, 2, 3],
        String::from("Hello"),
        "world",
        23.93,
    );
    println!(
        "first element: {}
    second element: {}
    third element: {}
    fourth element: {}
    fifth element: {}
    ",
        my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3, my_tuple.4,
    );

    let _nums : Vec<i32> = Vec::new();
    let numbers : Vec<i32> = vec![1, 2, 3, 4, 5];
    let (first, _, third, _, fifth) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);
    println!("{}, {}, {}", first, third, fifth);
}
