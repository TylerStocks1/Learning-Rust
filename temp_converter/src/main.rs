use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter the temperature to convert from Celcius to Fahrenheit");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    println!("{input}");

    let temp: f32 = input.trim().parse().expect("Please enter a number");
    let fahrenheit = temp * 1.8 + 32.0;
    println!(" the temp is: {temp} Celcius & in Fahrenheit this is {fahrenheit}");
}

//C -> F is F = (C x 1.8) + 32
