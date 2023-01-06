use std::io;

fn main() {
    let mut mode = String::new();

    println!("F° to C°(fc) or C° to F°(cf)");

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    let mode = mode.trim();

    if mode == "cf" {
        cto_f();
    } else if mode == "fc" {
        fto_c();
    } else {
        println!("mode is not defined");
    }
}

fn cto_f() {
    let mut celcius = String::new();

    println!("How is the Celcius temperature?");

    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to read line");

    let celcius: f32 = celcius.trim().parse().expect("Please type a number");

    let fahrenheit = celcius * 1.8 + 32.0;

    println!("{}℃  is  {}℉", celcius, fahrenheit);
}

fn fto_c() {
    let mut fahrenheit = String::new();

    println!("How is the Fahlenheit temperature?");

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    
    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number");

    let celcius = (fahrenheit - 32.0) / 1.8;

    println!("{}℉  is  {}℃", fahrenheit, celcius);
}