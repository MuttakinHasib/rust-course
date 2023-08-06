pub fn run() {
    greeting("Hello", "Hasib");
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    let c: i32 = 10;
    let add_nums = |a: i32, b: i32| a + b + c;

    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
