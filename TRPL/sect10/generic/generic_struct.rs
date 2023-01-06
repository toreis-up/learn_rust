use std::fmt;

struct Point<T, U> {
  x: T,
  y: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> fmt::Display for Point<T, U> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {} and y: {}", self.x, self.y)
  }
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
  let mix = Point { x: 3, y: 2.6 };

  println!("integer: {}", integer);
  println!("float: {}", float);
  println!("mix: {}", mix);
}