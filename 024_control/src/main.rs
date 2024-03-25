fn test_if() ->() {
    let num = 10i32;
    if num >= 10 {
        println!("num greater than ten");
    } else {
        println!("num less than ten");
    }
}
fn test_match() {
    let num = 2i64;
    match num {
        0 => println!("num is zero"),
        1 => println!("num is one"),
        2 => println!("num is three"),
        _ => println!("num is whatever"),
    }
}
fn test_match_tuple() {
    let city = "Shanghai";
    let weather = "Sunny";
    match (city, weather) {
        (city, weather) if city == "Shanghai" => {println!("Welcome to Shanghai");}
        (city, weather) if weather == "Cold" => {println!("Not a good day");}
        _ => println!("Opppps"),
    }
}
fn test_at() {
    let num = 10;
    match num {
        number @ 4 => println!("num is {}", number),
        number @ 10 => println!("num is {}", number),
        _ => println!("I do not known what is this"),
    }
}
fn main() {
    test_if();
    test_match();
    test_match_tuple();
    test_at();
}
