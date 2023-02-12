fn print_one<'a>(x: &'a i32) {
  println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
  *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'a i32) {
  println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// `'a`は関数より長くなくてはならないため上の関数は正しくない。
// ここでは、`&String::from("foo")`は`String`のデータとそれへの参照を作り出す。
// その後データはスコープを抜けるとともに破棄される。そのため、
// 不適切なデータに対する参照を返すことになってしまう。

fn main() {
  let x = 7;
  let y = 9;

  print_one(&x);
  print_multi(&x, &y);

  let z = pass_x(&x, &y);
  print_one(z);

  let mut t = 3;
  add_one(&mut t);

  print_one(&t);
}