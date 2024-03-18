fn print_country(country: String) -> () {
    println!("{}", country);
}

fn print_country_and_give_back(country: String) -> String {
    println!("{}", country);
    country
}

fn print_country_by_ref(country: &String) -> () {
    println!("{}", country);
}

fn ref_country_and_change_it(country: &mut String) -> () {
    country.push_str(", it's a beautiful country.");
    println!("{}", country);
}

fn take_country_and_change_it(mut country: String) -> () {
    country.push_str(", it's a beautiful country too.");
    println!("{}", country);
}
fn main() {
    let country = String::from("China");
    print_country(country);
    // print_country(country); // now this is an error
    // or we can do print_country(country.clone());
    let another_country = String::from("America");
    let another_country = print_country_and_give_back(another_country); // this is useful but awkward
    print_country_and_give_back(another_country);

    let third_country: String = String::from("England");
    print_country_by_ref(&third_country);
    print_country_by_ref(&third_country); // this works

    let mut fourth_country = "Janpanese".to_string(); // this must be mutable
    println!("{}", fourth_country);
    ref_country_and_change_it(&mut fourth_country);
    println!("{}", fourth_country);

    let final_country = String::from("French"); // well this can be unmutable
    take_country_and_change_it(final_country); // this func take final_country and make it mutable
}
