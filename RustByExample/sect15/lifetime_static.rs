use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
  println!("'static value passed in is: {:?}", input);
}

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
  &NUM
}

fn main() {
  {
    let static_string = "I'm in rom";
    println!("static_string: {}", static_string);
  }
  {
    let lifetime_num = 9;
    
    let coerced_static = coerce_static(&lifetime_num);
    
    println!("coerced_static: {}", coerced_static);
  }

  println!("NUM: {} stays accessible!!", NUM);
  // ----- //
  let i = 5;
  print_it(i);

  print_it(&i);
}

