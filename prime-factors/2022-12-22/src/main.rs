use primes::{ is_prime };

fn main() {
    println!("Hello, world!");
}

fn is_a_prime(a: u64) -> bool {
    if a <= 1 {
        return false;
    }
    let x = a-1;
    for n in 2..x {
        if a % n == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_prime_under_10() {
    assert_eq!(is_a_prime(0), false);
    assert_eq!(is_a_prime(1), false);
    assert_eq!(is_a_prime(2), true);
    assert_eq!(is_a_prime(3), true);
    assert_eq!(is_a_prime(4), false);
    assert_eq!(is_a_prime(5), true);
    assert_eq!(is_a_prime(6), false);
    assert_eq!(is_a_prime(7), true);
    assert_eq!(is_a_prime(8), false);
    assert_eq!(is_a_prime(9), false);
}

#[test]
fn test_is_prime_under_1000() {
    for n in 0..1000 {
        assert_eq!(is_a_prime(n), is_prime(n));
    }
}
