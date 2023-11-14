use std::io;
use std::io::Write;

pub(crate) fn even_or_odd() {

    println!("--- Even Or Odd ---");

    let number_string: String = input("Enter a number: ");
    let number: i32 = number_string.parse::<i32>().unwrap();

    if number % 2 == 0 {
        println!("The number is even");
    }
    else {
        println!("The number is odd");
    }
}

fn input(msg: &str) -> String  {
    print!("{}", msg);
    io::stdout().flush().unwrap(); 
    let mut value: String = String::new();
    io::stdin().read_line(&mut value).expect("failed to readline");

    return value.trim().to_string();
}
