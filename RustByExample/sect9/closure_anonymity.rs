// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/anonymity.html

fn apply<F>(f: F) where
  F: Fn() {
    f();
  }

fn main() {
  let x = 7;

  let print = || println!("{}", x);

  apply(print);
}