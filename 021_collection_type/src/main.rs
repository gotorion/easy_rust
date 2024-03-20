fn main() {
    let nums = [20, 30, 40];
    println!("num at index 2 is {}", nums[2]);
    let cities = ["Shanghai", "Beijing", "Wuhan", "Shenzhen"];

    let part_of_cities = &cities[1..=3]; // use = is not exclusive
    println!("part of cities are {:?}", part_of_cities);

    let another_part_of_cities = &cities[1..3]; // this is exclusive
    println!("another part of cities is {:?}", another_part_of_cities);
}
