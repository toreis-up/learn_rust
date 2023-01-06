// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/closure_examples/iter_find.html

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  let mut iter = vec1.iter(); // &i32が渡されている
  let mut into_iter = vec2.into_iter(); // i32が渡されている

  println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2)); // 参照は&&i32になっている
  println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2)); // 参照は&32になっている

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
  println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2)); // 配列にinto_iterを使うとやっぱり&i32が出るんだね
}