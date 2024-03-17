fn test_scope() -> () {
    let my_number = {
        let second_number = 10_i32;
        second_number
        // what about add a semicolon?
        // the value of my_number will be () and the print will be error
    };
    println!("my_number = {}", my_number);

    let your_number = {}; // your_number is ()
                          // println!("your_number = {}", your_number); // error
    println!("your_number = {:?}", your_number); // debug printing
}

fn test_mutable() -> () {
    let name: String = String::from("Hello");
    // name.push_str(", world"); // error

    let mut another_name: String = String::from("Hello");
    another_name.push_str(", world");
}

fn test_reference() -> () {
    let name: &str = "Hello world";
    let ref_name = &name;
    let ref_ref_name = &&&&&name;
    println!("name = {}", name);
    println!("another_name = {}", ref_name);
    println!("ref_ref_name = {}", ref_name);

    let mut mut_ref_name = &name;
    mut_ref_name = &"Hello name";
    println!("mut_ref_name = {}", mut_ref_name);
}

fn main() {
    // test_scope();
    test_reference();
}
