use std::marker::PhantomData;

// ジェネリックなタプル構造、2つ目のパラメータは幽霊型
// 比較演算子での比較を可能にする
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 同様に構造体を定義
#[derive(PartialEq)]
struct PhantomStruct<A, B> {first: A, phantom: PhantomData<B>}

// Note: ジェネリック型Aに対してはメモリが割り当てられているが、
// Bには割り当てられてないので、計算には使えない。

fn main() {
  let _tuple1: PhantomTuple<char, f32> = PhantomData('Q', PhantomData);

  let _tuple2: PhantomTuple<char, f64> = PhantomData('Q', PhantomData);

  let _struct1: PhantomStruct<char, f32> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };

  let _struct2: PhantomStruct<char, f64> = PhantomStruct {
    first: 'Q',
    phantom; PhantomData,
  }
}