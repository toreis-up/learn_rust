use std::ops; // 演算子

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// std::ops::Addトレイトは+の振る舞いを規定するために使われる
// ここではFooに対してAdd<Bar>を実装する。これは加算時の右辺がBar型の時に呼び出されるトレイト。
// つまり以下はFoo + Bar = FooBarという振る舞いをもたらす
impl ops::Add<Bar> for Foo {
  type Output = FooBar;

  fn add(self, _rhs: Bar) -> FooBar {
    println!("> Foo.add(Bar) was called");

    FooBar
  }
}

// 型を反転することで、非可換の加算を実装できる。
// ここではBarに対してAdd<Foo>を実装する。
// これは加算時の右辺がFoo型の時に呼び出されるメソッド。
// つまり以下はBar + Foo = BarFooという結果をもたらす。
impl ops::Add<Foo> for Bar {
  type Output = BarFoo;

  fn add(self, _rhs: Foo) -> BarFoo {
    println!("> Bar.add(Foo) was called");

    BarFoo
  }
}

fn main() {
  println!("Foo + Bar = {:?}", Foo+Bar);
  println!("Bar + Foo = {:?}", Bar+Foo);
}