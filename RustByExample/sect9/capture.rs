// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/capture.html

fn main() {
  use std::mem;

  let color = String::from("green");

  let print = || println!("`color`: {}", color);

  print();

  // イミュータブルな借用なので、値が変わらない、おっけー
  let _reborrow = &color;

  print();

  // 使い切ったので、移動しようが再借用しようが構わない
  let _color_moved = color;

  let mut count = 0;

  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  }

  inc();

  // let _reborrow = &count;
  // これはバグる
  // クロージャがcountをミュータブルで借用しているので、
  // なぜなら後で呼ばれるからです↓

  inc();

  let _count_reborrowed = &mut count;
  // クロージャはもうcountを呼び出さないので、ここで借用するのはおっけー

  let movable = Box::new(3);

  let consume = || {
    println!("`movable`: {:?}", movable);
    mem::drop(movable);
  };

  consume();

  // 一回ドロップしちゃったので、一度しか呼び出せない
  // consume()
  // ↑これバグる
}