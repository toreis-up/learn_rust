use std::io;

fn main() {
    let mut number = String::new();

    println!("What number do you want to know of fibonacci?");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Failed to convert in number");

    let fibo_answer = fibo(number);

    println!("fibonacci of {} is {}", number, fibo_answer);
}

fn fibo(num: i32) -> i32 {
    if num == 0 {
        0
    } else if num == 1 || num == 2 {
        1
    }
    else {
        fibo(num - 1) + fibo(num - 2)
    }
}