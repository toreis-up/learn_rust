// https://doc.rust-jp.rs/rust-by-example-ja/fn/hof.html

fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

fn main() {
  // 1000以下の"奇数を2乗した値"の合計を求める。
  println!("Find the sum of all the squared odd numbers under 1000");
  let upper = 1000;

  let mut acc = 0;

  for n in 0.. {
    let n_squared = n * n;

    if n_squared >= upper {
      break;
    } else if is_odd(n_squared) {
      acc += n_squared;
    }
  }

  println!("imperative style: {}", acc);

  let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n * n) // 2乗して
         .take_while(|&n_squared| n_squared < upper) // 上限より小さくて
         .filter(|&n_squared| is_odd(n_squared)) // 奇数のものを
         .fold(0, |acc, n_squared| acc + n_squared); // 足す

  println!("functional style: {}", sum_of_squared_odd_numbers);
}
