// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_functions.html

// 関数を引数として取り、即座に実行する関数
fn call_me<F: Fn()>(f: F) {
  f();
}

// Fn境界を満たすラッパ関数を定義
// 要はFnトレイトを満たしたラッパ(:関数を関数で包み込んで、中に入ってる関数を隠す)関数...ってコト？！
fn function() {
  println!("I'm a function!");
}

fn main() {
  let closure = || println!("I'm a closure!");

  call_me(closure);
  call_me(function);
}