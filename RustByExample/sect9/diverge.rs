fn foo() -> ! {
  // この呼び出しはリターンしない
  panic!("This call never returns.");
}

fn some_fn() {
  ()
}

fn main() {
  let a: () = some_fn();
  println!("This function returns and you can see this line.");

  // これをアンコメントするとpanicして終わる
  // foo();
}