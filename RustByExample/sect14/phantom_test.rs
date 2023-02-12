use std::ops::Add;
use std::marker::PhantomData;

// 単位を定義するために、空の列挙型を作成
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// LengthはUnitという幽霊型パラメータを持つ型
// f64は最初からClone, Copyトレイトを持つ型
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// Addトレイトは加算演算子の挙動を定義する
impl<Unit> Add for Length<Unit> {
  type Output = Length<Unit>;

  // add()はLengthの新しいインスタンスを返す。
  // Lengthの中の値は合計値になっている。
  fn add(self, rhs: Length<Unit>) -> Length<Unit> {
    // ここでの+はf64のAdd実装を呼び出す
    Length(self.0 + rhs.0, PhantomData)
  }
}

fn main() {
  let one_foot: Length<Inch> = Length(12.0, PhantomData);
  let one_meter: Length<Mm> = Length(1000.0, PhantomData);

  // 以下の+は上で定義したLength<Unit>用のadd()メソッドを呼び出す。
  // LengthはCopyトレイトを持っているため、add()はone_footおよびone_meterを
  // 消費する代わりにそのコピーを作り、self, rhsとして扱う
  let two_feet = one_foot + one_foot;
  let two_meters = one_meter + one_meter;

  println!("one foot + one_foot = {:?} in", two_feet.0);
  println!("one meter + one_meter = {:?} mm", two_meters.0);
}