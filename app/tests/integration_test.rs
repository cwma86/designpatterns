use app::adder::adder::add;

#[test]
fn test_add() {
    assert_eq!(add(3, 2), 5);
}
