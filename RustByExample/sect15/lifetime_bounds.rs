use std::fmt::Debug;
// ライフタイムを紐づけるトレイト

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Refはaというライフタイムを持つジェネリック型Tに対する参照を持ち、
// Tの値に対する参照は必ず'aよりも長生きでなくてはならない。
// さらに、Refのライフタイムは'aをこえてはならない。

// Debugトレイトを利用してプリントを行うジェネリック関数
fn print<T>(t: T) where
T: Debug {
  println!("`print`: t is {:?}", t);
}

// Debugを実装しているTへの参照を取る、
// Tへの*参照+は必ず'aよりも長生きでなくてはならない。
// さらに、'aは関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
  println!("`print_ref`: t is {:?}", t);
}

fn main() {
  let x = 7;
  let ref_x = Ref(&x);

  print_ref(&ref_x);
  print(ref_x);
}