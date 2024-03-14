fn print_string(str: &String) -> () {
    println!("{}", str);
}
fn main() {
    let first_string = String::from("This is first string");
    let second_string: String = "This is second string".into();
    let third_string = format!("This is {} string", "third");
    let fourth_string = "This is fourth string".to_string();

    print_string(&first_string);
    print_string(&second_string);
    print_string(&third_string);
    print_string(&fourth_string);
}
