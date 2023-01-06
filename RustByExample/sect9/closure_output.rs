// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/output_parameters.html

fn create_fn() -> impl Fn() {
  let text = "Fn".to_owned();

  move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
  let text = "FnMut".to_owned();

  move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
  let text = "FnOnce".to_owned();

  move || println!("This is a: {}", text)
}

fn number_print(x: i32) -> impl Fn() {
  move || println!("This function is called with {}", x)
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  let fn_once = create_fnonce();
  let fn_5 = number_print(5);

  fn_plain();
  fn_mut();
  fn_once();
  fn_5();

  fn_plain();
  fn_mut();
//   fn_once(); // 使っちゃったので発動できない
  fn_5();
}