fn test_print_01() -> () {
    println!("\t start \\ with tab \n and move to new line");
}

fn test_print_02() -> () {
    println!(
        "Hello
    world"
    );
}

fn test_print_03() -> () {
    // row string
    println!(r#"this is row string \t"#);
    println!(r##"even use a #hashtag"##);
}

fn test_print_04() -> () {
    println!("this will look like number");
    println!("{:?}", b"this will look like numbers");
    println!("{:#?}", b"this will look like numbers");
}

fn test_print_05() -> () {
    println!(
        "the size of String is always {:?}",
        std::mem::size_of::<String>()
    );
    println!("the size of u8 is always {:?}", std::mem::size_of::<u8>());
    println!("the size of u32 is always {:?}", std::mem::size_of::<u32>());
    println!("the size of f64 is always {:?}", std::mem::size_of::<f64>());
    println!("the size of &str can be change, like {:?} or {:?}"
,   std::mem::size_of_val("Hello world"), std::mem::size_of_val("你好，世界"));
}

fn main() {
    test_print_01();
    test_print_02();
    test_print_03();
    test_print_04();
    test_print_05();
}
