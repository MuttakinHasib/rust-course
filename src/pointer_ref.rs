pub fn run() {
    let arr = [1, 2, 3];
    let nums = arr;

    println!("{:?}", (arr, nums));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));
}
