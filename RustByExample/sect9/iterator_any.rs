fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2)); // iterは参照を渡す
  println!("2 in vec1: {}", vec1.into_iter().any(|x| x == 2)); // into_iterは値を渡す
  println!("2 in vec2: {}", vec2.iter().any(|&x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  println!("2 in array1: {}", array1.iter().any(|&x| x == 2)); // 配列に対しても同じ
  println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2)); // 配列に対しては"例外的に"&i32が帰ってくる
}