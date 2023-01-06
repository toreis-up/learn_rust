extern crate internal;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, internal::add_two(2));
}