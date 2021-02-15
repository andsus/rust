#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(x: &[T], y: &[T]) -> Comparison {
    match (x.len(), y.len()) {
        _ if x == y => Comparison::Equal,
        (a, b) if a < b && _sublist(x, y) => Comparison::Sublist,
        (a, b) if a > b && _sublist(y, x) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

//https://doc.rust-lang.org/std/primitive.slice.html#method.windows
pub fn _sublist<T: PartialEq>(less: &[T], more: &[T]) -> bool {
    less.is_empty() || more.windows(less.len()).any(|m_win| m_win == less)
}
