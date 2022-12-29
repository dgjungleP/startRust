use addr;

mod common;

#[test]
fn it_add_two() {
    common::set_up();
    assert_eq!(4, addr::add(2, 2));
}
