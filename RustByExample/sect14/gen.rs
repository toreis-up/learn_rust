// Aという具象型
struct A;

// Singleという型を定義する際にAを使用しているが、その最初の使用よりも
// 先に<A>がないため、またA自身も具象型であるから、Singleは具象型になる。
struct Single(A);

// ここでは<T>が一番初めのTの使用よりも先にきている。よってSingleGenはジェネリック型となる。
// なぜならが、型パラメータTがジェネリックだからであるため。Tはどんな型にもなり得るので、
// 上で定義したAを受け取ることも、そうでない型を受け取ることもできる。
struct SingleGen<T>(T);

fn main() {
  // Singleは具象型で、Aのみ受け取る
  let _s = Single(A);

  // SingleGen<char>という型を与えている。
  let _char: SingleGen<char> = SingleGen('a');

  // Aを使用
  let _t = SingleGen(A);

  // i32を使用
  let _i32 = SingleGen(3);
}