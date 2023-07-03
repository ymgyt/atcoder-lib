use super::gcd::gcd;

#[test]
fn gcd_test() {
    assert_eq!(gcd(100, 10), 10);
    assert_eq!(gcd(7, 3), 1);
}
