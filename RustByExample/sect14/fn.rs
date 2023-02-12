struct A;

struct S(A);

struct SGen<T>(T);

// ジェネリック関数じゃない
fn reg_fn(_s: S) {}

// Aという型を与えられたSGenという型を返すので、ジェネリックでない
fn gen_spec_t(_s: SGen<A>) {}

// i32型を与えられたSGenなので、これもジェネリックでない
fn gen_spec_i32(_s: SGen<i32>) {}

// genericという関数を定義し、SGen<T>という型の引数_sをとる。
// <T>がSGenに先行しているため、これはTに対してジェネリックな関数
fn generic<T>(_s: SGen<T>) {}

fn main() {
  reg_fn(S(A));           // 具象
  gen_spec_t(SGen(A));    // 型パラメータAを暗黙のうちに受け取っている
  gen_spec_i32(SGen(3));  // 型パラメータi32を暗黙のうちに受け取っている

  generic::<char>(SGen('a')) // generic()にcharを明示している

  generic(SGen('c'));

  generic(SGen(A));          // generic()にSGen<A>を暗黙のうちに渡している
}