use section11_test::Adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, Adder::add_two(2));
}