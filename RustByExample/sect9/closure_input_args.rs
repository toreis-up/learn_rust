// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_parameters.html

// whereって何よ
// ジェネリックを読みやすくするために使う
// トレイト境界(: ジェネリック型に対して、このトレイトを実装していなければならないと誓約を課すもの)を設置する
fn apply<F> (f: F) where
  F: FnOnce() {
    f();
  }

fn apply_to_3<F>(f: F) -> i32 where
  F: Fn(i32) -> i32 {
    f(3)
  }

fn main() {
  use std::mem;

  let greeting = "hello";

  let mut farewell = "goodbye".to_owned();

  let diary = || { // クロージャ
    println!("I said {}.", greeting);

    // farewellの値を変えるので、この時点でFnMutが必要になる <- わかる
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzzz");

    mem::drop(farewell);
  };

  apply(diary);

  let double = |x| 2 * x;
  println!("3 doubled: {}", apply_to_3(double));
}