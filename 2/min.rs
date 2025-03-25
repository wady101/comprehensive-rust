use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.
fn min<T: Ord>(l :T, r : T) -> T {
    if l > r { r } else { l }
    /*
    Could use this, but I did not, cauz I am too cool
    match l.cmp(&r) {
    Ordering::Less | Ordering::Equal => l,
    Ordering::Greater => r,
    }
    */
}

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
