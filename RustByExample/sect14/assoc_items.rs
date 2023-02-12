// コンテナ型に、その要素に対してジェネリックなトレイトを実装した場合、そのトレイトを使用するときには全てのジェネリック型を明記しなくてはならない。
// この例では、Containsトレイトはジェネリック型AとBの使用を許している。
// その後、Container型に対してContainsを実装しているが、その際後にdifference()が使用できるようにA Bはそれぞれi32と明記される。

// Containsはジェネリックトレイトであるため、fn difference()では全てのジェネリック型を宣言しなくてはならない。
// 実際、AとBは引数であるCによって決定されていて欲しいにも関わらず、である。
// これは関連型と呼ばれる機能によって可能になる。

struct Container(i32, i32);

// 2つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains<A, B> {
  fn contains(&self, _: &A, _: &B) -> bool; // AとBの両方を明示的に要求する
  fn first(&self) -> i32; // A Bいずれも要求しない
  fn last(&self) -> i32;  // A Bいずれも要求しない
}

impl Contains<i32, i32> for Container {
  // コンテナ内の2つの要素が等しければTrueを返す
  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  // 1つ目の値を取得
  fn first(&self) -> i32 {self.0}

  // 2つ目(最後)の値を取得
  fn last(&self) -> i32 {self.1}
}

// AとBはCに保持されていることを考慮すると、AとBを2度も書くのは面倒
fn difference<A, B, C>(container: &C) -> i32
  where C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does Container contain {} and {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));
  println!("First number: {}", container.first());
  println!("Last number: {}", container.last());

  println!("The difference is: {}", difference(&container));
}