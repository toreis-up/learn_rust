// Iteratorトレイトは、例えば配列のような、要素の集合に対してイテレータを実装するためのトレイト
// このトレイトはnextの要素に相当するものを決定するためのメソッドのみを要する。
// このメソッドはimplブロックのなかで手動で実装するか、あるいは配列やrangeのように自動で定義される。

struct Fibonacci {
  curr: u32,
  next: u32,
}

// IteratorをFibonacciに対して実装する
// Iteratorトレイトは次('next')の要素を取得するメソッドの定義だけを要求する
impl Iterator for Fibonacci {
  // We can refer to this type using Self::Item
  type Item = u32;

  // ここではイテレーションの流れを.currと.nextを使用して定義している。
  // 返り値の型はOption<T>で、これは
  //    * Iteratorが終了した時にはNoneを返し
  //    * そうでなければSomeでラップされた値を返す
  fn next(&mut self) -> Option<Self::Item> {
    let current = self.curr;
    
    self.curr = self.next;
    self.next = current + self.next;

    // フィボナッチ数列には終端がないので、Iteratorは決してNoneを返さず、常にSomeが返される
    Some(current)
  }
}

fn fibonacci() -> Fibonacci {
  Fibonacci { curr:0, next: 1}
}

fn main() {
  // 0..3は0, 1, 2をジェネレートするIterator
  let mut sequence = 0..3;

  println!("Four consecutive next calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  // forはNoneを返すまで、イテレータを舐めていき、出てきたSomeを
  // アンラップして変数(ここではi)に束縛する
  println!("Iterate through 0..3 using `for`");
  for i in 0..3 {
    println!("> {}", i);
  }

  // take(n)メソッドはIteratorをはじめからn番目の要素までの部分に減らす
  println!("The first four terms of the Fibonacci sequence are: ");
  for i in fibonacci().take(4) {
    println!("> {}", i);
  }

  // skip(n)メソッドはIteratorのはじめからn番目までの要素を飛ばす
  println!("The next four terms of the Fibonacci sequence are: ");
  for i in fibonacci().skip(4).take(4) {
    println!("> {}", i);
  }

  let array = [1u32, 3, 3, 7];

  // iterメソッドは配列ヤスライスからイテレータを作成する
  println!("Iterate the following array {:?}", &array);
  for i in array.iter() {
    println!("> {}", i);
  }
}