use std::iter::Iterator;
pub type NextFn<T, S> = dyn Fn(T) -> S;
pub type ParserFn<T, S> = dyn Fn(T, NextFn<T, S>) -> S;

pub fn pipe<T, S, I: Iterator<Item = NextFn<T, S>>>(functions: I) -> ParserFn<T, S> {
    |arg: T, next: NextFn| {
        let piped = functions.iter().fold(next, |accum, current| {
            curry_parse_function(accum, current)
        });

        return piped(arg);
    }
}

fn curry_parse_function<T, S>(next: NextFn<T, S>, parse: ParserFn<T, S>) -> NextFn<T, S> {
    |arg: T| parse(arg, next)
}