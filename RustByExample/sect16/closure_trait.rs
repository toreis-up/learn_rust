fn make_adder_funcion(y: i32) -> impl Fn(i32) -> i32 {
  let closure = move |x: i32| { x + y };
  closure
}

fn main() {
  let plus_one = make_adder_funcion(1);
  assert_eq!(plus_one(2), 3);
}