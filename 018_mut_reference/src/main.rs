fn test_mut_reference() -> () {
    let mut num = 10i32;
    println!("{}", num);
    let num_ref = &mut num;
    println!("{}", num_ref);
    *num_ref = 20;
    println!("{}", num);
}

fn test_shadowing() -> () {
    let country = String::from("China");
    let country_ref = &country;
    let country: i32 = 8;
    println!("{}, {}", country_ref, country);
}

fn main() {
    test_mut_reference();
    test_shadowing();
}
