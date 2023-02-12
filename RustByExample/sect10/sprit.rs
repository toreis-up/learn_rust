// このように宣言すると、'my.rs'を探し、
// その内容をこのファイルの中で'my'という名前で使用することができるようにする
mod my;

fn function() {
  println!("called `function()`");
}

fn main() {
  my::function();

  function();
  
  my::indirect_access();

  my::nested::function();
}