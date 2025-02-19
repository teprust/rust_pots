/// Определяет длину последовательности Коллатца для числа n.
fn collatz_length(mut n: i32) -> u32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if n == 1 {
            return counter;
        }
        else if n % 2 != 0 {
            n = 3*n+1;
        }
        else{
            n = n/2;
        }
    }

}
// Через cargo test в данной директории в консоли
#[test]
fn test_collatz_len() {
    assert_eq!(collatz_length(7), 17);
}

fn main() {
    println!("Длина: {}", collatz_length(7));
}
