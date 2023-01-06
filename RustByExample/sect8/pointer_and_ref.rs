// https://doc.rust-jp.rs/rust-by-example-ja/flow_control/match/destructuring/destructure_pointers.html

fn main() {
  let reference = &4;

  match reference {
    &val => println!("Got a value via destructing: {:?}", val),
  }

  match *reference {
    val => println!("Got a value via destructing: {:?}", val),
  }

  let _not_a_reference = 3;
  // これはリファレンスじゃない

  match _not_a_reference {
    val => println!("This is value: {:?}", val),
  }
  
  let ref _is_a_reference = 3;
  // これはリファレンス

  match _is_a_reference {
    &val => println!("This is value too: {:?}", val),
  }

  match *_is_a_reference {
    val => println!("This is value too: {:?}", val),
  }

  let value = 5;
  let mut mut_value = 6;

  match value {
    ref r => println!("Got a reference to a value {:?}", r),
  }

  match mut_value {
    ref mut m => {
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m);
    },
  }
}