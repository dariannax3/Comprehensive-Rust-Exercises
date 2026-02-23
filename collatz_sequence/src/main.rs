fn tests() {
    assert_eq!(collatz_length(11), 15);
    assert_eq!(collatz_length(6), 9);
    assert_eq!(collatz_length(3), 8);
    assert_eq!(collatz_length(1), 1);
}

fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

fn main() {
    println!("Length: {}", collatz_length(11));
    tests();
}
