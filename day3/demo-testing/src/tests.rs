use crate::add;
use crate::factorial;

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
    assert_eq!(factorial(7), 5040);
    assert_eq!(factorial(8), 40320);
}

#[test]
fn test_add() {
    assert_eq!(add(10, 20), 30);
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 0), 10);
    assert_eq!(add(0, 10), 10);
    assert_eq!(add(10, 10), 20);
}
