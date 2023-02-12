#[attribute]
これ↑あとりびゅーと

ex1.
#[test]
fn test() {
  なかみ
}

#![attribute]
↑これもあとりびゅーと
fn test2() {
  #![test]
}

## cfgあとりびゅーと
// ターゲットのOSがUnix系の場合にだけコンパイルする。
#[cfg(unix)]

// Windowsの場合にだけコンパイルする
#[cfg(windows)]

cfg!はブーリアンとして評価される
ex
if cfg!(target_os="linux") ...

## deriveあとりびゅーと
// Point構造体にDebugトレイトが自動的に実装される
struct Point {
  x: i32,
  y: i32,
  z: i32
}

fn main() {
  let some_point = Point {x: 10, y: 20, z: 0};
  println!("Debug: {:?}", some_point);
}

## crate_typeあとりびゅーと
// このクレートはライブラリである
#![crate_type = "lib"]
// このライブラリの名前は「rary」である
#![crate_name = "rary"]

// some code

crate_typeあとりびゅーとが使われている場合には、rustcにフラグを伝える必要はない！
