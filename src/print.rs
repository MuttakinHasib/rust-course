pub fn run() {
    // Print to console
    println!("Hello, world!");

    // Basic formatting
    println!("{} is from {}", "Hasib", "Bangladesh");

    // Positional arguments
    println!(
        "{2} is from {1} and {0} likes to {2}",
        "Hasib", "Bangladesh", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play with {activity}",
        name = "Hasib",
        activity = "Rust"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:o} Decimal {:?}",
        10, 10, 10, 1010
    );

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}
