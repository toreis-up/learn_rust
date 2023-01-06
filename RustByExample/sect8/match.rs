// https://doc.rust-jp.rs/rust-by-example-ja/flow_control/match.html

fn main() {
  let number = 13;

  println!("Tell me about {}", number);

  match number {
    1 => println!("one!"),
    2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
  }

  let boolean = true;
  let binary = match boolean {
    false => 0,
    true => 1,
  };

  println!("{} -> {}", boolean, binary);
}