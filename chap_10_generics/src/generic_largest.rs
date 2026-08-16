use std::cmp::PartialOrd;

pub(crate) fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for t in list {
        if largest < t {
            largest = t;
        }
    }
    largest
}
