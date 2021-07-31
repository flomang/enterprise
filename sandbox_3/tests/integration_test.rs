use sandbox_3;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, sandbox_3::add_two(2));
}