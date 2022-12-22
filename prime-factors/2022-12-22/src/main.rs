fn main() {
    println!("Hello, world!");
}

fn is_prime(a: i32) -> bool {
    if a == 1 {
        return false;
    }
    return true;
}

#[test]
fn test_is_prime_under_10() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
}

#[test]
fn test_is_prime_two() {
}
