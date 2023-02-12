fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
  let _x = 12;

  // &_xのライフタイムはyよりも短いため、関数内で'aを使用して変数のライフタイムを指定しようとすると失敗する
  // つまり、短いライフタイムを持つ参照をより長いものに強制的に代入することはできない。
  let y: &'a i32 = &_x;
  // ↑エラーになる
}

fn main() {
  let (four, nine) = (4, 9);

  print_refs(&four, &nine);
  // 借用された変数の寿命は、借り手のそれよりも長くなくてはならない。
  // つまり、four、nineのライフタイムはprint_refsのそれよりも
  // 長くなくてはならない。

  failed_borrow();
  // failed_borrowは関数のライフタイムよりも'aを長くさせるような参照を持たないが、それでも'aの方が長くなる。
  // なぜならそのような場合、'aはデフォルトで'staticなるからである。

}