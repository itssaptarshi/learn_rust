pub fn run(){
    let mut numbers:Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    numbers[2] = 446;
    numbers.push(4);
    numbers.push(5);
    numbers.push(6);
    numbers.pop();


    println!("{:?}", numbers);

    println!("{}", std::mem::size_of_val(&numbers));

}