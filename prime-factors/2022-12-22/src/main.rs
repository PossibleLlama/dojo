fn main() {
    println!("Hello, world!");
}

fn is_prime(_a: i32, _b: i32) -> bool {
    return false;
}

#[test]
fn test_single_num() {
    assert_eq!(is_prime(1, 1), false);
}
