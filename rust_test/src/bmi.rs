use std::io;
use std::io::Write;

pub(crate) fn bmi_calculator() {

    println!("--- BMI CALCULATOR ---");

    let height = input("Enter your height (m): ");
    let weight = input("Enter your weight (kg): ");

    let height: f32 = height.parse().unwrap();
    let weight: f32 = weight.parse().unwrap();

    let bmi = weight / (height * height);

    println!("\nYour BMI is: {}", bmi.to_string());

    if bmi < 18.5 {
        println!("You are underweight!");
    }
    else if bmi > 25.0 {
        println!("You are overweight!");
    }
    else {
        println!("You weight is healthy!");
    }
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap(); 
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("failed to readline");

    return value.trim().to_string();
}
