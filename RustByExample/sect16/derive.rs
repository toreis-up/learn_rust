// コンパイラには[#derive]アトリビュートを用いることで、型に対して特定のトレイトの標準的な実装を提供する機能がある。
// より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能。

// 例えばこんなトレイトがある
// 型の比較に関連するトレイト: Eq, PartialEq, Ord, PartialOrd
// コピー用のトレイト: Copy A = Bが実装できる   所有権のムーブがおきる
// クローン用のトレイト: Clone A = B.Clone()  所有権のムーブがおきない
// &Tからハッシュ値を計算するためのトレイト: Hash
// 空っぽのインスタンスを作成するためのトレイト: Default
// {:?}フォーマッタを使って値をフォーマットするためのトレイト: Debug

//　Centimetersは比較可能なタプルになる
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// Inchesはプリント可能なタプルになる
#[derive(Debug)]
struct Inches(i32);

impl Inches {
  fn to_centimeters(&self) -> Centimeters {
    let &Inches(inches) = self;

    Centimeters(inches as f64 * 2.54)
  }
}

// Secondsには特にアトリビュートを付け加えない
struct Seconds(i32);

fn main() {
  let _one_second = Seconds(1);
  
  // SecondはDebugトレイトくっつけてないので表示できない
  // のでこれはエラーを起こす
  // println!("One second looks like: {:?}", _one_second);

  // でもってPartialEqトレイトを実装していないのでSecondは比較できない
  // のでこれもエラーを起こす
  // let _this_is_true = (_one_second == _one_second);

  let foot = Inches(12);
  println!("one foot equals {:?}", foot);

  let meter = Centimeters(100.0);

  let cmp = 
    if foot.to_centimeters() < meter {
      "smaller"
    } else {
      "bigger"
    };

  println!("One foot is {} than one meter.", cmp);
}