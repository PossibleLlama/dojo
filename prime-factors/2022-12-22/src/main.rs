fn main() {
    println!("Hello, world!");
}

fn is_prime(a: i32) -> bool {
    if a <= 1 {
        return false;
    }
    for n in 2..a-1 {
        if a % n == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_prime_under_10() {
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
}

#[test]
fn test_is_prime_negative() {
    assert_eq!(is_prime(-1), false);
    assert_eq!(is_prime(-3), false);
    assert_eq!(is_prime(-5), false);
    assert_eq!(is_prime(-7), false);
    assert_eq!(is_prime(-9), false);
}
