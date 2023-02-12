struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を儲けているが、そのトレイトがからであるか否かは関係ない。
fn red<T>(_: &T) -> where T: Red &'static str {"red"}
fn blue<T>(_: &T) -> where T: Blue &'static str {"blue"}

fn main() {
  let cardinal = Cardinal;

  let blue_jay = BlueJay;

  let _turkey = Turkey;

  println!("A cardinal is {}", red(&cardinal));
  println!("A blue jay is {}", blue(&blue_jay));
}