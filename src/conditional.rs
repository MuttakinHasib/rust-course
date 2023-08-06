pub fn run() {
    let age = 24;

    if age >= 21 {
        println!("You can drink!");
    } else {
        println!("You can't drink!");
    }

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
