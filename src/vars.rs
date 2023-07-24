pub fn run() {
    let name = "Hasib";
    let age = 24;
    // age = 25; // This will throw an error

    // Define constant
    const ID: i32 = 001;

    // Assign multiple vars
    let (my_name, my_age) = ("Hasib", 24);

    println!("My name is {}", name);
    println!("I am {} years old", age);
    println!("ID: {}", ID);
    println!("{} is {}", my_name, my_age);
}
