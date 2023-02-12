// コピー不可な型
// clone()メソッドを用いないかぎり、値のコピーではなくムーブが起きる型
struct Empty;
struct Null;

// ジェネリック型<T>に対するトレイト
trait DoubleDrop<T> {
  // selfに加えてもう一つのジェネリック方を受け取り、
  // 何もしないメソッドのシグネチャを定義
  fn double_drop(self, _: T);
}

// Uをselfとして、Tをもう一つの引数として受け取るDoubleDrop<T>を実装する。
// U Tはいずれもジェネリック型
// trait DoubleDrop<T>をU(全ての型?)に対して実装
impl<T, U> DoubleDrop<T> for U {
  // このメソッドは2つの引数の所有権を取り、メモリ上から解放する。
  fn double_drop(self, _: T) {}
}

fn main() {
  let empty = Empty;
  let null  = Null;

  // emptyとnullを解放
  empty.double_drop(null);

  empty;
  null;
}