pub fn run() {
    let person: (&str, &str, i8) = ("Hasib", "Dev", 24);

    println!("{} is a {} and is {}", person.0, person.1, person.2)
}
