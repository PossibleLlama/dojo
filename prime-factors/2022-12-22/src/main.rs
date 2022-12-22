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
fn test_is_prime_single_num() {
    assert_eq!(is_prime(1), false);
}

#[test]
fn test_is_prime_two() {
    assert_eq!(is_prime(2), true);
}
