// fn return_str() -> &str {
//     let country = String::from("China");
//     let country_ref = &country;
//     country_ref
// }
fn return_static_str() -> &'static str {
    static NAME: &str = "john";
    let name_ref = &NAME;
    name_ref
}
fn main() {
    // let country = return_str(); // this is an error
    let _name = return_static_str();
}
