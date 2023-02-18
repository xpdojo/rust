use lang;

// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
#[test]
fn test_add() {
    assert_eq!(lang::add(1, 2), 3);
}

#[test]
#[should_panic]
fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(lang::bad_add(1, 2), 3);
}
