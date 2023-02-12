struct Container(i32, i32);

// 2つの要素がコンテナ型の中に保持されていることを確認するトレイト。
// また、最初あるいは最後の要素を取り出すこともできる。
trait Contains {
  // メソッドが使用できるジェネリック型の定義
  // これは型エイリアスを使うときのtypeとは違う
  type A;
  type B;

  fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
  fn first(&self) -> i32;
  fn last(&self) -> i32;
}

impl Contains for Container {
  // AとBがどの型であるかを明示。インプットの型(つまりSelfの型)が
  // Container(i32, i32)である場合、出力型はi32, i32となる。
  type A = i32;
  type B = i32;

  // &i32の代わりに &Self::Aまたは&Self::Bと書いても良い
  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  fn first(&self) -> i32 { self.0 }
  fn last(&self) -> i32 {self.1}
}

fn difference<C: Contains>(container: &C) -> i32 {
  container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does container contain {} and {} : {}",
    &number_1, &number_2, container.contains(&number_1, &number_2));
  
}