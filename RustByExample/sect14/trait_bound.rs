// プリント時のマーカー{:?}を実装するトレイト
use std::fmt::Debug;

trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Rectangle {
  fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// ジェネリック型TはDebugトレイトを実装していなくてはならない。
// その限りにおいて、Tがどのような具象型であっても次の関数は動作する。
fn print_debug<T> (t: &T) where T: Debug {
  println!("{:?}", t);
}

// 「TはHasAreaトレイトを実装していなくてはならない」という境界条件を満たしていれば、
// HasAreaの関数areaにアクセスできる。
fn area<T>(t: &T) -> f64 where T: HasArea { t.area() }

fn main() {
  let rectangle = Rectangle { length: 3.0, height: 4.0 };
  let _triangle = Triangle  { length: 3.0, height: 4.0 };

  print_debug(&rectangle);
  println!("Area: {}", rectangle.area());
}