fn print_hello_world() -> () {
    let greetings : String = String::from("Hello world");
    println!("{}", greetings);
}

fn mutiply_and_print(number_1: i32, number_2: i32) -> () {
    let result : i32 = number_1 * number_2;
    println!("{} mutiply {} = {}", number_1, number_2, result);
}
fn main() {
    print_hello_world();
    let num_1 = 10_i32;
    let num_2 = 2_i64;
    mutiply_and_print(num_1, num_2 as i32);
}
