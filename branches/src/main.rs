fn main() {
    fn2();
}
    
fn fn2() {
    let condition = true;
    let number: i32 = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn fn1() {
    let number = 3;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}