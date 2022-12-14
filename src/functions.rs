pub fn run(){
    greeting("Hello", "Rajat");
}

fn greeting(greeting:&str, name:&str){
    println!("{} {}, Nice to meet you", greeting,name);
}