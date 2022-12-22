use primes::{ is_prime };

fn main() {
    println!("primes of {:?} -> {:?}", 12, primes_that_make_up_num(12));
}

fn primes_that_make_up_num(a: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    match lowest_prime_factor(a) {
        None => {}
        Some(n) => {
            primes.push(n);
            primes = [primes, primes_that_make_up_num(a/n)].concat();
        }
    }
    primes.into_iter().filter(|n| *n != 0).collect::<Vec<u64>>()
}

fn lowest_prime_factor(a: u64) -> Option<u64> {
    if a < 2 {
        return None;
    } else if is_a_prime(a) {
        return Some(a);
    }
    for n in 2..a-1 {
        if a % n == 0 {
            return Some(n);
        }
    }
    None
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
    true
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
fn test_primes_that_make_up_num_multiple_of_3() {
    assert_eq!(primes_that_make_up_num(6), vec![2,3]);
    assert_eq!(primes_that_make_up_num(9), vec![3,3]);
    assert_eq!(primes_that_make_up_num(12), vec![2,2,3]);
    assert_eq!(primes_that_make_up_num(15), vec![3,5]);
    assert_eq!(primes_that_make_up_num(18), vec![2,3,3]);
    assert_eq!(primes_that_make_up_num(21), vec![3,7]);
    assert_eq!(primes_that_make_up_num(24), vec![2,2,2,3]);
}

#[test]
fn test_lowest_prime_factor_none() {
    assert_eq!(lowest_prime_factor(0), None);
    assert_eq!(lowest_prime_factor(1), None);
}

#[test]
fn test_lowest_prime_factor_primes() {
    assert_eq!(lowest_prime_factor(2), Some(2));
    assert_eq!(lowest_prime_factor(3), Some(3));
    assert_eq!(lowest_prime_factor(5), Some(5));
    assert_eq!(lowest_prime_factor(7), Some(7));
    assert_eq!(lowest_prime_factor(11), Some(11));
    assert_eq!(lowest_prime_factor(13), Some(13));
    assert_eq!(lowest_prime_factor(17), Some(17));
}

#[test]
fn test_lowest_prime_factor_multiple_of_2() {
    assert_eq!(lowest_prime_factor(4), Some(2));
    assert_eq!(lowest_prime_factor(6), Some(2));
    assert_eq!(lowest_prime_factor(8), Some(2));
    assert_eq!(lowest_prime_factor(10), Some(2));
    assert_eq!(lowest_prime_factor(12), Some(2));
    assert_eq!(lowest_prime_factor(14), Some(2));
    assert_eq!(lowest_prime_factor(16), Some(2));
    assert_eq!(lowest_prime_factor(18), Some(2));
}

#[test]
fn test_lowest_prime_factor_multiple_of_3() {
    assert_eq!(lowest_prime_factor(9), Some(3));
    assert_eq!(lowest_prime_factor(15), Some(3));
    assert_eq!(lowest_prime_factor(21), Some(3));
    assert_eq!(lowest_prime_factor(27), Some(3));
    assert_eq!(lowest_prime_factor(33), Some(3));
}

#[test]
fn test_lowest_prime_factor_multiple_of_5() {
    assert_eq!(lowest_prime_factor(25), Some(5));
    assert_eq!(lowest_prime_factor(125), Some(5));
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
