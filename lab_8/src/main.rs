use std::cmp::Ordering;

// TODO: Сделайте функцию min которая вызывается в main.
fn min<T: Ord>(v1: T, v2: T) -> T {
    if v1 < v2 {
        v1
    }
    else {
        v2
    }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}