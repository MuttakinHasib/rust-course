pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    println!("Vector Length: {}", numbers.len());
    println!(
        "Vector occupies {} bytes: ",
        std::mem::size_of_val(&numbers)
    );
    numbers.push(5);
    numbers.pop();

    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);

    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);
}
