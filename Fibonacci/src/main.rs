use std::io;

fn main() {
    println!("Enter the number you want to find on the Fibonacci scale");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let fibonacci_num: i128 = input.trim().parse().expect("Enter a number");
    let nth: i128 = fib(fibonacci_num);

    println!("Fibonacci number of {input} is {nth}");
}

fn fib(n: i128) -> i128 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib (n-1) + fib(n-2);
    }
}
// F = f(i - 1) + f(i - 2)
//Improvement ideas, Cache previous awnsers to make calcuations faster.
