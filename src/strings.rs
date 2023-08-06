pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());

    hello.push('W'); // push a single character
    hello.push_str("Wo");

    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("Containes: {}", hello.contains("W"));

    println!("Replace: {}", hello.replace("Wo", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
    println!("{}", hello);
}
