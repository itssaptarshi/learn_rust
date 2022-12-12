pub fn run(){
    let mut hello = String::from("Hello ") ;

    println!("{}", hello);
    println!("Length : {}", hello.len());

    hello.push('w');
    println!("{}", hello);

    hello.push_str("hhaaaaaattttt");
    println!("{}", hello);


    println!("{}", hello.capacity());
    println!("{}", hello.is_empty());
    println!("{}", hello.contains("Hello"));
    println!("{}", hello.replace("Hello", "Hola"));

    

    println!("{}", hello);
}