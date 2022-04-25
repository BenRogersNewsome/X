
pub trait Stream<T> {

    fn next(&mut self) -> T;
    fn peek(&self) -> T;
    fn is_end(self) -> bool;
}