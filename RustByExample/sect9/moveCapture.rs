fn main() {
  let haystack = vec![1, 2, 3];
  
  // ここでhaystackの所有権がcontainsに強制的に移動した？
  let contains = move |needle| heystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));
}
