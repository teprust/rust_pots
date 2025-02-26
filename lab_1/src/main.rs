

// Функция для реализации задания №1
fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {

    println!("Hello, world!");

    // Задание №1 "Числа Фибоначчи"
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

