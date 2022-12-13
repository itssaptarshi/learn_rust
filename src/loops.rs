pub fn run(){
    let mut count = 1;


    // FizzBuzz challange
    while count <=100{
        if count % 15 ==0{
            println!("FizzBuzz");
        }
        else if count %  5 == 0 {
            println!("Buzz");
        }else if count% 3==0 {
            println!("Fizz");
        }else {
            println!("{}", count);
        }
    count +=1;
    }


    //FizzBuzz using For loop
    for x in 0..100{
        if x % 15 ==0{
            println!("FizzBuzz");
        }
        else if x %  5 == 0 {
            println!("Buzz");
        }else if x% 3==0 {
            println!("Fizz");
        }else {
            println!("{}", x);
        }
    }
}