use primes::{ is_prime };

fn main() {
    println!("Hello, world!");
}

fn primes_that_make_up_num(a: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    if is_a_prime(a) {
        primes.push(a);
    } else {
        if is_a_prime(a/2) {
            primes.push(2);
            primes.push(a/2);
        }
    }
    return primes;
}

fn is_a_prime(a: u64) -> bool {
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
fn test_primes_that_make_up_num_none() {
    assert_eq!(primes_that_make_up_num(0), Vec::<u64>::new());
    assert_eq!(primes_that_make_up_num(1), Vec::<u64>::new());
}

#[test]
fn test_primes_that_make_up_num_primes() {
    assert_eq!(primes_that_make_up_num(2), vec![2]);
    assert_eq!(primes_that_make_up_num(3), vec![3]);
    assert_eq!(primes_that_make_up_num(5), vec![5]);
    assert_eq!(primes_that_make_up_num(7), vec![7]);
    assert_eq!(primes_that_make_up_num(11), vec![11]);
    assert_eq!(primes_that_make_up_num(13), vec![13]);
    assert_eq!(primes_that_make_up_num(17), vec![17]);
    assert_eq!(primes_that_make_up_num(19), vec![19]);
}

#[test]
fn test_primes_that_make_up_num_multiple_of_2() {
    assert_eq!(primes_that_make_up_num(4), vec![2,2]);
    assert_eq!(primes_that_make_up_num(6), vec![2,3]);
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
