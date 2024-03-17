const GLOBAL_NUM: u32 = 10;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
fn main() {
    println!("global num = {}", GLOBAL_NUM);
    println!("spring = {}", SEASONS[0]);
}
