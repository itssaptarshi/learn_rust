pub fn run(){
    let mut numbers:[i32;5] = [1,2,3,4,5];

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    numbers[2] = 446;
    println!("{:?}", numbers);

    println!("{}", std::mem::size_of_val(&numbers));

}