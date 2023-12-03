use either::{self, Either};
use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::Index,
    path::Iter,
    vec,
};

pub fn reverse_itr<'a, Container: DoubleEndedIterator<Item = T>, T>(
    into_itr: Container,
    reverse: bool,
) -> Either<
    std::iter::Rev<<Container as IntoIterator>::IntoIter>,
    <Container as IntoIterator>::IntoIter,
> {
    let mut itr = into_itr.into_iter();
    if reverse {
        Either::Left(itr.rev())
    } else {
        Either::Right(itr)
    }
}

pub fn get_input() -> Vec<String> {
    let mut lines_in = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines_in.push(match line {
            Ok(ln) => ln,
            Err(err) => {
                println!("Error: {}", err);
                "".to_string()
            }
        });
    }
    lines_in
}
